//! Consensus building from multiple Claude analyses.
//!
//! This module implements ensemble analysis where multiple independent Claude
//! analyses are combined to produce more reliable and robust assessments.

use super::response_parser::ClaudeRiskAnalysis;
use crate::prelude::*;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Consensus analyzer configuration
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    /// Number of independent analyses to run
    pub num_analyses: usize,

    /// Maximum acceptable divergence in seconds
    pub max_divergence_seconds: u32,

    /// Temperature variation for analyses
    pub temperature_range: (f32, f32),
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            num_analyses: 3,
            max_divergence_seconds: 60,
            temperature_range: (0.1, 0.3),
        }
    }
}

/// Result of consensus analysis
#[derive(Debug, Clone)]
pub struct ConsensusAnalysis {
    /// Consensus seconds to midnight (median)
    pub consensus_seconds: u32,

    /// Mean seconds to midnight
    pub mean_seconds: f64,

    /// Standard deviation
    pub std_dev: f64,

    /// Confidence in consensus
    pub confidence: f64,

    /// Risk level
    pub risk_level: String,

    /// Aggregated risk factors
    pub risk_factors: HashMap<String, f64>,

    /// Combined critical developments
    pub critical_developments: Vec<String>,

    /// Combined early warning indicators
    pub early_warning_indicators: Vec<String>,

    /// Consensus executive summary
    pub executive_summary: String,

    /// All individual analyses
    pub individual_analyses: Vec<ClaudeRiskAnalysis>,

    /// Agreement level (0.0 to 1.0)
    pub agreement_level: f64,

    /// Combined recommendations
    pub recommendations: Vec<String>,
}

/// Consensus analyzer
pub struct ConsensusAnalyzer {
    config: ConsensusConfig,
}

impl ConsensusAnalyzer {
    /// Create a new consensus analyzer
    pub fn new(config: ConsensusConfig) -> Self {
        Self { config }
    }

    /// Build consensus from multiple analyses
    pub fn build_consensus(&self, analyses: Vec<ClaudeRiskAnalysis>) -> Result<ConsensusAnalysis> {
        if analyses.len() < 2 {
            return Err(Error::Analysis(
                "Need at least 2 analyses for consensus".to_string(),
            ));
        }

        info!(
            num_analyses = analyses.len(),
            "Building consensus from analyses"
        );

        // Extract seconds to midnight values
        let seconds: Vec<u32> = analyses.iter().map(|a| a.seconds_to_midnight).collect();

        // Calculate statistics
        let (mean, std_dev, median) = self.calculate_statistics(&seconds);

        debug!(
            mean = mean,
            std_dev = std_dev,
            median = median,
            "Consensus statistics"
        );

        // Check for excessive divergence
        let divergence = self.calculate_divergence(&seconds);
        if divergence > self.config.max_divergence_seconds {
            warn!(
                divergence = divergence,
                threshold = self.config.max_divergence_seconds,
                "High divergence detected in analyses"
            );
        }

        // Aggregate risk factors (mean)
        let risk_factors = self.aggregate_risk_factors(&analyses);

        // Combine critical developments (unique)
        let critical_developments = self.merge_critical_developments(&analyses);

        // Combine early warning indicators (unique)
        let early_warning_indicators = self.merge_early_warning_indicators(&analyses);

        // Calculate consensus confidence
        let confidence = self.calculate_consensus_confidence(&analyses, std_dev);

        // Calculate agreement level
        let agreement_level = self.calculate_agreement_level(&seconds, mean);

        // Build consensus summary
        let executive_summary = self.build_consensus_summary(&analyses);

        // Merge recommendations
        let recommendations = self.merge_recommendations(&analyses);

        // Determine risk level
        let risk_level = self.seconds_to_risk_level(median);

        Ok(ConsensusAnalysis {
            consensus_seconds: median,
            mean_seconds: mean,
            std_dev,
            confidence,
            risk_level,
            risk_factors,
            critical_developments,
            early_warning_indicators,
            executive_summary,
            individual_analyses: analyses,
            agreement_level,
            recommendations,
        })
    }

    /// Calculate statistics (mean, std_dev, median)
    fn calculate_statistics(&self, values: &[u32]) -> (f64, f64, u32) {
        let mean = values.iter().sum::<u32>() as f64 / values.len() as f64;

        let variance = values
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;
        let std_dev = variance.sqrt();

        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        let median = sorted[sorted.len() / 2];

        (mean, std_dev, median)
    }

    /// Calculate divergence (max - min)
    fn calculate_divergence(&self, values: &[u32]) -> u32 {
        let min = *values.iter().min().unwrap_or(&0);
        let max = *values.iter().max().unwrap_or(&0);
        max - min
    }

    /// Aggregate risk factors across analyses
    fn aggregate_risk_factors(&self, analyses: &[ClaudeRiskAnalysis]) -> HashMap<String, f64> {
        let mut aggregated: HashMap<String, Vec<f64>> = HashMap::new();

        for analysis in analyses {
            for (factor, &value) in &analysis.risk_factors {
                aggregated.entry(factor.clone()).or_default().push(value);
            }
        }

        // Calculate mean for each factor
        aggregated
            .into_iter()
            .map(|(factor, values)| {
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                (factor, mean)
            })
            .collect()
    }

