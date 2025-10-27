# WarGames/JOSHUA: Claude Integration Specifications
## Comprehensive Guide to Anthropic Claude API Integration
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Overview](#1-overview)
2. [Claude API Configuration](#2-claude-api-configuration)
3. [Prompt Engineering](#3-prompt-engineering)
4. [Response Parsing](#4-response-parsing)
5. [Error Handling and Retries](#5-error-handling-and-retries)
6. [Context Management](#6-context-management)
7. [Cost Optimization](#7-cost-optimization)
8. [Testing and Validation](#8-testing-and-validation)
9. [Production Patterns](#9-production-patterns)
10. [Monitoring and Observability](#10-monitoring-and-observability)

---

## 1. Overview

### 1.1 Purpose

The Claude Analysis Engine is the **core intelligence component** of the WarGames/JOSHUA system. It uses Anthropic's Claude API to perform sophisticated nuclear risk analysis with consistent, reproducible results. This document specifies the complete integration pattern.

### 1.2 Key Requirements

**Consistency:** Multiple analyses of the same data must produce similar results (variance < 5%)  
**Reliability:** 99.9% uptime with automatic failover and retry logic  
**Speed:** Complete analysis in < 3 minutes for comprehensive risk assessment  
**Accuracy:** >90% agreement with expert human analysts on risk level classification  
**Cost Efficiency:** Optimize token usage while maintaining analysis quality  

### 1.3 Integration Architecture

```
Data Collection → Context Building → Prompt Construction → Claude API
                                                              ↓
Historical Storage ← Result Validation ← Response Parsing ← Analysis
```

---

## 2. Claude API Configuration

### 2.1 Model Selection

**Primary Model:** `claude-sonnet-4-20250514`  
**Fallback Model:** `claude-3-5-sonnet-20241022`  
**Justification:** Sonnet-4 provides optimal balance of:
- Speed (2-3 minute analysis times)
- Cost ($3 per million input tokens, $15 per million output tokens)
- Reasoning capability (critical for complex geopolitical analysis)
- Consistency (lower temperature variance)

**DO NOT USE:**
- Claude Opus: Too slow and expensive for frequent assessments
- Claude Haiku: Insufficient reasoning depth for nuclear risk analysis

### 2.2 Core Configuration

```rust
/// Claude API client configuration for WarGames/JOSHUA
pub struct ClaudeConfig {
    /// API endpoint
    pub base_url: String,  // "https://api.anthropic.com"
    
    /// API credentials
    pub api_key: SecureString,  // Encrypted at rest
    
    /// Model selection
    pub model: String,  // "claude-sonnet-4-20250514"
    
    /// Generation parameters
    pub max_tokens: u32,  // 8000 (allows comprehensive analysis)
    pub temperature: f32,  // 0.1 (low for consistency)
    pub top_p: f32,        // 0.95 (standard)
    pub top_k: i32,        // -1 (disabled, use top_p)
    
    /// Request timeouts
    pub request_timeout: Duration,   // 180 seconds
    pub connect_timeout: Duration,   // 30 seconds
    
    /// Retry configuration
    pub max_retries: u32,            // 3
    pub retry_delay: Duration,       // 5 seconds
    pub exponential_backoff: bool,   // true
    pub max_retry_delay: Duration,   // 60 seconds
    
    /// Rate limiting
    pub requests_per_minute: u32,    // 50 (API limit)
    pub tokens_per_minute: u32,      // 40_000 (API limit)
    
    /// Advanced features
    pub enable_streaming: bool,      // false (not needed)
    pub enable_caching: bool,        // true (prompt caching)
    pub cache_ttl: Duration,         // 5 minutes
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.anthropic.com".to_string(),
            api_key: SecureString::from_env("CLAUDE_API_KEY"),
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 8000,
            temperature: 0.1,  // CRITICAL: Low for consistency
            top_p: 0.95,
            top_k: -1,
            request_timeout: Duration::from_secs(180),
            connect_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
            exponential_backoff: true,
            max_retry_delay: Duration::from_secs(60),
            requests_per_minute: 50,
            tokens_per_minute: 40_000,
            enable_streaming: false,
            enable_caching: true,
            cache_ttl: Duration::from_secs(300),
        }
    }
}
```

### 2.3 API Client Implementation

```rust
/// Claude API client with automatic retry and rate limiting
pub struct ClaudeClient {
    config: ClaudeConfig,
    http_client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
    token_counter: Arc<AtomicU32>,
    metrics: Arc<ClaudeMetrics>,
}

impl ClaudeClient {
    /// Create new Claude API client
    pub fn new(config: ClaudeConfig) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(config.request_timeout)
            .connect_timeout(config.connect_timeout)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .pool_max_idle_per_host(10)
            .build()?;
        
        let rate_limiter = Arc::new(RateLimiter::new(
            config.requests_per_minute,
            config.tokens_per_minute,
        ));
        
        Ok(Self {
            config,
            http_client,
            rate_limiter,
            token_counter: Arc::new(AtomicU32::new(0)),
            metrics: Arc::new(ClaudeMetrics::default()),
        })
    }
    
    /// Send message to Claude API with automatic retry
    pub async fn messages_create(
        &self,
        request: MessageRequest,
    ) -> Result<MessageResponse> {
        // Wait for rate limit
        self.rate_limiter.acquire(request.estimated_tokens()).await?;
        
        // Execute with retry logic
        let mut last_error = None;
        for attempt in 0..self.config.max_retries {
            match self.execute_request(&request).await {
                Ok(response) => {
                    self.metrics.record_success(&response);
                    return Ok(response);
                }
                Err(e) if e.is_retryable() => {
                    last_error = Some(e);
                    let delay = self.calculate_retry_delay(attempt);
                    tracing::warn!(
                        "Claude API request failed (attempt {}/{}): {}. Retrying in {:?}",
                        attempt + 1,
                        self.config.max_retries,
                        last_error.as_ref().unwrap(),
                        delay
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    self.metrics.record_error(&e);
                    return Err(e);
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| anyhow!("Max retries exceeded")))
    }
    
    /// Execute single API request
    async fn execute_request(
        &self,
        request: &MessageRequest,
    ) -> Result<MessageResponse> {
        let start = Instant::now();
        
        let response = self.http_client
            .post(format!("{}/v1/messages", self.config.base_url))
            .header("x-api-key", self.config.api_key.expose_secret())
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(request)
            .send()
            .await?;
        
        let status = response.status();
        let latency = start.elapsed();
        
        if !status.is_success() {
            let error_body = response.text().await?;
            return Err(Self::parse_error(status, &error_body));
        }
        
        let mut api_response: MessageResponse = response.json().await?;
        api_response.latency = Some(latency);
        
        // Track token usage
        self.token_counter.fetch_add(
            api_response.usage.input_tokens + api_response.usage.output_tokens,
            Ordering::Relaxed,
        );
        
        Ok(api_response)
    }
    
    /// Calculate exponential backoff delay
    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        if !self.config.exponential_backoff {
            return self.config.retry_delay;
        }
        
        let delay = self.config.retry_delay * 2_u32.pow(attempt);
        std::cmp::min(delay, self.config.max_retry_delay)
    }
}
```

### 2.4 Request/Response Types

```rust
/// Claude API message request
#[derive(Debug, Clone, Serialize)]
pub struct MessageRequest {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    
    pub messages: Vec<Message>,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,  // "user" or "assistant"
    pub content: String,
}

/// Claude API response
#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponse {
    pub id: String,
    pub type_: String,  // "message"
    pub role: String,   // "assistant"
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
    
    #[serde(skip)]
    pub latency: Option<Duration>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlock {
    pub type_: String,  // "text"
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}
```

---

## 3. Prompt Engineering

### 3.1 System Prompt

The system prompt establishes Claude's role and analysis framework. This is **CRITICAL** for consistent results.

```rust
pub const SYSTEM_PROMPT: &str = r#"You are the JOSHUA Strategic Analysis System, an expert nuclear risk assessment AI developed to monitor and analyze global nuclear war probability.

# Your Role
You provide objective, data-driven assessments of nuclear war risk based on the Doomsday Clock framework used by the Bulletin of Atomic Scientists. Your analysis must be:
- Evidence-based and citing specific sources
- Quantitative where possible
- Balanced and avoiding sensationalism
- Consistent with established risk assessment methodologies
- Transparent about uncertainty and confidence levels

# Assessment Framework
You evaluate risk across these dimensions:
1. Nuclear Arsenal Changes (weight: 0.20) - Modernization, expansion, new capabilities
2. Arms Control Breakdown (weight: 0.20) - Treaty violations, negotiation failures
3. Regional Conflicts (weight: 0.20) - Active conflicts involving nuclear powers
4. Leadership & Rhetoric (weight: 0.10) - Nuclear threats, destabilizing statements
5. Technical Incidents (weight: 0.15) - False alarms, accidents, cyber attacks
6. Communication Breakdown (weight: 0.10) - Diplomatic failures, hotline issues
7. Emerging Technology (weight: 0.10) - AI weapons, hypersonics, cyber capabilities
8. Economic Factors (weight: 0.05) - Sanctions, trade wars, resource competition

# Output Format
You MUST respond with valid JSON in this EXACT structure:
{
  "analysis_id": "<unique_id>",
  "timestamp": "<iso8601_timestamp>",
  "seconds_to_midnight": <integer 0-1440>,
  "risk_level": "<critical|severe|elevated|moderate|low>",
  "confidence": <float 0.0-1.0>,
  "risk_factors": {
    "nuclear_arsenal_changes": <float 0.0-1.0>,
    "arms_control_breakdown": <float 0.0-1.0>,
    "regional_conflicts": <float 0.0-1.0>,
    "leadership_rhetoric": <float 0.0-1.0>,
    "technical_incidents": <float 0.0-1.0>,
    "communication_breakdown": <float 0.0-1.0>,
    "emerging_technology": <float 0.0-1.0>,
    "economic_factors": <float 0.0-1.0>
  },
  "critical_developments": [
    {
      "event": "<description>",
      "source": "<source_name>",
      "impact": "<critical|high|medium|low>",
      "affected_regions": [<list>],
      "escalation_potential": <float 0.0-1.0>,
      "confidence": <float 0.0-1.0>
    }
  ],
  "trend_analysis": {
    "direction": "<deteriorating|stable|improving>",
    "velocity": <float -10.0 to +10.0>,
    "acceleration": <float -5.0 to +5.0>,
    "primary_drivers": [<top 3 factors>]
  },
  "early_warning_indicators": [
    {
      "indicator": "<description>",
      "severity": "<critical|high|medium|low>",
      "confidence": <float 0.0-1.0>,
      "recommended_monitoring": "<specific action>"
    }
  ],
  "executive_summary": "<2-3 paragraph summary for decision makers>",
  "detailed_analysis": "<comprehensive analysis with specific evidence and citations>",
  "uncertainty_factors": [
    {
      "factor": "<description>",
      "impact_on_assessment": "<high|medium|low>"
    }
  ],
  "recommended_actions": [
    {
      "action": "<description>",
      "priority": "<immediate|high|medium|low>",
      "rationale": "<explanation>"
    }
  ]
}

# Critical Instructions
- ALWAYS output valid JSON - no markdown code blocks, no explanatory text outside JSON
- Seconds to midnight: 0 = immediate nuclear war, 1440 = no risk (24 hours to midnight)
- Risk levels: critical (<120s), severe (120-300s), elevated (300-600s), moderate (600-900s), low (>900s)
- Confidence: Based on data quality, source reliability, agreement between sources
- Citations: Reference specific sources in your detailed analysis
- Consistency: Similar data should produce similar risk scores (±5 seconds)

# Today's Date
{current_date}
"#;
```

### 3.2 User Prompt Construction

```rust
/// Build comprehensive risk assessment prompt
pub fn build_risk_assessment_prompt(
    data: &AggregatedData,
    historical_context: &HistoricalContext,
) -> Result<String> {
    let mut prompt = String::new();
    
    // 1. Context and Baseline
    prompt.push_str("# Risk Assessment Request\n\n");
    prompt.push_str(&format!(
        "Current Date: {}\n",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    prompt.push_str(&format!(
        "Assessment Type: Comprehensive Nuclear Risk Analysis\n\n"
    ));
    
    // 2. Previous Assessment Summary
    if let Some(prev) = &historical_context.previous_assessment {
        prompt.push_str("## Previous Assessment Baseline\n\n");
        prompt.push_str(&format!(
            "- Date: {}\n",
            prev.timestamp.format("%Y-%m-%d")
        ));
        prompt.push_str(&format!(
            "- Seconds to Midnight: {}\n",
            prev.seconds_to_midnight
        ));
        prompt.push_str(&format!(
            "- Risk Level: {}\n",
            prev.risk_level
        ));
        prompt.push_str(&format!(
            "- Key Drivers: {}\n\n",
            prev.primary_drivers.join(", ")
        ));
    }
    
    // 3. Recent Trend
    if !historical_context.recent_assessments.is_empty() {
        prompt.push_str("## Recent Risk Trend (Last 30 Days)\n\n");
        prompt.push_str("```\n");
        prompt.push_str("Date       | Seconds | Risk Level | Change\n");
        prompt.push_str("-----------|---------|------------|-------\n");
        
        for (i, assessment) in historical_context.recent_assessments.iter().enumerate() {
            let change = if i > 0 {
                let prev_seconds = historical_context.recent_assessments[i - 1].seconds_to_midnight;
                let delta = assessment.seconds_to_midnight - prev_seconds;
                format!("{:+}", delta)
            } else {
                "—".to_string()
            };
            
            prompt.push_str(&format!(
                "{} | {:7} | {:10} | {}\n",
                assessment.timestamp.format("%Y-%m-%d"),
                assessment.seconds_to_midnight,
                assessment.risk_level,
                change
            ));
        }
        prompt.push_str("```\n\n");
    }
    
    // 4. New Data Collection Summary
    prompt.push_str("## Newly Collected Data\n\n");
    prompt.push_str(&format!(
        "Collection Period: {} to {}\n",
        data.collection_start.format("%Y-%m-%d %H:%M UTC"),
        data.collection_end.format("%Y-%m-%d %H:%M UTC")
    ));
    prompt.push_str(&format!(
        "Total Data Points: {}\n",
        data.total_data_points
    ));
    prompt.push_str(&format!(
        "Sources: {} active, {} failed\n",
        data.sources_active,
        data.sources_failed
    ));
    prompt.push_str(&format!(
        "Overall Quality Score: {:.2}\n\n",
        data.quality_score
    ));
    
    // 5. Critical Developments by Category
    prompt.push_str("## Critical Developments by Risk Category\n\n");
    
    for category in &[
        "nuclear_arsenal_changes",
        "arms_control_breakdown",
        "regional_conflicts",
        "leadership_rhetoric",
        "technical_incidents",
        "communication_breakdown",
        "emerging_technology",
        "economic_factors",
    ] {
        if let Some(items) = data.by_category.get(*category) {
            if !items.is_empty() {
                prompt.push_str(&format!("### {}\n\n", category_display_name(category)));
                
                for item in items.iter().take(10) {  // Top 10 per category
                    prompt.push_str(&format!(
                        "**[{} - {}] {}**\n",
                        item.source,
                        item.published_at.format("%Y-%m-%d"),
                        item.title
                    ));
                    prompt.push_str(&format!("Relevance: {:.2}\n", item.relevance_score));
                    if let Some(sentiment) = &item.sentiment {
                        prompt.push_str(&format!(
                            "Sentiment: {} ({:.2})\n",
                            sentiment.label,
                            sentiment.score
                        ));
                    }
                    prompt.push_str(&format!("{}\n\n", item.summary));
                }
            }
        }
    }
    
    // 6. Data Quality Assessment
    prompt.push_str("## Data Quality Assessment\n\n");
    prompt.push_str(&format!(
        "- News Sources: {} collected, {:.1}% success rate\n",
        data.news_count,
        data.news_success_rate * 100.0
    ));
    prompt.push_str(&format!(
        "- Research Sources: {} collected, {:.1}% success rate\n",
        data.research_count,
        data.research_success_rate * 100.0
    ));
    prompt.push_str(&format!(
        "- Government Sources: {} collected, {:.1}% success rate\n",
        data.gov_count,
        data.gov_success_rate * 100.0
    ));
    prompt.push_str(&format!(
        "- Social Media: {} data points, {:.1}% relevance\n\n",
        data.social_count,
        data.social_relevance * 100.0
    ));
    
    // 7. Specific Assessment Instructions
    prompt.push_str("## Assessment Instructions\n\n");
    prompt.push_str("Based on the data provided:\n\n");
    prompt.push_str("1. Analyze each risk factor category thoroughly\n");
    prompt.push_str("2. Compare current situation to historical baseline\n");
    prompt.push_str("3. Identify trends and acceleration/deceleration\n");
    prompt.push_str("4. Assess confidence based on data quality and agreement\n");
    prompt.push_str("5. Provide specific, actionable early warning indicators\n");
    prompt.push_str("6. Generate executive summary suitable for decision makers\n\n");
    
    prompt.push_str("**CRITICAL: Respond ONLY with valid JSON. No markdown, no code blocks, no explanatory text.**\n");
    
    Ok(prompt)
}
```

### 3.3 Prompt Optimization Techniques

**Token Efficiency:**
```rust
/// Optimize prompt for token usage while maintaining quality
pub fn optimize_prompt(prompt: &str, max_tokens: usize) -> String {
    // 1. Remove redundant whitespace
    let optimized = prompt
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    // 2. Truncate data points if needed
    let token_count = estimate_tokens(&optimized);
    if token_count > max_tokens {
        // Implement smart truncation:
        // - Keep system prompt (required)
        // - Keep historical context (important)
        // - Truncate data points (least to most relevant)
        truncate_intelligently(&optimized, max_tokens)
    } else {
        optimized
    }
}

/// Estimate token count (rough approximation: 1 token ≈ 4 characters)
fn estimate_tokens(text: &str) -> usize {
    text.len() / 4
}
```

**Prompt Caching:**
```rust
/// Use Claude's prompt caching for repeated system prompts
pub struct CachedPromptBuilder {
    system_prompt: String,
    cache_key: String,
    cache_ttl: Duration,
}

impl CachedPromptBuilder {
    pub fn build_request(&self, user_prompt: &str) -> MessageRequest {
        MessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 8000,
            temperature: 0.1,
            system: Some(self.system_prompt.clone()),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                }
            ],
            // Enable prompt caching for system prompt
            ..Default::default()
        }
    }
}
```

---

## 4. Response Parsing

### 4.1 JSON Response Parser

```rust
/// Parse Claude's JSON response with comprehensive validation
pub struct ResponseParser {
    schema_validator: JsonSchemaValidator,
    confidence_threshold: f64,
}

impl ResponseParser {
    pub fn parse_risk_analysis(
        &self,
        response: &MessageResponse,
    ) -> Result<RiskAnalysis> {
        // Extract text content
        let text = self.extract_text_content(response)?;
        
        // Clean response (remove markdown code blocks if present)
        let cleaned = self.clean_json_response(&text)?;
        
        // Parse JSON
        let raw_json: serde_json::Value = serde_json::from_str(&cleaned)
            .context("Failed to parse JSON response from Claude")?;
        
        // Validate against schema
        self.schema_validator.validate(&raw_json)?;
        
        // Deserialize to structured type
        let analysis: RiskAnalysis = serde_json::from_value(raw_json)
            .context("Failed to deserialize risk analysis")?;
        
        // Validate business rules
        self.validate_analysis(&analysis)?;
        
        Ok(analysis)
    }
    
    /// Extract text content from response
    fn extract_text_content(&self, response: &MessageResponse) -> Result<String> {
        let mut content = String::new();
        
        for block in &response.content {
            if block.type_ == "text" {
                content.push_str(&block.text);
                content.push('\n');
            }
        }
        
        if content.is_empty() {
            return Err(anyhow!("No text content in Claude response"));
        }
        
        Ok(content.trim().to_string())
    }
    
    /// Clean JSON response (remove markdown code blocks, etc.)
    fn clean_json_response(&self, text: &str) -> Result<String> {
        let text = text.trim();
        
        // Remove markdown code blocks if present
        let cleaned = if text.starts_with("```json") || text.starts_with("```") {
            text.lines()
                .skip(1)  // Skip opening ```
                .take_while(|line| !line.trim().starts_with("```"))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            text.to_string()
        };
        
        // Find JSON boundaries
        let start = cleaned.find('{')
            .ok_or_else(|| anyhow!("No JSON object found in response"))?;
        let end = cleaned.rfind('}')
            .ok_or_else(|| anyhow!("No closing brace found in response"))?;
        
        Ok(cleaned[start..=end].to_string())
    }
    
    /// Validate analysis against business rules
    fn validate_analysis(&self, analysis: &RiskAnalysis) -> Result<()> {
        // Validate seconds_to_midnight range
        if analysis.seconds_to_midnight > 1440 {
            return Err(anyhow!(
                "Invalid seconds_to_midnight: {} (max: 1440)",
                analysis.seconds_to_midnight
            ));
        }
        
        // Validate risk_level matches seconds_to_midnight
        let expected_level = RiskLevel::from_seconds(analysis.seconds_to_midnight);
        if analysis.risk_level != expected_level {
            tracing::warn!(
                "Risk level mismatch: {} seconds should be {:?}, got {:?}",
                analysis.seconds_to_midnight,
                expected_level,
                analysis.risk_level
            );
        }
        
        // Validate confidence range
        if analysis.confidence < 0.0 || analysis.confidence > 1.0 {
            return Err(anyhow!(
                "Invalid confidence: {} (must be 0.0-1.0)",
                analysis.confidence
            ));
        }
        
        // Validate risk factor ranges
        for (factor, value) in &analysis.risk_factors {
            if *value < 0.0 || *value > 1.0 {
                return Err(anyhow!(
                    "Invalid risk factor {}: {} (must be 0.0-1.0)",
                    factor,
                    value
                ));
            }
        }
        
        // Validate required fields
        if analysis.executive_summary.is_empty() {
            return Err(anyhow!("Missing executive summary"));
        }
        
        if analysis.detailed_analysis.is_empty() {
            return Err(anyhow!("Missing detailed analysis"));
        }
        
        // Validate confidence threshold
        if analysis.confidence < self.confidence_threshold {
            tracing::warn!(
                "Low confidence analysis: {:.2} (threshold: {:.2})",
                analysis.confidence,
                self.confidence_threshold
            );
        }
        
        Ok(())
    }
}
```

### 4.2 Error Recovery

```rust
/// Handle parsing errors with intelligent recovery
impl ResponseParser {
    pub fn parse_with_recovery(
        &self,
        response: &MessageResponse,
    ) -> Result<RiskAnalysis> {
        match self.parse_risk_analysis(response) {
            Ok(analysis) => Ok(analysis),
            Err(e) => {
                tracing::error!("Failed to parse Claude response: {}", e);
                
                // Try recovery strategies
                if let Some(recovered) = self.attempt_recovery(response) {
                    tracing::info!("Successfully recovered analysis");
                    Ok(recovered)
                } else {
                    Err(e)
                }
            }
        }
    }
    
    fn attempt_recovery(&self, response: &MessageResponse) -> Option<RiskAnalysis> {
        // Strategy 1: Try to extract partial JSON
        if let Some(partial) = self.extract_partial_json(response) {
            if self.validate_analysis(&partial).is_ok() {
                return Some(partial);
            }
        }
        
        // Strategy 2: Try to extract key fields with regex
        if let Some(extracted) = self.extract_key_fields(response) {
            if self.validate_analysis(&extracted).is_ok() {
                return Some(extracted);
            }
        }
        
        // Strategy 3: Request re-analysis
        None
    }
    
    fn extract_partial_json(&self, response: &MessageResponse) -> Option<RiskAnalysis> {
        // Implementation: Try to parse as much JSON as possible
        // Fill in missing fields with defaults
        None  // Placeholder
    }
    
    fn extract_key_fields(&self, response: &MessageResponse) -> Option<RiskAnalysis> {
        // Implementation: Use regex to extract critical fields
        // Build minimal valid RiskAnalysis
        None  // Placeholder
    }
}
```

---

## 5. Error Handling and Retries

### 5.1 Error Classification

```rust
/// Claude API error types
#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Overloaded: {0}")]
    Overloaded(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parsing error: {0}")]
    Parsing(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ClaudeError {
    /// Determine if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ClaudeError::RateLimit(_) |
            ClaudeError::Overloaded(_) |
            ClaudeError::Timeout(_) |
            ClaudeError::Network(_)
        )
    }
    
    /// Get recommended retry delay
    pub fn retry_delay(&self) -> Duration {
        match self {
            ClaudeError::RateLimit(_) => Duration::from_secs(60),
            ClaudeError::Overloaded(_) => Duration::from_secs(30),
            ClaudeError::Timeout(_) => Duration::from_secs(10),
            ClaudeError::Network(_) => Duration::from_secs(5),
            _ => Duration::from_secs(0),
        }
    }
    
    /// Parse HTTP error response
    pub fn from_http_error(status: StatusCode, body: &str) -> Self {
        match status.as_u16() {
            401 => ClaudeError::Authentication(body.to_string()),
            429 => ClaudeError::RateLimit(body.to_string()),
            400 => ClaudeError::InvalidRequest(body.to_string()),
            529 => ClaudeError::Overloaded(body.to_string()),
            _ => ClaudeError::Unknown(format!("HTTP {}: {}", status, body)),
        }
    }
}
```

### 5.2 Retry Strategy

```rust
/// Intelligent retry logic with exponential backoff
pub struct RetryStrategy {
    max_retries: u32,
    base_delay: Duration,
    max_delay: Duration,
    exponential: bool,
    jitter: bool,
}

impl RetryStrategy {
    pub async fn execute_with_retry<F, Fut, T>(
        &self,
        mut operation: F,
    ) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, ClaudeError>>,
    {
        let mut last_error = None;
        
        for attempt in 0..self.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && attempt < self.max_retries - 1 => {
                    let delay = self.calculate_delay(attempt, &e);
                    
                    tracing::warn!(
                        "Operation failed (attempt {}/{}): {}. Retrying in {:?}",
                        attempt + 1,
                        self.max_retries,
                        e,
                        delay
                    );
                    
                    tokio::time::sleep(delay).await;
                    last_error = Some(e);
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }
        
        Err(anyhow!(last_error.unwrap_or_else(|| 
            ClaudeError::Unknown("Max retries exceeded".to_string())
        )))
    }
    
    fn calculate_delay(&self, attempt: u32, error: &ClaudeError) -> Duration {
        // Start with error-specific delay
        let mut delay = error.retry_delay();
        
        // Apply exponential backoff if enabled
        if self.exponential {
            delay = delay.max(self.base_delay * 2_u32.pow(attempt));
        } else {
            delay = delay.max(self.base_delay);
        }
        
        // Cap at max delay
        delay = delay.min(self.max_delay);
        
        // Add jitter if enabled (±20%)
        if self.jitter {
            let jitter_range = delay.as_millis() as f64 * 0.2;
            let jitter = (rand::random::<f64>() - 0.5) * 2.0 * jitter_range;
            delay = Duration::from_millis((delay.as_millis() as f64 + jitter) as u64);
        }
        
        delay
    }
}
```

### 5.3 Circuit Breaker

```rust
/// Circuit breaker pattern for Claude API
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    state: Arc<Mutex<CircuitState>>,
}

#[derive(Debug)]
enum CircuitState {
    Closed {
        failures: u32,
    },
    Open {
        opened_at: Instant,
    },
    HalfOpen {
        successes: u32,
    },
}

impl CircuitBreaker {
    pub async fn execute<F, Fut, T>(
        &self,
        operation: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        // Check circuit state
        {
            let mut state = self.state.lock().await;
            match *state {
                CircuitState::Open { opened_at } => {
                    if opened_at.elapsed() > self.timeout {
                        *state = CircuitState::HalfOpen { successes: 0 };
                        tracing::info("Circuit breaker: Transitioning to half-open");
                    } else {
                        return Err(anyhow!("Circuit breaker is open"));
                    }
                }
                _ => {}
            }
        }
        
        // Execute operation
        match operation().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }
    
    async fn on_success(&self) {
        let mut state = self.state.lock().await;
        *state = match *state {
            CircuitState::HalfOpen { successes } => {
                if successes + 1 >= self.success_threshold {
                    tracing::info!("Circuit breaker: Closed");
                    CircuitState::Closed { failures: 0 }
                } else {
                    CircuitState::HalfOpen { successes: successes + 1 }
                }
            }
            CircuitState::Closed { .. } => {
                CircuitState::Closed { failures: 0 }
            }
            _ => *state,
        };
    }
    
    async fn on_failure(&self) {
        let mut state = self.state.lock().await;
        *state = match *state {
            CircuitState::Closed { failures } => {
                if failures + 1 >= self.failure_threshold {
                    tracing::warn!("Circuit breaker: Opened");
                    CircuitState::Open { opened_at: Instant::now() }
                } else {
                    CircuitState::Closed { failures: failures + 1 }
                }
            }
            CircuitState::HalfOpen { .. } => {
                tracing::warn!("Circuit breaker: Re-opened");
                CircuitState::Open { opened_at: Instant::now() }
            }
            _ => *state,
        };
    }
}
```

---

## 6. Context Management

### 6.1 Historical Context Builder

```rust
/// Build historical context for Claude analysis
pub struct ContextBuilder {
    db: DatabasePool,
    max_history_items: usize,
    lookback_days: u32,
}

impl ContextBuilder {
    pub async fn build_context(&self) -> Result<HistoricalContext> {
        // Get previous assessment
        let previous = self.get_previous_assessment().await?;
        
        // Get recent trend
        let recent_assessments = self.get_recent_assessments(30).await?;
        
        // Get major events since last assessment
        let major_events = if let Some(prev) = &previous {
            self.get_major_events_since(prev.timestamp).await?
        } else {
            Vec::new()
        };
        
        // Calculate trend statistics
        let trend_stats = self.calculate_trend_stats(&recent_assessments);
        
        Ok(HistoricalContext {
            previous_assessment: previous,
            recent_assessments,
            major_events,
            trend_stats,
        })
    }
    
    async fn get_previous_assessment(&self) -> Result<Option<AssessmentSummary>> {
        sqlx::query_as::<_, AssessmentSummary>(
            r#"
            SELECT 
                id,
                timestamp,
                seconds_to_midnight,
                risk_level,
                confidence,
                primary_drivers
            FROM assessments
            WHERE status = 'completed'
            ORDER BY timestamp DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.db)
        .await
        .context("Failed to fetch previous assessment")
    }
    
    async fn get_recent_assessments(&self, days: u32) -> Result<Vec<AssessmentSummary>> {
        let cutoff = Utc::now() - Duration::days(days as i64);
        
        sqlx::query_as::<_, AssessmentSummary>(
            r#"
            SELECT 
                id,
                timestamp,
                seconds_to_midnight,
                risk_level,
                confidence,
                primary_drivers
            FROM assessments
            WHERE status = 'completed'
              AND timestamp >= $1
            ORDER BY timestamp DESC
            LIMIT $2
            "#
        )
        .bind(cutoff)
        .bind(self.max_history_items as i64)
        .fetch_all(&self.db)
        .await
        .context("Failed to fetch recent assessments")
    }
    
    fn calculate_trend_stats(&self, assessments: &[AssessmentSummary]) -> TrendStats {
        if assessments.is_empty() {
            return TrendStats::default();
        }
        
        let scores: Vec<i32> = assessments
            .iter()
            .map(|a| a.seconds_to_midnight)
            .collect();
        
        let mean = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
        let variance = scores
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        let std_dev = variance.sqrt();
        
        // Calculate trend direction (linear regression)
        let (slope, _intercept) = Self::linear_regression(&scores);
        
        TrendStats {
            mean,
            std_dev,
            min: *scores.iter().min().unwrap(),
            max: *scores.iter().max().unwrap(),
            trend_direction: if slope < -0.5 {
                "deteriorating"
            } else if slope > 0.5 {
                "improving"
            } else {
                "stable"
            }.to_string(),
            trend_strength: slope.abs(),
        }
    }
    
    fn linear_regression(values: &[i32]) -> (f64, f64) {
        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = values.iter().sum::<i32>() as f64 / n;
        
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        
        for (i, &y) in values.iter().enumerate() {
            let x = i as f64;
            numerator += (x - x_mean) * (y as f64 - y_mean);
            denominator += (x - x_mean).powi(2);
        }
        
        let slope = if denominator != 0.0 {
            numerator / denominator
        } else {
            0.0
        };
        
        let intercept = y_mean - slope * x_mean;
        
        (slope, intercept)
    }
}
```

---

## 7. Cost Optimization

### 7.1 Token Management

```rust
/// Track and optimize token usage
pub struct TokenManager {
    daily_limit: u32,
    monthly_limit: u32,
    current_usage: Arc<AtomicU32>,
    db: DatabasePool,
}

impl TokenManager {
    pub async fn check_budget(&self, estimated_tokens: u32) -> Result<bool> {
        let current = self.current_usage.load(Ordering::Relaxed);
        
        // Check daily limit
        let daily_usage = self.get_daily_usage().await?;
        if daily_usage + estimated_tokens > self.daily_limit {
            tracing::warn!(
                "Daily token budget exceeded: {} + {} > {}",
                daily_usage,
                estimated_tokens,
                self.daily_limit
            );
            return Ok(false);
        }
        
        // Check monthly limit
        let monthly_usage = self.get_monthly_usage().await?;
        if monthly_usage + estimated_tokens > self.monthly_limit {
            tracing::warn!(
                "Monthly token budget exceeded: {} + {} > {}",
                monthly_usage,
                estimated_tokens,
                self.monthly_limit
            );
            return Ok(false);
        }
        
        Ok(true)
    }
    
    pub async fn record_usage(&self, usage: Usage) -> Result<()> {
        let total = usage.input_tokens + usage.output_tokens;
        self.current_usage.fetch_add(total, Ordering::Relaxed);
        
        // Store in database for historical tracking
        sqlx::query(
            r#"
            INSERT INTO token_usage 
                (timestamp, input_tokens, output_tokens, model)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(Utc::now())
        .bind(usage.input_tokens as i32)
        .bind(usage.output_tokens as i32)
        .bind("claude-sonnet-4-20250514")
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_cost_estimate(&self, usage: Usage) -> f64 {
        // Claude Sonnet 4 pricing
        const INPUT_COST_PER_MTK: f64 = 3.0;   // $3 per million tokens
        const OUTPUT_COST_PER_MTK: f64 = 15.0;  // $15 per million tokens
        
        let input_cost = (usage.input_tokens as f64 / 1_000_000.0) * INPUT_COST_PER_MTK;
        let output_cost = (usage.output_tokens as f64 / 1_000_000.0) * OUTPUT_COST_PER_MTK;
        
        input_cost + output_cost
    }
}
```

### 7.2 Prompt Caching Strategy

```rust
/// Leverage Claude's prompt caching to reduce costs
pub struct PromptCachingStrategy {
    cache: Arc<DashMap<String, CachedPrompt>>,
    ttl: Duration,
}

impl PromptCachingStrategy {
    pub fn should_cache_system_prompt(&self) -> bool {
        // Always cache system prompt (it doesn't change frequently)
        true
    }
    
    pub fn should_cache_historical_context(&self, context: &HistoricalContext) -> bool {
        // Cache historical context if it's > 1000 tokens
        context.estimated_tokens() > 1000
    }
    
    pub fn build_cached_request(
        &self,
        system_prompt: &str,
        historical_context: Option<&str>,
        user_prompt: &str,
    ) -> MessageRequest {
        // Combine cacheable parts
        let mut messages = Vec::new();
        
        // System prompt (always cacheable)
        messages.push(Message {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        });
        
        // Historical context (cacheable if large enough)
        if let Some(context) = historical_context {
            messages.push(Message {
                role: "user".to_string(),
                content: format!("# Historical Context\n\n{}", context),
            });
        }
        
        // Current data (not cacheable, changes every time)
        messages.push(Message {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        });
        
        MessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            messages,
            ..Default::default()
        }
    }
}
```

---

## 8. Testing and Validation

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_parse_valid_response() {
        let parser = ResponseParser::new();
        
        let response = MessageResponse {
            id: "test_001".to_string(),
            type_: "message".to_string(),
            role: "assistant".to_string(),
            content: vec![ContentBlock {
                type_: "text".to_string(),
                text: r#"{
                    "analysis_id": "test_analysis_001",
                    "timestamp": "2025-10-27T12:00:00Z",
                    "seconds_to_midnight": 90,
                    "risk_level": "critical",
                    "confidence": 0.87,
                    "risk_factors": {
                        "nuclear_arsenal_changes": 0.78,
                        "arms_control_breakdown": 0.85,
                        "regional_conflicts": 0.89,
                        "leadership_rhetoric": 0.56,
                        "technical_incidents": 0.45,
                        "communication_breakdown": 0.67,
                        "emerging_technology": 0.71,
                        "economic_factors": 0.42
                    },
                    "critical_developments": [],
                    "trend_analysis": {
                        "direction": "deteriorating",
                        "velocity": -2.5,
                        "acceleration": -0.5,
                        "primary_drivers": ["regional_conflicts"]
                    },
                    "early_warning_indicators": [],
                    "executive_summary": "Risk is critical...",
                    "detailed_analysis": "Detailed analysis...",
                    "uncertainty_factors": [],
                    "recommended_actions": []
                }"#.to_string(),
            }],
            model: "claude-sonnet-4-20250514".to_string(),
            stop_reason: Some("end_turn".to_string()),
            stop_sequence: None,
            usage: Usage {
                input_tokens: 5000,
                output_tokens: 1500,
            },
            latency: Some(Duration::from_secs(3)),
        };
        
        let result = parser.parse_risk_analysis(&response);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.seconds_to_midnight, 90);
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }
    
    #[tokio::test]
    async fn test_retry_on_rate_limit() {
        // Test retry logic with rate limit error
        // Implementation details...
    }
    
    #[tokio::test]
    async fn test_circuit_breaker() {
        // Test circuit breaker pattern
        // Implementation details...
    }
}
```

### 8.2 Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore]  // Requires API key
    async fn test_full_analysis_workflow() {
        let config = ClaudeConfig::default();
        let client = ClaudeClient::new(config).unwrap();
        let parser = ResponseParser::new();
        
        // Build test prompt
        let prompt = build_test_prompt();
        
        // Send request
        let request = MessageRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 8000,
            temperature: 0.1,
            system: Some(SYSTEM_PROMPT.to_string()),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            ..Default::default()
        };
        
        let response = client.messages_create(request).await.unwrap();
        
        // Parse response
        let analysis = parser.parse_risk_analysis(&response).unwrap();
        
        // Validate results
        assert!(analysis.seconds_to_midnight <= 1440);
        assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0);
        assert!(!analysis.executive_summary.is_empty());
    }
}
```

### 8.3 Consistency Tests

```rust
/// Test Claude analysis consistency
#[cfg(test)]
mod consistency_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore]
    async fn test_analysis_consistency() {
        // Run same analysis multiple times and check variance
        let config = ClaudeConfig::default();
        let client = ClaudeClient::new(config).unwrap();
        let parser = ResponseParser::new();
        
        let prompt = build_test_prompt();
        let num_runs = 5;
        let mut results = Vec::new();
        
        for _ in 0..num_runs {
            let request = build_request(&prompt);
            let response = client.messages_create(request).await.unwrap();
            let analysis = parser.parse_risk_analysis(&response).unwrap();
            results.push(analysis);
        }
        
        // Calculate variance
        let scores: Vec<i32> = results
            .iter()
            .map(|r| r.seconds_to_midnight)
            .collect();
        
        let mean = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
        let variance = scores
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        let std_dev = variance.sqrt();
        
        // Variance should be < 5% of mean
        let acceptable_variance = mean * 0.05;
        assert!(
            std_dev < acceptable_variance,
            "Consistency test failed: std_dev={:.2}, acceptable={:.2}",
            std_dev,
            acceptable_variance
        );
    }
}
```

---

## 9. Production Patterns

### 9.1 Multiple Analysis Ensemble

```rust
/// Run multiple analyses and build consensus
pub struct EnsembleAnalyzer {
    client: ClaudeClient,
    parser: ResponseParser,
    num_analyses: usize,
    consensus_threshold: f64,
}

