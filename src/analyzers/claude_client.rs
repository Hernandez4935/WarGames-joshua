//! Claude API client with retry logic and rate limiting.
//!
//! This module implements a production-grade client for the Anthropic Claude API
//! with comprehensive error handling, retry logic, and rate limiting.

use super::claude_models::{ErrorResponse, MessageRequest, MessageResponse};
use crate::prelude::*;
use parking_lot::Mutex;
use reqwest::StatusCode;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Claude API client configuration
#[derive(Debug, Clone)]
pub struct ClaudeConfig {
    /// API base URL
    pub base_url: String,

    /// API key (should be encrypted at rest)
    pub api_key: String,

    /// Model to use
    pub model: String,

    /// Maximum tokens for responses
    pub max_tokens: u32,

    /// Temperature for generation
    pub temperature: f32,

    /// Request timeout
    pub request_timeout: Duration,

    /// Connect timeout
    pub connect_timeout: Duration,

    /// Maximum retry attempts
    pub max_retries: u32,

    /// Base retry delay
    pub retry_delay: Duration,

    /// Use exponential backoff
    pub exponential_backoff: bool,

    /// Maximum retry delay
    pub max_retry_delay: Duration,

    /// Requests per minute limit
    pub requests_per_minute: u32,

    /// Tokens per minute limit
    pub tokens_per_minute: u32,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            base_url: crate::constants::CLAUDE_API_BASE.to_string(),
            api_key: String::new(),
            model: crate::constants::CLAUDE_MODEL.to_string(),
            max_tokens: crate::constants::CLAUDE_MAX_TOKENS,
            temperature: crate::constants::CLAUDE_TEMPERATURE as f32,
            request_timeout: Duration::from_secs(180),
            connect_timeout: Duration::from_secs(30),
            max_retries: crate::constants::MAX_RETRY_ATTEMPTS,
            retry_delay: Duration::from_secs(crate::constants::RETRY_DELAY_SECONDS),
            exponential_backoff: true,
            max_retry_delay: Duration::from_secs(60),
            requests_per_minute: 50,
            tokens_per_minute: 40_000,
        }
    }
}

/// Claude API client
pub struct ClaudeClient {
    config: ClaudeConfig,
    http_client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
    metrics: Arc<ClaudeMetrics>,
}

impl ClaudeClient {
    /// Create a new Claude API client
    pub fn new(config: ClaudeConfig) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(config.request_timeout)
            .connect_timeout(config.connect_timeout)
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .pool_max_idle_per_host(10)
            .build()
            .map_err(|e| Error::Configuration(format!("Failed to create HTTP client: {}", e)))?;

        let rate_limiter = Arc::new(RateLimiter::new(
            config.requests_per_minute,
            config.tokens_per_minute,
        ));

        Ok(Self {
            config,
            http_client,
            rate_limiter,
            metrics: Arc::new(ClaudeMetrics::default()),
        })
    }

    /// Send message to Claude API with automatic retry
    pub async fn messages_create(&self, request: MessageRequest) -> Result<MessageResponse> {
        // Wait for rate limit
        self.rate_limiter
            .acquire(request.estimated_tokens())
            .await?;

        // Execute with retry logic
        let mut last_error = None;
        for attempt in 0..self.config.max_retries {
            match self.execute_request(&request).await {
                Ok(response) => {
                    self.metrics.record_success(&response);
                    return Ok(response);
                }
                Err(e) if Self::is_retryable(&e) && attempt < self.config.max_retries - 1 => {
                    let delay = self.calculate_retry_delay(attempt);
                    warn!(
                        attempt = attempt + 1,
                        max_retries = self.config.max_retries,
                        delay_ms = delay.as_millis(),
                        error = %e,
                        "Claude API request failed, retrying"
                    );
                    last_error = Some(e);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    self.metrics.record_error();
                    return Err(e);
                }
            }
        }

        self.metrics.record_error();
        Err(last_error.unwrap_or_else(|| Error::Other("Max retries exceeded".to_string())))
    }

    /// Execute single API request
    async fn execute_request(&self, request: &MessageRequest) -> Result<MessageResponse> {
        let start = Instant::now();

        debug!(
            model = %request.model,
            estimated_tokens = request.estimated_tokens(),
            "Sending Claude API request"
        );

        let response = self
            .http_client
            .post(format!("{}/messages", self.config.base_url))
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", crate::constants::CLAUDE_API_VERSION)
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

        info!(
            response_id = %api_response.id,
            input_tokens = api_response.usage.input_tokens,
            output_tokens = api_response.usage.output_tokens,
            latency_ms = latency.as_millis(),
            cost_usd = api_response.estimated_cost(),
            "Claude API request successful"
        );

        Ok(api_response)
    }

    /// Check if error is retryable
    fn is_retryable(error: &Error) -> bool {
        matches!(
            error,
            Error::RateLimit { .. } | Error::Timeout(_) | Error::Http(_)
        )
    }

    /// Calculate retry delay with optional exponential backoff
    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        if !self.config.exponential_backoff {
            return self.config.retry_delay;
        }

        let delay = self.config.retry_delay * 2_u32.pow(attempt);
        std::cmp::min(delay, self.config.max_retry_delay)
    }

    /// Parse HTTP error response
    fn parse_error(status: StatusCode, body: &str) -> Error {
        // Try to parse structured error
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(body) {
            let message = format!(
                "{}: {}",
                error_response.error.type_, error_response.error.message
            );

            return match status {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Authentication(message),
                StatusCode::TOO_MANY_REQUESTS => Error::RateLimit {
                    resource: "Claude API".to_string(),
                },
                _ => Error::claude_api(message, Some(status.as_u16())),
            };
        }

        // Fallback to generic error
        Error::claude_api(format!("HTTP {} - {}", status, body), Some(status.as_u16()))
    }

    /// Get metrics
    pub fn metrics(&self) -> &ClaudeMetrics {
        &self.metrics
    }
}

