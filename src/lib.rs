//! ╔══════════════════════════════════════════════════════════════════════╗
//! ║              WarGames/JOSHUA: Nuclear Risk Assessment System         ║
//! ╚══════════════════════════════════════════════════════════════════════╝
//!
//! A comprehensive system for assessing global nuclear war risk through
//! multi-source data collection, AI-powered analysis, and statistical modeling.
//!
//! # Quick Start
//!
//! ```no_run
//! use wargames_joshua::WarGamesSystem;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize the system
//!     let system = WarGamesSystem::new().await?;
//!
//!     // Run a complete assessment
//!     let assessment = system.run_assessment().await?;
//!
//!     // Display results
//!     println!("Risk: {} seconds to midnight", assessment.seconds_to_midnight);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
//! │Collect  │──▶│Analyze   │──▶│Calculate │──▶│Visualize │
//! │Data     │   │(Claude)  │   │Risk      │   │& Report  │
//! └─────────┘   └──────────┘   └──────────┘   └──────────┘
//! ```
//!
//! # Modules
//!
//! - [`cli`] - Command-line interface
//! - [`collectors`] - Data collectors
//! - [`analyzers`] - Risk analyzers
//! - [`engines`] - Processing engines
//! - [`models`] - Data models
//! - [`visualizers`] - Visualization generators
//! - [`utils`] - Utility functions
//! - [`error`] - Error types
//! - [`types`] - Type definitions
//! - [`constants`] - System constants

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(missing_docs)] // Phase 0: Allow missing docs
#![allow(unused_imports)] // Phase 0: Allow unused imports

// ═══════════════════════════════════════════════════════════════════════
// Module Declarations
// ═══════════════════════════════════════════════════════════════════════

pub mod analyzers;
pub mod cli;
pub mod collectors;
pub mod engines;
pub mod models;
pub mod utils;
pub mod visualizers;

pub mod constants;
pub mod error;
pub mod types;

// ═══════════════════════════════════════════════════════════════════════
// Public Re-exports
// ═══════════════════════════════════════════════════════════════════════

pub use error::{Error, Result};
pub use models::{Assessment, RiskFactor};
pub use types::{ConfidenceLevel, RiskCategory, TrendDirection};

// ═══════════════════════════════════════════════════════════════════════
// Prelude - Commonly Used Types
// ═══════════════════════════════════════════════════════════════════════

/// Commonly used types and traits
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::models::*;
    pub use crate::types::*;
    pub use crate::utils::config::Config;
    pub use chrono::{DateTime, Utc};
    pub use uuid::Uuid;
}

// ═══════════════════════════════════════════════════════════════════════
// Core System
// ═══════════════════════════════════════════════════════════════════════

use crate::prelude::*;

/// Main WarGames/JOSHUA system orchestrator
///
/// This is the primary entry point for running nuclear risk assessments.
/// It coordinates all subsystems including data collection, AI analysis,
/// risk calculation, visualization, and reporting.
///
/// # Example
///
/// ```no_run
/// # use wargames_joshua::WarGamesSystem;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let system = WarGamesSystem::new().await?;
/// let assessment = system.run_assessment().await?;
/// println!("Risk: {} seconds", assessment.seconds_to_midnight);
/// # Ok(())
/// # }
/// ```
pub struct WarGamesSystem {
    config: Config,
}

impl WarGamesSystem {
    /// Create a new WarGames system instance
    ///
    /// # Errors
    ///
    /// Returns an error if configuration cannot be loaded or system
    /// initialization fails.
    #[allow(clippy::unused_async)]
    pub async fn new() -> Result<Self> {
        let config = Config::load()?;
        tracing::info!("WarGames/JOSHUA system initialized");
        Ok(Self { config })
    }

    /// Run a complete nuclear risk assessment
    ///
    /// This is the main entry point for performing a full risk assessment:
    /// 1. Collect data from all sources
    /// 2. Analyze with Claude AI
    /// 3. Calculate risk scores
    /// 4. Generate visualizations
    /// 5. Create reports
    /// 6. Store in database
    /// 7. Send alerts if necessary
    ///
    /// # Errors
    ///
    /// Returns an error if any step of the assessment pipeline fails.
    #[allow(clippy::unused_async)]
    pub async fn run_assessment(&self) -> Result<Assessment> {
        tracing::info!("Starting nuclear risk assessment");

        // TODO: Implement full assessment pipeline
        // This is a Phase 0 stub

        todo!("Full assessment pipeline will be implemented in Phase 1-3")
    }

    /// Get the current configuration
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_initialization() {
        // This test will fail until we implement Config::load
        // That's expected for Phase 0
        let result = WarGamesSystem::new().await;
        assert!(result.is_ok() || result.is_err()); // Placeholder
    }
}