impl EnsembleAnalyzer {
    pub async fn analyze_with_consensus(
        &self,
        data: &AggregatedData,
        context: &HistoricalContext,
    ) -> Result<ConsensusAnalysis> {
        // Run multiple analyses in parallel
        let analyses = self.run_multiple_analyses(data, context).await?;
        
        // Build consensus
        let consensus = self.build_consensus(&analyses)?;
        
        // Detect and handle disagreements
        self.analyze_disagreements(&analyses, &consensus)?;
        
        Ok(consensus)
    }
    
    async fn run_multiple_analyses(
        &self,
        data: &AggregatedData,
        context: &HistoricalContext,
    ) -> Result<Vec<RiskAnalysis>> {
        let mut handles = Vec::new();
        
        for i in 0..self.num_analyses {
            let client = self.client.clone();
            let parser = self.parser.clone();
            let data = data.clone();
            let context = context.clone();
            
            let handle = tokio::spawn(async move {
                let prompt = build_risk_assessment_prompt(&data, &context)?;
                let request = build_request(&prompt);
                let response = client.messages_create(request).await?;
                parser.parse_risk_analysis(&response)
            });
            
            handles.push(handle);
        }
        
        // Wait for all analyses
        let results = futures::future::try_join_all(handles).await?;
        let analyses: Vec<RiskAnalysis> = results
            .into_iter()
            .collect::<Result<Vec<_>>>()?;
        
        Ok(analyses)
    }
    
