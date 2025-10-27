//! Error types for the WarGames/JOSHUA system.
//!
//! This module provides comprehensive error handling with context and source tracking.

use std::fmt;

/// Result type alias for WarGames operations
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive error types for the WarGames/JOSHUA system
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Database operation failed
    #[error("Database operation failed: {operation}")]
    Database {
        operation: String,
        #[source]
        source: sqlx::Error,
    },

    /// Claude API error
    #[error("Claude API error: {message}")]
    ClaudeApi {
        message: String,
        status_code: Option<u16>,
    },

    /// Data collection failed
    #[error("Data collection failed for {collector}")]
    Collection {
        collector: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Notification delivery failed
    #[error("Notification delivery failed for {channel}")]
    Notification {
        channel: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Parsing error
    #[error("Parsing error: {0}")]
    Parsing(String),

    /// Visualization generation error
    #[error("Visualization error: {0}")]
    Visualization(String),

    /// Risk calculation error
    #[error("Risk calculation error: {0}")]
    RiskCalculation(String),

    /// Analysis error
    #[error("Analysis error: {0}")]
    Analysis(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// TOML deserialization error
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// HTTP request error
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded for {resource}")]
    RateLimit { resource: String },

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Generic error with context
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a new database error
    pub fn database(operation: impl Into<String>, source: sqlx::Error) -> Self {
        Error::Database {
            operation: operation.into(),
            source,
        }
    }

    /// Create a new Claude API error
    pub fn claude_api(message: impl Into<String>, status_code: Option<u16>) -> Self {
        Error::ClaudeApi {
            message: message.into(),
            status_code,
        }
    }

    /// Create a new collection error
    pub fn collection(
        collector: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Error::Collection {
            collector: collector.into(),
            source,
        }
    }

    /// Create a new notification error
    pub fn notification(
        channel: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Error::Notification {
            channel: channel.into(),
            source,
        }
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Error::Validation(message.into())
    }

    /// Create a new configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Error::Configuration(message.into())
    }

    /// Create a new not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Error::NotFound(message.into())
    }

    /// Create a new rate limit error
    pub fn rate_limit(resource: impl Into<String>) -> Self {
        Error::RateLimit {
            resource: resource.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Error::Timeout(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::validation("Invalid input");
        assert_eq!(err.to_string(), "Validation error: Invalid input");
    }

    #[test]
    fn test_error_construction() {
        let err = Error::not_found("Resource not found");
        assert!(matches!(err, Error::NotFound(_)));
    }
}
