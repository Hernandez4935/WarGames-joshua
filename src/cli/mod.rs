//! Command-line interface for WarGames/JOSHUA.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// WarGames/JOSHUA: Global Thermonuclear War Risk Assessment System
#[derive(Parser)]
#[clap(name = "joshua")]
#[clap(author = "WarGames/JOSHUA Development Team")]
#[clap(version)]
#[clap(about = "Global Thermonuclear War Risk Assessment System", long_about = None)]
pub struct Cli {
    /// Subcommand to execute
    #[clap(subcommand)]
    pub command: Commands,

    /// Verbosity level (-v, -vv, -vvv)
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Configuration file path
    #[clap(short, long, default_value = "config/default_config.toml")]
    pub config: PathBuf,
}

/// Available commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a new risk assessment
    Assess {
        /// Force fresh data collection (bypass cache)
        #[clap(short, long)]
        force: bool,

        /// Output format (markdown, html, json)
        #[clap(short, long, default_value = "markdown")]
        output: String,

        /// Include interactive terminal UI
        #[clap(short, long)]
        interactive: bool,
    },

    /// View historical assessments
    History {
        /// Number of past assessments to show
        #[clap(short = 'n', long, default_value = "10")]
        count: usize,

        /// Start date for history (YYYY-MM-DD)
        #[clap(long)]
        from: Option<String>,

        /// End date for history (YYYY-MM-DD)
        #[clap(long)]
        to: Option<String>,
    },

    /// Generate trend analysis
    Trends {
        /// Time period (daily, weekly, monthly, yearly)
        #[clap(short, long, default_value = "monthly")]
        period: String,

        /// Specific risk factors to analyze
        #[clap(short, long)]
        factors: Vec<String>,
    },

    /// Simulate scenarios
    Simulate {
        /// Scenario file or preset name
        #[clap(short, long)]
        scenario: String,

        /// Number of Monte Carlo iterations
        #[clap(short = 'n', long, default_value = "1000")]
        iterations: usize,
    },

    /// Interactive terminal mode (`WarGames` style)
    Interactive,

    /// System diagnostics and health check
    Diagnose,

    /// Initialize database with schema
    InitDb {
        /// Database connection string
        #[clap(short, long)]
        connection: Option<String>,
    },
}

impl Cli {
    /// Parse CLI arguments
    #[must_use]
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