    fn build_consensus(&self, analyses: &[RiskAnalysis]) -> Result<ConsensusAnalysis> {
        // Calculate median seconds_to_midnight
        let mut scores: Vec<i32> = analyses
            .iter()
            .map(|a| a.seconds_to_midnight)
            .collect();
        scores.sort_unstable();
        let median_seconds = scores[scores.len() / 2];
        
        // Calculate mean and std dev
        let mean = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
        let variance = scores
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        let std_dev = variance.sqrt();
        
        // Aggregate risk factors (mean)
        let mut aggregated_factors = HashMap::new();
        for factor in [
            "nuclear_arsenal_changes",
            "arms_control_breakdown",
            "regional_conflicts",
            "leadership_rhetoric",
            "technical_incidents",
            "communication_breakdown",
            "emerging_technology",
            "economic_factors",
        ] {
            let values: Vec<f64> = analyses
                .iter()
                .filter_map(|a| a.risk_factors.get(factor).copied())
                .collect();
            
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            aggregated_factors.insert(factor.to_string(), mean);
        }
        
        // Calculate consensus confidence
        let confidence = self.calculate_consensus_confidence(analyses, std_dev);
        
        // Merge critical developments
        let critical_developments = self.merge_critical_developments(analyses);
        
        // Build consensus summary
        let executive_summary = self.build_consensus_summary(analyses);
        
        Ok(ConsensusAnalysis {
            consensus_seconds: median_seconds,
            mean_seconds: mean as i32,
            std_dev,
            confidence,
            risk_level: RiskLevel::from_seconds(median_seconds),
            risk_factors: aggregated_factors,
            critical_developments,
            executive_summary,
            individual_analyses: analyses.to_vec(),
            agreement_level: self.calculate_agreement_level(analyses),
        })
    }
    
