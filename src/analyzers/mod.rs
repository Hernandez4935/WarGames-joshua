//! Risk analyzers for assessing nuclear threats.

use crate::collectors::AggregatedData;
use crate::prelude::*;
use async_trait::async_trait;

/// Trait for risk analysis components
#[async_trait]
pub trait RiskAnalyzer: Send + Sync {
    /// Analyze risk factors and return assessment
    async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis>;

    /// Get the risk category this analyzer covers
    fn risk_category(&self) -> RiskCategory;

    /// Weight of this analyzer's contribution to overall risk
    fn weight(&self) -> f64;

    /// Minimum confidence level required for analysis
    fn min_confidence(&self) -> ConfidenceLevel {
        ConfidenceLevel::Low
    }
}

/// Result of risk analysis
#[derive(Debug, Clone)]
pub struct RiskAnalysis {
    /// Risk category analyzed
    pub category: RiskCategory,

    /// Overall score for this category (0.0 to 1.0)
    pub overall_score: f64,

    /// Confidence in this analysis
    pub confidence: ConfidenceLevel,

    /// Individual risk factors identified
    pub risk_factors: Vec<RiskFactor>,

    /// Summary of findings
    pub summary: String,

    /// Recommendations
    pub recommendations: Vec<String>,
}

impl RiskAnalysis {
    /// Create a new risk analysis
    #[must_use]
    pub fn new(category: RiskCategory) -> Self {
        Self {
            category,
            overall_score: 0.0,
            confidence: ConfidenceLevel::Moderate,
            risk_factors: Vec::new(),
            summary: String::new(),
            recommendations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_analysis_creation() {
        let analysis = RiskAnalysis::new(RiskCategory::RegionalConflicts);
        assert_eq!(analysis.category, RiskCategory::RegionalConflicts);
    }
}
