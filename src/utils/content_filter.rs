//! Content filtering and relevance scoring.

use crate::constants::{GEOPOLITICAL_KEYWORDS, NUCLEAR_KEYWORDS};
use regex::RegexSet;

/// Content filter for identifying relevant data
pub struct ContentFilter {
    nuclear_patterns: RegexSet,
    geopolitical_patterns: RegexSet,
}

impl ContentFilter {
    /// Create a new content filter
    pub fn new() -> Self {
        // Create case-insensitive regex patterns for keywords
        let nuclear_patterns = RegexSet::new(
            NUCLEAR_KEYWORDS
                .iter()
                .map(|k| format!("(?i)\\b{}\\b", regex::escape(k))),
        )
        .expect("Failed to compile nuclear keyword patterns");

        let geopolitical_patterns = RegexSet::new(
            GEOPOLITICAL_KEYWORDS
                .iter()
                .map(|k| format!("(?i)\\b{}\\b", regex::escape(k))),
        )
        .expect("Failed to compile geopolitical keyword patterns");

        Self {
            nuclear_patterns,
            geopolitical_patterns,
        }
    }

    /// Check if content is relevant based on keywords
    pub fn is_relevant(&self, content: &str) -> bool {
        self.nuclear_patterns.is_match(content) || self.geopolitical_patterns.is_match(content)
    }

    /// Calculate relevance score (0.0 to 1.0)
    pub fn relevance_score(&self, content: &str) -> f64 {
        let nuclear_matches = self.nuclear_patterns.matches(content).into_iter().count();
        let geopolitical_matches = self
            .geopolitical_patterns
            .matches(content)
            .into_iter()
            .count();

        let total_matches = nuclear_matches + geopolitical_matches;

        if total_matches == 0 {
            return 0.0;
        }

        // Weight nuclear keywords more heavily
        let weighted_score = (nuclear_matches as f64 * 2.0) + (geopolitical_matches as f64);
        let max_possible = (NUCLEAR_KEYWORDS.len() * 2 + GEOPOLITICAL_KEYWORDS.len()) as f64;

        (weighted_score / max_possible).min(1.0)
    }

    /// Extract matched keywords from content
    pub fn extract_keywords(&self, content: &str) -> Vec<String> {
        let mut keywords = Vec::new();

        for (idx, _) in self
            .nuclear_patterns
            .matches(content)
            .into_iter()
            .enumerate()
        {
            keywords.push(NUCLEAR_KEYWORDS[idx].to_string());
        }

        for (idx, _) in self
            .geopolitical_patterns
            .matches(content)
            .into_iter()
            .enumerate()
        {
            keywords.push(GEOPOLITICAL_KEYWORDS[idx].to_string());
        }

        keywords
    }
}

impl Default for ContentFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_filter_relevant() {
        let filter = ContentFilter::new();

        let content = "North Korea conducted a nuclear test today with ICBM capabilities.";
        assert!(filter.is_relevant(content));

        let score = filter.relevance_score(content);
        assert!(score > 0.0);
    }

    #[test]
    fn test_content_filter_irrelevant() {
        let filter = ContentFilter::new();

        let content = "The weather is nice today and stocks are up.";
        assert!(!filter.is_relevant(content));

        let score = filter.relevance_score(content);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_extract_keywords() {
        let filter = ContentFilter::new();

        let content = "Russia's nuclear weapons and China military exercises near Taiwan.";
        let keywords = filter.extract_keywords(content);

        assert!(!keywords.is_empty());
        assert!(keywords.iter().any(|k| k.contains("nuclear")));
    }

    #[test]
    fn test_case_insensitive() {
        let filter = ContentFilter::new();

        let content1 = "NUCLEAR WEAPONS test";
        let content2 = "nuclear weapons test";

        assert!(filter.is_relevant(content1));
        assert!(filter.is_relevant(content2));
        assert_eq!(
            filter.relevance_score(content1),
            filter.relevance_score(content2)
        );
    }
}