    fn calculate_consensus_confidence(&self, analyses: &[RiskAnalysis], std_dev: f64) -> f64 {
        // High agreement = high confidence
        // Low std_dev = high confidence
        
        let mean_confidence = analyses.iter().map(|a| a.confidence).sum::<f64>() 
            / analyses.len() as f64;
        
        // Penalize high variance
        let variance_penalty = (std_dev / 100.0).min(0.3);  // Max 30% penalty
        
        (mean_confidence - variance_penalty).max(0.0).min(1.0)
    }
    
    fn calculate_agreement_level(&self, analyses: &[RiskAnalysis]) -> f64 {
        // Calculate how much analyses agree
        let scores: Vec<i32> = analyses
            .iter()
            .map(|a| a.seconds_to_midnight)
            .collect();
        
        let mean = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
        let variance = scores
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
        
        // High variance = low agreement
        1.0 - (variance.sqrt() / mean).min(1.0)
    }
}
```

### 9.2 Streaming for Long Analyses

```rust
/// Stream Claude response for progress tracking
pub struct StreamingAnalyzer {
    client: ClaudeClient,
}

impl StreamingAnalyzer {
    pub async fn analyze_with_progress<F>(
        &self,
        prompt: String,
        mut progress_callback: F,
    ) -> Result<RiskAnalysis>
    where
        F: FnMut(StreamProgress),
    {
        // Note: Claude API doesn't support true streaming for JSON responses
        // This is a simulation using chunked polling
        
        progress_callback(StreamProgress {
            stage: "data_collection".to_string(),
            percent: 0.0,
        });
        
        // Start analysis
        let request = build_request(&prompt);
        
        progress_callback(StreamProgress {
            stage: "claude_analysis".to_string(),
            percent: 30.0,
        });
        
        let response = self.client.messages_create(request).await?;
        
        progress_callback(StreamProgress {
            stage: "parsing".to_string(),
            percent: 90.0,
        });
        
        let parser = ResponseParser::new();
        let analysis = parser.parse_risk_analysis(&response)?;
        
        progress_callback(StreamProgress {
            stage: "complete".to_string(),
            percent: 100.0,
        });
        
        Ok(analysis)
    }
}
```

---

## 10. Monitoring and Observability

### 10.1 Metrics Collection

```rust
/// Comprehensive metrics for Claude integration
#[derive(Debug, Clone, Default)]
pub struct ClaudeMetrics {
    // Request metrics
    total_requests: Arc<AtomicU64>,
    successful_requests: Arc<AtomicU64>,
    failed_requests: Arc<AtomicU64>,
    
