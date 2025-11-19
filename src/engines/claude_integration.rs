//! Claude AI integration engine.
//!
//! This is the primary orchestrator for Claude-powered nuclear risk analysis.
//! It coordinates data collection, prompt construction, API calls, response parsing,
//! and consensus building.

use crate::analyzers::{
    ClaudeClient, ClaudeConfig, ConsensusAnalyzer, ConsensusConfig, MessageRequest, PromptBuilder,
    ResponseParser,
};
use crate::collectors::AggregatedData;
use crate::prelude::*;
use crate::utils::config::Config;
use std::env;
use tracing::{debug, error, info, warn};

/// Claude integration engine for AI-powered analysis
pub struct ClaudeIntegrationEngine {
    client: ClaudeClient,
    prompt_builder: PromptBuilder,
    response_parser: ResponseParser,
    consensus_analyzer: ConsensusAnalyzer,
    enable_consensus: bool,
}

impl ClaudeIntegrationEngine {
    /// Create a new Claude integration engine from configuration
    pub fn new(config: &Config) -> Result<Self> {
        // Load API key from environment
        let api_key = env::var(&config.claude_api.api_key_env).map_err(|_| {
            Error::Configuration(format!(
                "Claude API key not found in environment variable: {}",
                config.claude_api.api_key_env
            ))
        })?;

        // Build Claude client configuration
        let client_config = ClaudeConfig {
            api_key,
            model: config.claude_api.model.clone(),
            max_tokens: config.claude_api.max_tokens,
            temperature: config.claude_api.temperature as f32,
            ..ClaudeConfig::default()
        };

        let client = ClaudeClient::new(client_config)?;
        let prompt_builder = PromptBuilder::new();
        let response_parser = ResponseParser::new();

        let consensus_config = ConsensusConfig {
            num_analyses: 3, // Run 3 independent analyses
            max_divergence_seconds: 60,
            temperature_range: (0.1, 0.3),
        };
        let consensus_analyzer = ConsensusAnalyzer::new(consensus_config);

        Ok(Self {
            client,
            prompt_builder,
            response_parser,
            consensus_analyzer,
            enable_consensus: true, // Can be made configurable
        })
    }

