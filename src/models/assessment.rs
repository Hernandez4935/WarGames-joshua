//! Risk assessment data model.

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Complete risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    /// Unique assessment identifier
    pub id: Uuid,

    /// When the assessment was performed
    pub assessment_date: DateTime<Utc>,

    /// Seconds to midnight (Doomsday Clock metric)
    pub seconds_to_midnight: u32,

    /// Raw risk score (0.0 to 1.0)
    pub raw_risk_score: f64,

    /// Bayesian-adjusted risk score
    pub bayesian_adjusted_score: f64,

    /// Overall confidence in assessment
    pub overall_confidence: ConfidenceLevel,

    /// Trend direction compared to previous assessment
    pub trend_direction: TrendDirection,

    /// Magnitude of trend change
    pub trend_magnitude: f64,

    /// Change from previous assessment (in seconds)
    pub delta_from_previous: Option<i32>,

    /// Executive summary
    pub executive_summary: String,

    /// Detailed analysis text
    pub detailed_analysis: String,

    /// Risk factors contributing to assessment
    pub risk_factors: Vec<RiskFactor>,

    /// Critical warnings
    pub critical_warnings: Vec<String>,

    /// Recommendations
    pub recommendations: Vec<String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl Assessment {
    /// Create a new assessment
    pub fn new(seconds_to_midnight: u32, executive_summary: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            assessment_date: Utc::now(),
            seconds_to_midnight,
            raw_risk_score: Self::seconds_to_risk_score(seconds_to_midnight),
            bayesian_adjusted_score: Self::seconds_to_risk_score(seconds_to_midnight),
            overall_confidence: ConfidenceLevel::Moderate,
            trend_direction: TrendDirection::Stable,
            trend_magnitude: 0.0,
            delta_from_previous: None,
            executive_summary,
            detailed_analysis: String::new(),
            risk_factors: Vec::new(),
            critical_warnings: Vec::new(),
            recommendations: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Convert seconds to midnight to risk score (0.0 to 1.0)
    fn seconds_to_risk_score(seconds: u32) -> f64 {
        1.0 - (f64::from(seconds) / f64::from(crate::constants::MAX_SECONDS_TO_MIDNIGHT))
    }

    /// Get risk level as string
    pub fn risk_level(&self) -> &str {
        use crate::constants::*;
        match self.seconds_to_midnight {
            0..=CRITICAL_THRESHOLD => "Critical",
            ..=SEVERE_THRESHOLD => "Severe",
            ..=HIGH_THRESHOLD => "High",
            ..=MODERATE_THRESHOLD => "Moderate",
            _ => "Low",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessment_creation() {
        let assessment = Assessment::new(89, "Test summary".to_string());
        assert_eq!(assessment.seconds_to_midnight, 89);
        assert!(assessment.raw_risk_score > 0.9);
    }

    #[test]
    fn test_risk_level_categorization() {
        let assessment = Assessment::new(50, "Critical".to_string());
        assert_eq!(assessment.risk_level(), "Critical");
    }
}