    /// Merge critical developments (deduplicate)
    fn merge_critical_developments(&self, analyses: &[ClaudeRiskAnalysis]) -> Vec<String> {
        let mut developments = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for analysis in analyses {
            for dev in &analysis.critical_developments {
                let key = dev.event.to_lowercase();
                if !seen.contains(&key) {
                    seen.insert(key);
                    developments.push(format!(
                        "{} (Impact: {:?}, Escalation: {:.2})",
                        dev.event, dev.impact, dev.escalation_potential
                    ));
                }
            }
        }

        developments
    }

    /// Merge early warning indicators (deduplicate)
    fn merge_early_warning_indicators(&self, analyses: &[ClaudeRiskAnalysis]) -> Vec<String> {
        let mut indicators = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for analysis in analyses {
            for indicator in &analysis.early_warning_indicators {
                let key = indicator.to_lowercase();
                if !seen.contains(&key) {
                    seen.insert(key);
                    indicators.push(indicator.clone());
                }
            }
        }

        indicators
    }

    /// Calculate consensus confidence
    fn calculate_consensus_confidence(&self, analyses: &[ClaudeRiskAnalysis], std_dev: f64) -> f64 {
        // Average confidence of individual analyses
        let mean_confidence = analyses
            .iter()
            .map(|a| a.confidence_level.to_score())
            .sum::<f64>()
            / analyses.len() as f64;

        // Penalize high variance
        let variance_penalty = (std_dev / 100.0).min(0.3);

        (mean_confidence - variance_penalty).max(0.0).min(1.0)
    }

    /// Calculate agreement level
    fn calculate_agreement_level(&self, values: &[u32], mean: f64) -> f64 {
        let variance = values
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;

        // High variance = low agreement
        1.0 - (variance.sqrt() / mean).min(1.0)
    }

    /// Build consensus summary
    fn build_consensus_summary(&self, analyses: &[ClaudeRiskAnalysis]) -> String {
        // Take the most common key points from individual summaries
        // For simplicity, we'll concatenate the first analysis's summary
        // In production, this would use NLP to extract common themes

        if analyses.is_empty() {
            return "No analyses available for consensus".to_string();
        }

        analyses[0].executive_summary.clone()
    }

    /// Merge recommendations (deduplicate)
    fn merge_recommendations(&self, analyses: &[ClaudeRiskAnalysis]) -> Vec<String> {
        let mut recommendations = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for analysis in analyses {
            for rec in &analysis.recommendations {
                let key = rec.to_lowercase();
                if !seen.contains(&key) {
                    seen.insert(key);
                    recommendations.push(rec.clone());
                }
            }
        }

        recommendations
    }

    /// Convert seconds to risk level
    fn seconds_to_risk_level(&self, seconds: u32) -> String {
        use crate::constants::*;
        match seconds {
            0..=CRITICAL_THRESHOLD => "Critical",
            ..=SEVERE_THRESHOLD => "Severe",
            ..=HIGH_THRESHOLD => "High",
            ..=MODERATE_THRESHOLD => "Moderate",
            _ => "Low",
        }
        .to_string()
    }
}

impl Default for ConsensusAnalyzer {
    fn default() -> Self {
        Self::new(ConsensusConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_analysis(seconds: u32) -> ClaudeRiskAnalysis {
        ClaudeRiskAnalysis {
            seconds_to_midnight: seconds,
            confidence_level: ConfidenceLevel::High,
            trend_direction: TrendDirection::Stable,
            risk_factors: HashMap::new(),
            critical_developments: Vec::new(),
            early_warning_indicators: Vec::new(),
            executive_summary: "Test summary".to_string(),
            detailed_analysis: "Test analysis".to_string(),
            recommendations: Vec::new(),
        }
    }

    #[test]
    fn test_consensus_builder() {
        let analyzer = ConsensusAnalyzer::default();

        let analyses = vec![
            create_test_analysis(90),
            create_test_analysis(95),
            create_test_analysis(88),
        ];

        let consensus = analyzer.build_consensus(analyses);
        assert!(consensus.is_ok());

        let result = consensus.unwrap();
        assert_eq!(result.consensus_seconds, 90); // Median
        assert!(result.agreement_level > 0.9); // High agreement
    }

    #[test]
    fn test_high_divergence_detection() {
        let analyzer = ConsensusAnalyzer::default();

        let analyses = vec![
            create_test_analysis(50),
            create_test_analysis(150),
            create_test_analysis(100),
        ];

        let consensus = analyzer.build_consensus(analyses);
        assert!(consensus.is_ok());

        let result = consensus.unwrap();
        // Should still work but flag high divergence
        assert!(result.std_dev > 20.0);
    }

    #[test]
    fn test_insufficient_analyses() {
        let analyzer = ConsensusAnalyzer::default();
        let analyses = vec![create_test_analysis(90)];

        let consensus = analyzer.build_consensus(analyses);
        assert!(consensus.is_err());
    }
}
