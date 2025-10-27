//! Content deduplication using SHA-256 hashing.

use crate::models::DataPoint;
use crate::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// Content deduplicator using SHA-256 hashing
pub struct ContentDeduplicator {
    #[allow(dead_code)]
    similarity_threshold: f64,
}

impl ContentDeduplicator {
    /// Create a new deduplicator
    pub fn new(similarity_threshold: f64) -> Self {
        Self {
            similarity_threshold,
        }
    }

    /// Deduplicate a list of data points
    pub fn deduplicate(&self, data_points: Vec<DataPoint>) -> Result<Vec<DataPoint>> {
        let mut unique_points = Vec::new();
        let mut seen_hashes = HashSet::new();

        for point in data_points {
            let hash = self.hash_content(&point.content);

            if !seen_hashes.contains(&hash) {
                seen_hashes.insert(hash);
                unique_points.push(point);
            }
        }

        Ok(unique_points)
    }

    /// Hash content using SHA-256
    pub fn hash_content(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Check if two content strings are similar
    pub fn is_similar(&self, content1: &str, content2: &str) -> bool {
        let hash1 = self.hash_content(content1);
        let hash2 = self.hash_content(content2);

        // For exact matching, compare hashes
        // For fuzzy matching, you'd implement Simhash or similar
        hash1 == hash2
    }

    /// Deduplicate by URL if available
    pub fn deduplicate_by_url(&self, data_points: Vec<DataPoint>) -> Result<Vec<DataPoint>> {
        let mut unique_points = Vec::new();
        let mut seen_urls = HashSet::new();

        for point in data_points {
            if let Some(url) = &point.source_url {
                if !seen_urls.contains(url) {
                    seen_urls.insert(url.clone());
                    unique_points.push(point);
                }
            } else {
                // No URL, use content hash
                let hash = self.hash_content(&point.content);
                if !seen_urls.contains(&hash) {
                    seen_urls.insert(hash);
                    unique_points.push(point);
                }
            }
        }

        Ok(unique_points)
    }
}

impl Default for ContentDeduplicator {
    fn default() -> Self {
        Self::new(crate::constants::DEDUPLICATION_THRESHOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DataCategory;

    #[test]
    fn test_hash_content() {
        let dedup = ContentDeduplicator::default();

        let content1 = "Test content";
        let content2 = "Test content";
        let content3 = "Different content";

        let hash1 = dedup.hash_content(content1);
        let hash2 = dedup.hash_content(content2);
        let hash3 = dedup.hash_content(content3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_deduplicate() {
        let dedup = ContentDeduplicator::default();

        let points = vec![
            DataPoint::new(
                "Source1".to_string(),
                "Content1".to_string(),
                DataCategory::NewsMedia,
            ),
            DataPoint::new(
                "Source2".to_string(),
                "Content1".to_string(),
                DataCategory::NewsMedia,
            ),
            DataPoint::new(
                "Source3".to_string(),
                "Content2".to_string(),
                DataCategory::NewsMedia,
            ),
        ];

        let unique = dedup.deduplicate(points).unwrap();
        assert_eq!(unique.len(), 2);
    }

    #[test]
    fn test_deduplicate_by_url() {
        let dedup = ContentDeduplicator::default();

        let points = vec![
            DataPoint::new(
                "Source1".to_string(),
                "Content1".to_string(),
                DataCategory::NewsMedia,
            )
            .with_url("http://example.com/1".to_string()),
            DataPoint::new(
                "Source2".to_string(),
                "Content1".to_string(),
                DataCategory::NewsMedia,
            )
            .with_url("http://example.com/1".to_string()),
            DataPoint::new(
                "Source3".to_string(),
                "Content2".to_string(),
                DataCategory::NewsMedia,
            )
            .with_url("http://example.com/2".to_string()),
        ];

        let unique = dedup.deduplicate_by_url(points).unwrap();
        assert_eq!(unique.len(), 2);
    }
}