    // Latency metrics
    total_latency_ms: Arc<AtomicU64>,
    min_latency_ms: Arc<AtomicU64>,
    max_latency_ms: Arc<AtomicU64>,
    
    // Token metrics
    total_input_tokens: Arc<AtomicU64>,
    total_output_tokens: Arc<AtomicU64>,
    
    // Cost metrics
    total_cost_usd: Arc<AtomicU64>,  // Stored as cents
    
    // Error metrics
    rate_limit_errors: Arc<AtomicU64>,
    timeout_errors: Arc<AtomicU64>,
    parsing_errors: Arc<AtomicU64>,
    
    // Analysis metrics
    mean_seconds_to_midnight: Arc<AtomicU64>,
    mean_confidence: Arc<AtomicU64>,  // Stored as percent
}

impl ClaudeMetrics {
    pub fn record_success(&self, response: &MessageResponse) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        
        if let Some(latency) = response.latency {
            let latency_ms = latency.as_millis() as u64;
            self.total_latency_ms.fetch_add(latency_ms, Ordering::Relaxed);
            
            // Update min/max
            let mut current_min = self.min_latency_ms.load(Ordering::Relaxed);
            while current_min == 0 || latency_ms < current_min {
                match self.min_latency_ms.compare_exchange(
                    current_min,
                    latency_ms,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(x) => current_min = x,
                }
            }
            
            let mut current_max = self.max_latency_ms.load(Ordering::Relaxed);
            while latency_ms > current_max {
                match self.max_latency_ms.compare_exchange(
                    current_max,
                    latency_ms,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(x) => current_max = x,
                }
            }
        }
        
