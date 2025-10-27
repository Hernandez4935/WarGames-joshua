//! Data quality scoring system.

use crate::models::DataPoint;
use crate::prelude::*;
use chrono::Duration;

/// Data quality scorer
pub struct DataQualityScorer;

impl DataQualityScorer {
    /// Create a new quality scorer
    pub fn new() -> Self {
        Self
    }

    /// Calculate overall quality score for a data point (0.0 to 1.0)
    pub fn score(&self, data_point: &DataPoint) -> f64 {
        let source_score = self.score_source(data_point);
        let timeliness_score = self.score_timeliness(data_point);
        let completeness_score = self.score_completeness(data_point);
        let relevance_score = data_point.reliability;

        // Weighted average
        let weights = [0.30, 0.20, 0.10, 0.40]; // source, timeliness, completeness, relevance
        let scores = [
            source_score,
            timeliness_score,
            completeness_score,
            relevance_score,
        ];

        weights.iter().zip(scores.iter()).map(|(w, s)| w * s).sum()
    }

    /// Score based on source reliability
    fn score_source(&self, data_point: &DataPoint) -> f64 {
        data_point.reliability
    }

    /// Score based on data timeliness
    fn score_timeliness(&self, data_point: &DataPoint) -> f64 {
        let age = Utc::now() - data_point.collected_at;

        if age < Duration::days(1) {
            1.0
        } else if age < Duration::days(7) {
            0.9
        } else if age < Duration::days(30) {
            0.7
        } else if age < Duration::days(90) {
            0.5
        } else {
            0.3
        }
    }

    /// Score based on data completeness
    fn score_completeness(&self, data_point: &DataPoint) -> f64 {
        let mut score: f64 = 0.5; // Base score

        if data_point.title.is_some() {
            score += 0.15;
        }

        if data_point.source_url.is_some() {
            score += 0.15;
        }

        if data_point.published_at.is_some() {
            score += 0.10;
        }

        if !data_point.metadata.is_empty() {
            score += 0.10;
        }

        score.min(1.0)
    }

    /// Filter data points by minimum quality score
    pub fn filter_by_quality(&self, data_points: Vec<DataPoint>, min_score: f64) -> Vec<DataPoint> {
        data_points
            .into_iter()
            .filter(|dp| self.score(dp) >= min_score)
            .collect()
    }
}

impl Default for DataQualityScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DataCategory;

    #[test]
    fn test_quality_scorer() {
        let scorer = DataQualityScorer::new();

        let point = DataPoint::new(
            "Test Source".to_string(),
            "Test content".to_string(),
            DataCategory::NewsMedia,
        )
        .with_reliability(0.9)
        .with_url("http://example.com".to_string())
        .with_title("Test Title".to_string());

        let score = scorer.score(&point);
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_timeliness_score() {
        let scorer = DataQualityScorer::new();

        let mut point = DataPoint::new(
            "Test".to_string(),
            "Content".to_string(),
            DataCategory::NewsMedia,
        );

        // Fresh data
        let score1 = scorer.score_timeliness(&point);
        assert_eq!(score1, 1.0);

        // Old data
        point.collected_at = Utc::now() - Duration::days(100);
        let score2 = scorer.score_timeliness(&point);
        assert!(score2 < score1);
    }

    #[test]
    fn test_filter_by_quality() {
        let scorer = DataQualityScorer::new();

        let points = vec![
            DataPoint::new(
                "High Quality".to_string(),
                "Content".to_string(),
                DataCategory::NewsMedia,
            )
            .with_reliability(0.9)
            .with_url("http://example.com".to_string())
            .with_title("Title".to_string()),
            DataPoint::new(
                "Low Quality".to_string(),
                "Content".to_string(),
                DataCategory::NewsMedia,
            )
            .with_reliability(0.2),
        ];

        let filtered = scorer.filter_by_quality(points, 0.5);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].source, "High Quality");
    }
}