    /// Create a simple instance for testing (requires API key in env)
    pub fn new_from_env() -> Result<Self> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| Error::Configuration("ANTHROPIC_API_KEY not set".to_string()))?;

        let client_config = ClaudeConfig {
            api_key,
            ..ClaudeConfig::default()
        };

        let client = ClaudeClient::new(client_config)?;

        Ok(Self {
            client,
            prompt_builder: PromptBuilder::new(),
            response_parser: ResponseParser::new(),
            consensus_analyzer: ConsensusAnalyzer::default(),
            enable_consensus: false, // Single analysis for simple use
        })
    }

    /// Analyze risk using Claude AI (primary method)
    pub async fn analyze_risk(
        &self,
        data: &AggregatedData,
        previous_assessment: Option<&Assessment>,
    ) -> Result<Assessment> {
        info!(
            data_points = data.data_points.len(),
            sources = data.sources_count,
            enable_consensus = self.enable_consensus,
            "Starting Claude AI risk analysis"
        );

        if self.enable_consensus {
            self.analyze_with_consensus(data, previous_assessment).await
        } else {
            self.analyze_single(data, previous_assessment).await
        }
    }

    /// Run single Claude analysis
    async fn analyze_single(
        &self,
        data: &AggregatedData,
        previous_assessment: Option<&Assessment>,
    ) -> Result<Assessment> {
        // Build prompt
        let prompt = self
            .prompt_builder
            .build_risk_assessment_prompt(data, previous_assessment)?;

        debug!(prompt_length = prompt.len(), "Built analysis prompt");

        // Create request
        let request = MessageRequest::new(
            crate::constants::CLAUDE_MODEL,
            crate::constants::CLAUDE_MAX_TOKENS,
        )
        .with_system(self.prompt_builder.system_prompt())
        .add_user_message(prompt)
        .with_temperature(crate::constants::CLAUDE_TEMPERATURE as f32);

        // Call Claude API
        let response = self.client.messages_create(request).await?;

        info!(
            response_id = %response.id,
            input_tokens = response.usage.input_tokens,
            output_tokens = response.usage.output_tokens,
            cost_usd = response.estimated_cost(),
            "Claude API call successful"
        );

        // Parse response
        let analysis_text = response.extract_text();
        let claude_analysis = self.response_parser.parse_risk_analysis(&analysis_text)?;

        // Convert to Assessment
        let mut assessment = self.response_parser.to_assessment(claude_analysis);

        // Set delta from previous if available
        if let Some(prev) = previous_assessment {
            let delta = assessment.seconds_to_midnight as i32 - prev.seconds_to_midnight as i32;
            assessment.delta_from_previous = Some(delta);
        }

        info!(
            seconds_to_midnight = assessment.seconds_to_midnight,
            risk_level = assessment.risk_level(),
            confidence = ?assessment.overall_confidence,
            "Risk analysis complete"
        );

        Ok(assessment)
    }

    /// Run multiple analyses and build consensus
    async fn analyze_with_consensus(
        &self,
        data: &AggregatedData,
        previous_assessment: Option<&Assessment>,
    ) -> Result<Assessment> {
        info!("Running consensus analysis with multiple Claude calls");

        // Build prompt (same for all analyses)
        let prompt = self
            .prompt_builder
            .build_risk_assessment_prompt(data, previous_assessment)?;

        // Run multiple analyses in parallel
        let num_analyses = 3; // From consensus config
        let mut handles = Vec::new();

        for i in 0..num_analyses {
            let client = &self.client;
            let parser = &self.response_parser;
            let system_prompt = self.prompt_builder.system_prompt().to_string();
            let prompt_clone = prompt.clone();

            // Vary temperature slightly for diversity
            let temperature = 0.1 + (i as f32 * 0.1);

            debug!(
                analysis_num = i + 1,
                temperature = temperature,
                "Starting analysis"
            );

            // Create request
            let request = MessageRequest::new(
                crate::constants::CLAUDE_MODEL,
                crate::constants::CLAUDE_MAX_TOKENS,
            )
            .with_system(system_prompt)
            .add_user_message(prompt_clone)
            .with_temperature(temperature);

            // Call API
            let response = client.messages_create(request).await?;
            let analysis_text = response.extract_text();

            // Parse
            match parser.parse_risk_analysis(&analysis_text) {
                Ok(analysis) => handles.push(analysis),
                Err(e) => {
                    warn!(error = %e, "Failed to parse analysis {}, continuing", i + 1);
                }
            }
        }

        // Ensure we have at least 2 analyses
        if handles.len() < 2 {
            return Err(Error::Analysis(
                "Insufficient successful analyses for consensus".to_string(),
            ));
        }

        info!(
            successful_analyses = handles.len(),
            "Building consensus from analyses"
        );

        // Build consensus
        let consensus = self.consensus_analyzer.build_consensus(handles)?;

        info!(
            consensus_seconds = consensus.consensus_seconds,
            agreement_level = consensus.agreement_level,
            confidence = consensus.confidence,
            "Consensus analysis complete"
        );

        // Convert consensus to Assessment
        let mut assessment = Assessment::new(
            consensus.consensus_seconds,
            consensus.executive_summary.clone(),
        );

        assessment.overall_confidence = ConfidenceLevel::from_score(consensus.confidence);
        assessment.raw_risk_score = 1.0
            - (consensus.consensus_seconds as f64
                / crate::constants::MAX_SECONDS_TO_MIDNIGHT as f64);
        assessment.bayesian_adjusted_score = assessment.raw_risk_score;
        assessment.recommendations = consensus.recommendations.clone();
        assessment.critical_warnings = consensus.critical_developments.clone();

        // Create risk factors from consensus
        for (name, score) in consensus.risk_factors {
            let category = self.parse_risk_category(&name);
            let mut risk_factor =
                RiskFactor::new(category, name.clone(), score, assessment.overall_confidence);
            risk_factor.set_trend(TrendDirection::Stable);
            assessment.risk_factors.push(risk_factor);
        }

        // Set delta from previous if available
        if let Some(prev) = previous_assessment {
            let delta = assessment.seconds_to_midnight as i32 - prev.seconds_to_midnight as i32;
            assessment.delta_from_previous = Some(delta);

            // Set trend direction
            assessment.trend_direction = if delta < -10 {
                TrendDirection::Deteriorating
            } else if delta > 10 {
                TrendDirection::Improving
            } else {
                TrendDirection::Stable
            };
        }

        Ok(assessment)
    }

    /// Parse risk category from string
    fn parse_risk_category(&self, name: &str) -> RiskCategory {
        match name {
            "nuclear_arsenal_changes" => RiskCategory::NuclearArsenalChanges,
            "arms_control_breakdown" => RiskCategory::ArmsControlBreakdown,
            "regional_conflicts" => RiskCategory::RegionalConflicts,
            "leadership_instability" => RiskCategory::LeadershipInstability,
            "technical_incidents" => RiskCategory::TechnicalIncidents,
            "communication_failures" => RiskCategory::CommunicationFailures,
            "emerging_tech_risks" => RiskCategory::EmergingTechRisks,
            "economic_pressure" => RiskCategory::EconomicPressure,
            _ => RiskCategory::RegionalConflicts,
        }
    }

    /// Get client metrics
    pub fn metrics(&self) -> String {
        let metrics = self.client.metrics();
        format!(
            "Success Rate: {:.1}%, Avg Latency: {}ms, Total Cost: ${:.4}",
            metrics.success_rate() * 100.0,
            metrics.average_latency_ms(),
            metrics.total_cost_usd()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_risk_category() {
        let engine = ClaudeIntegrationEngine::new_from_env().ok();
        if let Some(e) = engine {
            assert_eq!(
                e.parse_risk_category("nuclear_arsenal_changes"),
                RiskCategory::NuclearArsenalChanges
            );
        }
    }
}
