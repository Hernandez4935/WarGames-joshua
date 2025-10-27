//! Configuration management.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// General settings
    pub general: GeneralConfig,

    /// Claude API settings
    pub claude_api: ClaudeApiConfig,

    /// Data collection settings
    pub data_collection: DataCollectionConfig,

    /// Risk calculation settings
    pub risk_calculation: RiskCalculationConfig,

    /// Database settings
    pub database: DatabaseConfig,

    /// Notification settings
    pub notifications: NotificationConfig,

    /// Logging settings
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub app_name: String,
    pub version: String,
    pub default_assessment_interval: String,
    pub data_retention_months: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeApiConfig {
    pub api_key_env: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollectionConfig {
    pub parallel_collectors: usize,
    pub timeout_per_source: u64,
    pub cache_duration_hours: u64,
    pub deduplication_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCalculationConfig {
    pub weights: RiskWeights,
    pub thresholds: RiskThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskWeights {
    pub arsenal_changes: f64,
    pub doctrine_changes: f64,
    pub regional_conflicts: f64,
    pub leadership_rhetoric: f64,
    pub technical_incidents: f64,
    pub communication_breakdown: f64,
    pub emerging_technology: f64,
    pub economic_factors: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskThresholds {
    pub critical: u32,
    pub severe: u32,
    pub high: u32,
    pub moderate: u32,
    pub low: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: String,
    pub connection_string: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enable_webhook: bool,
    pub webhook_url: Option<String>,
    pub alert_on_critical: bool,
    pub alert_on_trend_change: bool,
    pub minimum_change_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        Self::load_from_path("config/default_config.toml")
    }

    /// Load configuration from specific path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // If file doesn't exist, return default config
        if !path.exists() {
            tracing::warn!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Get API key from environment
    pub fn get_api_key(&self) -> Result<String> {
        std::env::var(&self.claude_api.api_key_env).map_err(|_| {
            Error::Configuration(format!(
                "Environment variable {} not set",
                self.claude_api.api_key_env
            ))
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                app_name: "WarGames/JOSHUA".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                default_assessment_interval: "monthly".to_string(),
                data_retention_months: 60,
            },
            claude_api: ClaudeApiConfig {
                api_key_env: "ANTHROPIC_API_KEY".to_string(),
                model: crate::constants::CLAUDE_MODEL.to_string(),
                max_tokens: crate::constants::CLAUDE_MAX_TOKENS,
                temperature: crate::constants::CLAUDE_TEMPERATURE,
                timeout_seconds: crate::constants::DEFAULT_API_TIMEOUT.as_secs(),
                max_retries: crate::constants::MAX_RETRY_ATTEMPTS,
            },
            data_collection: DataCollectionConfig {
                parallel_collectors: crate::constants::DEFAULT_PARALLEL_COLLECTORS,
                timeout_per_source: crate::constants::DEFAULT_COLLECTION_TIMEOUT.as_secs(),
                cache_duration_hours: crate::constants::DEFAULT_CACHE_DURATION_SECS / 3600,
                deduplication_threshold: crate::constants::DEDUPLICATION_THRESHOLD,
            },
            risk_calculation: RiskCalculationConfig {
                weights: RiskWeights {
                    arsenal_changes: 0.15,
                    doctrine_changes: 0.15,
                    regional_conflicts: 0.20,
                    leadership_rhetoric: 0.10,
                    technical_incidents: 0.15,
                    communication_breakdown: 0.10,
                    emerging_technology: 0.10,
                    economic_factors: 0.05,
                },
                thresholds: RiskThresholds {
                    critical: crate::constants::CRITICAL_THRESHOLD,
                    severe: crate::constants::SEVERE_THRESHOLD,
                    high: crate::constants::HIGH_THRESHOLD,
                    moderate: crate::constants::MODERATE_THRESHOLD,
                    low: crate::constants::LOW_THRESHOLD,
                },
            },
            database: DatabaseConfig {
                db_type: "postgresql".to_string(),
                connection_string: "postgresql://user:pass@localhost:5432/wargames".to_string(),
                pool_size: crate::constants::DEFAULT_DB_POOL_SIZE,
            },
            notifications: NotificationConfig {
                enable_webhook: false,
                webhook_url: None,
                alert_on_critical: true,
                alert_on_trend_change: true,
                minimum_change_percentage: 10.0,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "text".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.app_name, "WarGames/JOSHUA");
    }

    #[test]
    fn test_risk_weights_sum_to_one() {
        let config = Config::default();
        let sum = config.risk_calculation.weights.arsenal_changes
            + config.risk_calculation.weights.doctrine_changes
            + config.risk_calculation.weights.regional_conflicts
            + config.risk_calculation.weights.leadership_rhetoric
            + config.risk_calculation.weights.technical_incidents
            + config.risk_calculation.weights.communication_breakdown
            + config.risk_calculation.weights.emerging_technology
            + config.risk_calculation.weights.economic_factors;

        assert!((sum - 1.0).abs() < 0.001);
    }
}
