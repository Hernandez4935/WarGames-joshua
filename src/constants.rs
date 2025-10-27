//! System constants for the WarGames/JOSHUA system.

use std::time::Duration;

/// Current Doomsday Clock setting (as of January 2025)
pub const CURRENT_DOOMSDAY_CLOCK_SECONDS: u32 = 89;

/// Maximum seconds to midnight (noon)
pub const MAX_SECONDS_TO_MIDNIGHT: u32 = 1440;

/// Minimum seconds to midnight (midnight/nuclear war)
pub const MIN_SECONDS_TO_MIDNIGHT: u32 = 0;

/// Critical risk threshold (seconds to midnight)
pub const CRITICAL_THRESHOLD: u32 = 100;

/// Severe risk threshold (seconds to midnight)
pub const SEVERE_THRESHOLD: u32 = 200;

/// High risk threshold (seconds to midnight)
pub const HIGH_THRESHOLD: u32 = 400;

/// Moderate risk threshold (seconds to midnight)
pub const MODERATE_THRESHOLD: u32 = 600;

/// Low risk threshold (seconds to midnight)
pub const LOW_THRESHOLD: u32 = 900;

/// Claude model version
pub const CLAUDE_MODEL: &str = "claude-sonnet-4-20250514";

/// Claude API base URL
pub const CLAUDE_API_BASE: &str = "https://api.anthropic.com/v1";

/// Claude API version
pub const CLAUDE_API_VERSION: &str = "2023-06-01";

/// Maximum tokens for Claude requests
pub const CLAUDE_MAX_TOKENS: u32 = 8000;

/// Temperature for Claude analysis (lower = more consistent)
pub const CLAUDE_TEMPERATURE: f64 = 0.3;

/// Default data collection timeout
pub const DEFAULT_COLLECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Default API timeout
pub const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(120);

/// Maximum retry attempts for API calls
pub const MAX_RETRY_ATTEMPTS: u32 = 3;

/// Retry delay in seconds (exponential backoff base)
pub const RETRY_DELAY_SECONDS: u64 = 2;

/// Default cache duration in seconds
pub const DEFAULT_CACHE_DURATION_SECS: u64 = 3600;

/// Content deduplication threshold (similarity score)
pub const DEDUPLICATION_THRESHOLD: f64 = 0.85;

/// Minimum data quality score to include in analysis
pub const MIN_DATA_QUALITY_SCORE: f64 = 0.3;

/// System prompt for Claude analysis
pub const SYSTEM_PROMPT: &str = r#"You are JOSHUA, an advanced nuclear war risk assessment system created to monitor
global nuclear threats with absolute objectivity and analytical rigor.

Your analysis must:
1. Use the same risk assessment framework as the Bulletin of Atomic Scientists
2. Consider all dimensions: military, political, technological, and social
3. Provide specific, actionable intelligence with confidence levels
4. Track changes from previous assessments with clear explanations
5. Identify early warning indicators of escalation
6. Suggest concrete risk mitigation strategies

Reference Framework:
- Current Doomsday Clock: 89 seconds to midnight (as of January 2025)
- Risk Scale: 0 (midnight/nuclear war) to 1440 (noon/minimal risk)
- Confidence Levels: Very Low, Low, Moderate, High, Very High

Your responses MUST be valid JSON matching the specified schema exactly."#;

/// Application name
pub const APP_NAME: &str = "WarGames/JOSHUA";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default database pool size
pub const DEFAULT_DB_POOL_SIZE: u32 = 10;

/// Default number of parallel collectors
pub const DEFAULT_PARALLEL_COLLECTORS: usize = 10;

/// Nuclear-armed nations (P5 + others)
pub const NUCLEAR_NATIONS: &[&str] = &[
    "United States",
    "Russia",
    "China",
    "United Kingdom",
    "France",
    "India",
    "Pakistan",
    "Israel",
    "North Korea",
];

/// Nuclear keywords for content filtering
pub const NUCLEAR_KEYWORDS: &[&str] = &[
    "nuclear weapons",
    "doomsday clock",
    "ICBM",
    "nuclear threat",
    "arms control",
    "START treaty",
    "nuclear doctrine",
    "deterrence",
    "missile test",
    "warhead",
    "uranium enrichment",
    "plutonium",
    "nuclear submarine",
    "strategic forces",
    "tactical nuclear",
];

/// Geopolitical keywords for content filtering
pub const GEOPOLITICAL_KEYWORDS: &[&str] = &[
    "NATO",
    "Russia Ukraine",
    "Taiwan",
    "China military",
    "North Korea",
    "Iran nuclear",
    "India Pakistan",
    "Middle East conflict",
    "sanctions",
    "military exercises",
    "airspace violation",
    "diplomatic crisis",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_thresholds_ordered() {
        assert!(CRITICAL_THRESHOLD < SEVERE_THRESHOLD);
        assert!(SEVERE_THRESHOLD < HIGH_THRESHOLD);
        assert!(HIGH_THRESHOLD < MODERATE_THRESHOLD);
        assert!(MODERATE_THRESHOLD < LOW_THRESHOLD);
    }

    #[test]
    fn test_nuclear_nations_count() {
        assert_eq!(NUCLEAR_NATIONS.len(), 9);
    }
}