        // Record token usage
        self.total_input_tokens.fetch_add(
            response.usage.input_tokens as u64,
            Ordering::Relaxed,
        );
        self.total_output_tokens.fetch_add(
            response.usage.output_tokens as u64,
            Ordering::Relaxed,
        );
        
        // Calculate cost
        let cost_cents = Self::calculate_cost_cents(&response.usage);
        self.total_cost_usd.fetch_add(cost_cents, Ordering::Relaxed);
    }
    
    pub fn record_error(&self, error: &ClaudeError) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        
        match error {
            ClaudeError::RateLimit(_) => {
                self.rate_limit_errors.fetch_add(1, Ordering::Relaxed);
            }
            ClaudeError::Timeout(_) => {
                self.timeout_errors.fetch_add(1, Ordering::Relaxed);
            }
            ClaudeError::Parsing(_) => {
                self.parsing_errors.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
    }
    
    pub fn export_metrics(&self) -> MetricsSnapshot {
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        
        MetricsSnapshot {
            total_requests,
            successful_requests: successful,
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            success_rate: if total_requests > 0 {
                successful as f64 / total_requests as f64
            } else {
                0.0
            },
            average_latency_ms: if successful > 0 {
                self.total_latency_ms.load(Ordering::Relaxed) / successful
            } else {
                0
            },
            min_latency_ms: self.min_latency_ms.load(Ordering::Relaxed),
            max_latency_ms: self.max_latency_ms.load(Ordering::Relaxed),
            total_input_tokens: self.total_input_tokens.load(Ordering::Relaxed),
            total_output_tokens: self.total_output_tokens.load(Ordering::Relaxed),
            total_cost_usd: self.total_cost_usd.load(Ordering::Relaxed) as f64 / 100.0,
            rate_limit_errors: self.rate_limit_errors.load(Ordering::Relaxed),
            timeout_errors: self.timeout_errors.load(Ordering::Relaxed),
            parsing_errors: self.parsing_errors.load(Ordering::Relaxed),
        }
    }
    
    fn calculate_cost_cents(usage: &Usage) -> u64 {
        const INPUT_COST_PER_MTK: f64 = 3.0;
        const OUTPUT_COST_PER_MTK: f64 = 15.0;
        
        let input_cost = (usage.input_tokens as f64 / 1_000_000.0) * INPUT_COST_PER_MTK;
        let output_cost = (usage.output_tokens as f64 / 1_000_000.0) * OUTPUT_COST_PER_MTK;
        
        ((input_cost + output_cost) * 100.0) as u64  // Convert to cents
    }
}
```

### 10.2 Logging

```rust
/// Comprehensive logging for Claude integration
pub struct ClaudeLogger {
    logger: slog::Logger,
}

