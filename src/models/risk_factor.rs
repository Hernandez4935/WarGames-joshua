//! Risk factor data model.

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Individual risk factor contributing to assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Unique identifier
    pub id: Uuid,

    /// Risk category
    pub category: RiskCategory,

    /// Factor name
    pub name: String,

    /// Raw value (0.0 to 1.0)
    pub value: f64,

    /// Weighted value
    pub weighted_value: f64,

    /// Contribution to overall risk
    pub contribution_to_risk: f64,

    /// Confidence level
    pub confidence: ConfidenceLevel,

    /// Supporting data sources
    pub data_sources: Vec<String>,

    /// Timestamp when observed
    pub timestamp: DateTime<Utc>,

    /// Trend compared to historical baseline
    pub trend: Option<TrendDirection>,
}

impl RiskFactor {
    /// Create a new risk factor
    pub fn new(
        category: RiskCategory,
        name: String,
        value: f64,
        confidence: ConfidenceLevel,
    ) -> Self {
        let category_weight = category.default_weight();
        let weighted_value = value * category_weight;

        Self {
            id: Uuid::new_v4(),
            category,
            name,
            value,
            weighted_value,
            contribution_to_risk: weighted_value,
            confidence,
            data_sources: Vec::new(),
            timestamp: Utc::now(),
            trend: None,
        }
    }

    /// Add a data source
    pub fn add_source(&mut self, source: String) {
        self.data_sources.push(source);
    }

    /// Set trend direction
    pub fn set_trend(&mut self, trend: TrendDirection) {
        self.trend = Some(trend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_factor_creation() {
        let factor = RiskFactor::new(
            RiskCategory::RegionalConflicts,
            "Ukraine Crisis".to_string(),
            0.8,
            ConfidenceLevel::High,
        );
        assert_eq!(factor.category, RiskCategory::RegionalConflicts);
        assert!(factor.weighted_value > 0.0);
    }
}