/// Simple rate limiter
pub struct RateLimiter {
    requests_per_minute: u32,
    tokens_per_minute: u32,
    request_count: AtomicU32,
    token_count: AtomicU32,
    window_start: Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32, tokens_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            tokens_per_minute,
            request_count: AtomicU32::new(0),
            token_count: AtomicU32::new(0),
            window_start: Mutex::new(Instant::now()),
        }
    }

    pub async fn acquire(&self, tokens: u32) -> Result<()> {
        loop {
            // Check if we need to reset the window
            {
                let mut window_start = self.window_start.lock();
                if window_start.elapsed() >= Duration::from_secs(60) {
                    self.request_count.store(0, Ordering::Relaxed);
                    self.token_count.store(0, Ordering::Relaxed);
                    *window_start = Instant::now();
                }
            }

            let requests = self.request_count.load(Ordering::Relaxed);
            let current_tokens = self.token_count.load(Ordering::Relaxed);

            if requests < self.requests_per_minute
                && current_tokens + tokens < self.tokens_per_minute
            {
                self.request_count.fetch_add(1, Ordering::Relaxed);
                self.token_count.fetch_add(tokens, Ordering::Relaxed);
                return Ok(());
            }

            // Wait a bit before retrying
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

/// Metrics for Claude API usage
#[derive(Debug, Default)]
pub struct ClaudeMetrics {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_latency_ms: AtomicU64,
    total_input_tokens: AtomicU64,
    total_output_tokens: AtomicU64,
}

impl ClaudeMetrics {
    pub fn record_success(&self, response: &MessageResponse) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);

        if let Some(latency) = response.latency {
            self.total_latency_ms
                .fetch_add(latency.as_millis() as u64, Ordering::Relaxed);
        }

        self.total_input_tokens
            .fetch_add(response.usage.input_tokens as u64, Ordering::Relaxed);
        self.total_output_tokens
            .fetch_add(response.usage.output_tokens as u64, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let successful = self.successful_requests.load(Ordering::Relaxed);
        successful as f64 / total as f64
    }

    pub fn average_latency_ms(&self) -> u64 {
        let total = self.successful_requests.load(Ordering::Relaxed);
        if total == 0 {
            return 0;
        }
        let total_latency = self.total_latency_ms.load(Ordering::Relaxed);
        total_latency / total
    }

    pub fn total_cost_usd(&self) -> f64 {
        const INPUT_COST_PER_MTK: f64 = 3.0;
        const OUTPUT_COST_PER_MTK: f64 = 15.0;

        let input_tokens = self.total_input_tokens.load(Ordering::Relaxed) as f64;
        let output_tokens = self.total_output_tokens.load(Ordering::Relaxed) as f64;

        let input_cost = (input_tokens / 1_000_000.0) * INPUT_COST_PER_MTK;
        let output_cost = (output_tokens / 1_000_000.0) * OUTPUT_COST_PER_MTK;

        input_cost + output_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = ClaudeConfig::default();
        assert_eq!(config.model, crate::constants::CLAUDE_MODEL);
        assert_eq!(config.max_tokens, crate::constants::CLAUDE_MAX_TOKENS);
    }

    #[test]
    fn test_metrics_tracking() {
        let metrics = ClaudeMetrics::default();
        assert_eq!(metrics.success_rate(), 0.0);

        metrics.record_error();
        assert_eq!(metrics.success_rate(), 0.0);
    }
}