impl ClaudeLogger {
    pub fn log_request(&self, request: &MessageRequest) {
        slog::info!(
            self.logger,
            "Claude API request";
            "model" => &request.model,
            "max_tokens" => request.max_tokens,
            "temperature" => request.temperature,
            "estimated_tokens" => Self::estimate_tokens(request),
        );
    }
    
    pub fn log_response(&self, response: &MessageResponse) {
        slog::info!(
            self.logger,
            "Claude API response";
            "id" => &response.id,
            "model" => &response.model,
            "input_tokens" => response.usage.input_tokens,
            "output_tokens" => response.usage.output_tokens,
            "latency_ms" => response.latency.map(|d| d.as_millis()),
            "stop_reason" => &response.stop_reason,
        );
    }
    
    pub fn log_error(&self, error: &ClaudeError) {
        slog::error!(
            self.logger,
            "Claude API error";
            "error_type" => format!("{:?}", error),
            "is_retryable" => error.is_retryable(),
            "retry_delay_ms" => error.retry_delay().as_millis(),
        );
    }
    
    pub fn log_analysis(&self, analysis: &RiskAnalysis) {
        slog::info!(
            self.logger,
            "Risk analysis completed";
            "analysis_id" => &analysis.analysis_id,
            "seconds_to_midnight" => analysis.seconds_to_midnight,
            "risk_level" => format!("{:?}", analysis.risk_level),
            "confidence" => analysis.confidence,
        );
    }
    
