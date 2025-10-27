//! Base collector with common functionality.

use crate::collectors::{AggregatedData, DataCollector};
use crate::models::DataPoint;
use crate::prelude::*;
use crate::utils::{ContentFilter, DataQualityScorer, HttpClient, TimedCache};
use async_trait::async_trait;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;

/// Base collector providing common functionality
pub struct BaseCollector {
    pub http_client: HttpClient,
    pub cache: Arc<RwLock<TimedCache<String, Vec<DataPoint>>>>,
    pub content_filter: ContentFilter,
    pub quality_scorer: DataQualityScorer,
}

impl BaseCollector {
    /// Create a new base collector
    pub fn new() -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::new()?,
            cache: Arc::new(RwLock::new(TimedCache::new())),
            content_filter: ContentFilter::new(),
            quality_scorer: DataQualityScorer::new(),
        })
    }

    /// Check cache for data
    #[must_use]
    pub fn get_cached(&self, key: &str) -> Option<Vec<DataPoint>> {
        let cache = self.cache.read();
        cache.get(&key.to_string())
    }

    /// Store data in cache
    pub fn set_cache(&self, key: String, data: Vec<DataPoint>, ttl: Duration) {
        let cache = self.cache.write();
        cache.insert(key, data, ttl);
    }

    /// Filter data points by relevance and quality
    #[must_use]
    pub fn filter_and_score(&self, mut data_points: Vec<DataPoint>) -> Vec<DataPoint> {
        // Filter by relevance
        data_points.retain(|dp| self.content_filter.is_relevant(&dp.content));

        // Update reliability scores based on content relevance
        for dp in &mut data_points {
            let relevance = self.content_filter.relevance_score(&dp.content);
            dp.reliability = (dp.reliability + relevance) / 2.0;
        }

        // Filter by minimum quality
        self.quality_scorer
            .filter_by_quality(data_points, crate::constants::MIN_DATA_QUALITY_SCORE)
    }
}

impl Default for BaseCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create base collector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DataCategory;

    #[test]
    fn test_base_collector_creation() {
        let collector = BaseCollector::new();
        assert!(collector.is_ok());
    }

    #[test]
    fn test_cache_operations() {
        let collector = BaseCollector::new().unwrap();

        let data = vec![DataPoint::new(
            "Test".to_string(),
            "Nuclear test content".to_string(),
            DataCategory::NewsMedia,
        )];

        collector.set_cache(
            "test_key".to_string(),
            data.clone(),
            Duration::from_secs(60),
        );

        let cached = collector.get_cached("test_key");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
    }

    #[test]
    fn test_filter_and_score() {
        let collector = BaseCollector::new().unwrap();

        let data = vec![
            DataPoint::new(
                "Relevant".to_string(),
                "North Korea tests nuclear ICBM missile".to_string(),
                DataCategory::NewsMedia,
            )
            .with_reliability(0.8),
            DataPoint::new(
                "Irrelevant".to_string(),
                "Weather is sunny today".to_string(),
                DataCategory::NewsMedia,
            )
            .with_reliability(0.8),
        ];

        let filtered = collector.filter_and_score(data);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].source, "Relevant");
    }
}
