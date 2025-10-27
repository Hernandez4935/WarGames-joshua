//! Data collectors for gathering information from various sources.

pub mod base;

use crate::prelude::*;
use async_trait::async_trait;
use std::time::Duration;

pub use base::BaseCollector;

/// Core trait for all data collection sources
#[async_trait]
pub trait DataCollector: Send + Sync {
    /// Collect data from this source
    async fn collect(&self) -> Result<Vec<DataPoint>>;

    /// Get the source name for logging
    fn source_name(&self) -> &str;

    /// Get the reliability score for this source (0.0-1.0)
    fn reliability_score(&self) -> f64;

    /// Get the collection category
    fn category(&self) -> DataCategory;

    /// Health check with default implementation
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }

    /// Rate limit for this collector (requests per hour)
    fn rate_limit(&self) -> Option<u32> {
        None
    }

    /// Timeout duration for collection operations
    fn timeout(&self) -> Duration {
        crate::constants::DEFAULT_COLLECTION_TIMEOUT
    }
}

/// Aggregated data from multiple collectors
#[derive(Debug, Clone)]
pub struct AggregatedData {
    /// All collected data points
    pub data_points: Vec<DataPoint>,

    /// Collection timestamp
    pub collection_timestamp: DateTime<Utc>,

    /// Number of sources that successfully collected
    pub sources_count: usize,

    /// Sources that failed
    pub failed_sources: Vec<String>,

    /// Total collection duration
    pub collection_duration: Duration,
}

impl AggregatedData {
    /// Filter data points by category
    pub fn filter_by_category(&self, category: DataCategory) -> Vec<&DataPoint> {
        self.data_points
            .iter()
            .filter(|dp| dp.category == category)
            .collect()
    }

    /// Get average reliability score
    pub fn average_reliability(&self) -> f64 {
        if self.data_points.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.data_points.iter().map(|dp| dp.reliability).sum();
        sum / self.data_points.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregated_data_filtering() {
        let data = AggregatedData {
            data_points: vec![DataPoint::new(
                "Test".to_string(),
                "content".to_string(),
                DataCategory::NewsMedia,
            )],
            collection_timestamp: Utc::now(),
            sources_count: 1,
            failed_sources: Vec::new(),
            collection_duration: Duration::from_secs(1),
        };

        let filtered = data.filter_by_category(DataCategory::NewsMedia);
        assert_eq!(filtered.len(), 1);
    }
}