    fn estimate_tokens(request: &MessageRequest) -> usize {
        let mut total = 0;
        
        if let Some(system) = &request.system {
            total += system.len() / 4;
        }
        
        for message in &request.messages {
            total += message.content.len() / 4;
        }
        
        total
    }
}
```

---

## Best Practices Summary

### 1. **Always Use Low Temperature (0.1)**
Ensures consistency across multiple analyses of the same data.

### 2. **Implement Comprehensive Error Handling**
- Retry on transient errors
- Circuit breaker for sustained failures
- Graceful degradation

### 3. **Validate All Responses**
- JSON schema validation
- Business rule validation
- Range checks on all numeric values

### 4. **Build Historical Context**
- Include previous assessments
- Show trends and patterns
- Provide baseline for comparison

### 5. **Monitor Token Usage**
- Track daily/monthly budgets
- Optimize prompts for efficiency
- Use prompt caching when possible

### 6. **Test Consistency**
- Multiple runs on same data should agree
- Variance < 5% acceptable
- Flag high disagreement for manual review

### 7. **Log Everything**
- All requests and responses
- Token usage and costs
- Errors and retries
- Analysis results

### 8. **Use Ensemble Methods**
- Run 3-5 analyses for critical assessments
- Build consensus from multiple results
- Detect and investigate disagreements

---

## Support and Resources

**Claude API Documentation:** https://docs.anthropic.com  
**WarGames/JOSHUA Repository:** (internal)  
**Prompt Engineering Guide:** /mnt/project/docs/prompt-engineering.md  
**Troubleshooting Guide:** /mnt/project/docs/troubleshooting.md  

---

**Document Version:** 1.0.0  
**Last Updated:** October 2025  
**Maintained By:** WarGames/JOSHUA Development Team  
**Next Review:** November 2025

*"Through Claude's eyes, we see the patterns in chaos."*
