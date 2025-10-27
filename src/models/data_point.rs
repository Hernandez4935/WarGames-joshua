//! Data point model for collected information.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Raw data point collected from external sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// Unique identifier
    pub id: Uuid,

    /// Source name
    pub source: String,

    /// Source URL (if available)
    pub source_url: Option<String>,

    /// Title
    pub title: Option<String>,

    /// Content
    pub content: String,

    /// Published timestamp
    pub published_at: Option<DateTime<Utc>>,

    /// Collection timestamp
    pub collected_at: DateTime<Utc>,

    /// Data category
    pub category: DataCategory,

    /// Source reliability score (0.0 to 1.0)
    pub reliability: f64,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(source: String, content: String, category: DataCategory) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            source_url: None,
            title: None,
            content,
            published_at: None,
            collected_at: Utc::now(),
            category,
            reliability: 0.5,
            metadata: HashMap::new(),
        }
    }

    /// Set the source URL
    pub fn with_url(mut self, url: String) -> Self {
        self.source_url = Some(url);
        self
    }

    /// Set the title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set reliability score
    pub fn with_reliability(mut self, reliability: f64) -> Self {
        self.reliability = reliability.clamp(0.0, 1.0);
        self
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let dp = DataPoint::new(
            "Reuters".to_string(),
            "Test content".to_string(),
            DataCategory::NewsMedia,
        );
        assert_eq!(dp.source, "Reuters");
        assert_eq!(dp.category, DataCategory::NewsMedia);
    }

    #[test]
    fn test_data_point_builder() {
        let dp = DataPoint::new(
            "Test".to_string(),
            "Content".to_string(),
            DataCategory::NewsMedia,
        )
        .with_url("https://example.com".to_string())
        .with_title("Title".to_string())
        .with_reliability(0.9);

        assert_eq!(dp.reliability, 0.9);
        assert!(dp.source_url.is_some());
    }
}
