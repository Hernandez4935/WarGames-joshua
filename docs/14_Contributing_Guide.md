# WarGames/JOSHUA: Technical Contributing Guide
## Deep-Dive Developer Contribution Guide
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Development Environment Setup](#2-development-environment-setup)
3. [Architecture Deep Dive](#3-architecture-deep-dive)
4. [Code Style and Standards](#4-code-style-and-standards)
5. [Implementing New Features](#5-implementing-new-features)
6. [Testing Guidelines](#6-testing-guidelines)
7. [Documentation Requirements](#7-documentation-requirements)
8. [Pull Request Process](#8-pull-request-process)
9. [Phase-Based Development](#9-phase-based-development)
10. [Security Considerations](#10-security-considerations)
11. [Performance Guidelines](#11-performance-guidelines)
12. [Release Process](#12-release-process)
13. [Getting Help](#13-getting-help)

---

## 1. Introduction

### 1.1 Project Mission and Goals

WarGames/JOSHUA aims to provide **transparent, data-driven nuclear war risk assessment** through:

- **Continuous monitoring** of global nuclear threats
- **AI-powered analysis** using state-of-the-art language models
- **Statistical rigor** through Bayesian networks and Monte Carlo simulation
- **Open methodology** with reproducible, auditable assessments

**Why contribute?**

Nuclear risk assessment is traditionally opaque, conducted by closed expert panels. By contributing to an open-source, scientifically rigorous system, you help democratize nuclear risk awareness and potentially influence policy decisions that affect global security.

### 1.2 Types of Contributions

**Code Contributions:**
- Data collector implementations (new sources)
- Risk analysis algorithms and improvements
- Visualization enhancements
- Performance optimizations
- Bug fixes and reliability improvements

**Documentation Contributions:**
- User guides and tutorials
- API documentation
- Code examples and recipes
- Architecture explanations
- Translation (internationalization)

**Research Contributions:**
- Risk modeling methodologies
- Historical validation studies
- Comparative analysis with expert assessments
- Statistical improvements

**Infrastructure Contributions:**
- CI/CD pipeline enhancements
- Deployment automation
- Monitoring and observability
- Security hardening

**Community Contributions:**
- Issue triage and support
- Code reviews
- Community management
- Outreach and adoption

### 1.3 High-Level vs. This Document

**Note:** The root [`CONTRIBUTING.md`](../CONTRIBUTING.md) provides high-level contribution guidelines suitable for all contributors. **This document** is a **technical deep-dive** focused on:

- Internal architecture and design patterns
- Implementation details and code organization
- Advanced testing strategies
- Performance optimization techniques
- Complex feature development workflows

If you're new to the project, start with the root `CONTRIBUTING.md`, then return here for deeper technical guidance.

---

## 2. Development Environment Setup

### 2.1 Prerequisites

**Required:**
- **Rust 1.75+** with `rustup`, `cargo`, `rustfmt`, `clippy`
- **PostgreSQL 14+** (or SQLite for lightweight development)
- **Git** for version control
- **Anthropic API key** for Claude integration (Phase 2+)

**Recommended:**
- **Redis** for caching (optional but improves performance)
- **Docker** for containerized development
- **Just** command runner for build automation
- **cargo-watch** for live reloading during development

**IDE Setup:**
- **VS Code** with rust-analyzer, CodeLLDB
- **IntelliJ IDEA** with Rust plugin
- **Neovim/Vim** with rust.vim, coc-rust-analyzer

### 2.2 Detailed Setup Instructions

#### Step 1: Install Rust and Tools

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Verify installation
rustc --version  # Should be 1.75+
cargo --version

# Install components
rustup component add rustfmt clippy rust-analyzer

# Install development tools
cargo install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated just
```

#### Step 2: Install PostgreSQL

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib libpq-dev

# Start service
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Create database and user
sudo -u postgres psql
```

```sql
CREATE DATABASE wargames_joshua;
CREATE USER joshua WITH ENCRYPTED PASSWORD 'joshua_dev_password';
GRANT ALL PRIVILEGES ON DATABASE wargames_joshua TO joshua;
\q
```

**macOS:**
```bash
brew install postgresql libpq

# Start service
brew services start postgresql

# Create database
createdb wargames_joshua
```

**Windows:**
```powershell
# Download and install from https://www.postgresql.org/download/windows/
# Or use Docker (recommended)
docker run -d \
  --name postgres-dev \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=wargames_joshua \
  -p 5432:5432 \
  postgres:15
```

#### Step 3: Clone and Build

```bash
# Clone repository
git clone https://github.com/yourusername/wargames-joshua.git
cd wargames-joshua

# Create development configuration
cp config/default_config.toml config/dev_config.toml

# Edit dev_config.toml with your settings
nano config/dev_config.toml
```

**Development configuration (`config/dev_config.toml`):**

```toml
[general]
log_level = "debug"  # More verbose for development
data_directory = "./dev-data"

[database]
connection_string = "postgresql://joshua:joshua_dev_password@localhost:5432/wargames_joshua"
pool_size = 5  # Smaller for development

[claude]
api_key = "sk-ant-api03-..."  # Your development API key
model = "claude-sonnet-4-20250514"
cache_responses = true  # Save API costs during development
cache_ttl = 86400  # 24 hours for development

[data_collection]
# Use fewer sources during development
enabled_sources = ["reuters", "sipri", "bulletin"]
collection_timeout = 60  # Shorter timeout for faster iteration

[notifications]
enabled = false  # Disable during development
```

**Build the project:**

```bash
# Debug build (faster compilation)
cargo build

# Run tests
cargo test

# Run with development config
cargo run -- --config config/dev_config.toml diagnose

# Initialize database
cargo run -- --config config/dev_config.toml init-db
```

#### Step 4: Development Workflow Setup

**Using `cargo-watch` for live reloading:**

```bash
# Watch for changes and rebuild
cargo watch -x build

# Watch, build, and run tests
cargo watch -x test

# Watch and run specific binary
cargo watch -x 'run -- assess --dry-run'
```

**Using `just` for common tasks:**

Create `justfile` in project root:

```makefile
# Justfile for WarGames/JOSHUA development

# Default recipe
default:
    @just --list

# Build debug binary
build:
    cargo build

# Build release binary
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --check

# Run full CI checks locally
ci: fmt-check lint test
    @echo "✓ All CI checks passed"

# Generate code coverage
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Run security audit
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Initialize development database
init-db:
    cargo run -- --config config/dev_config.toml init-db

# Run development server
dev:
    cargo watch -x 'run -- --config config/dev_config.toml interactive'

# Run assessment
assess:
    cargo run -- --config config/dev_config.toml assess --interactive

# Clean build artifacts
clean:
    cargo clean

# Build and install locally
install:
    cargo install --path .
```

**Usage:**

```bash
# List available recipes
just

# Run tests
just test

# Full CI check before push
just ci

# Development server with auto-reload
just dev
```

---

## 3. Architecture Deep Dive

### 3.1 Module Organization

The codebase follows a clear modular structure:

```
wargames-joshua/
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Library root, public API
│   │
│   ├── cli/                     # Command-line interface
│   │   ├── mod.rs               # CLI module root
│   │   ├── commands/            # Command implementations
│   │   │   ├── assess.rs        # Assessment command
│   │   │   ├── history.rs       # History command
│   │   │   ├── trends.rs        # Trends command
│   │   │   ├── simulate.rs      # Simulation command
│   │   │   └── interactive.rs   # TUI mode
│   │   └── args.rs              # Argument parsing (clap)
│   │
│   ├── engines/                 # Core processing engines
│   │   ├── mod.rs
│   │   ├── data_collection.rs   # Data collection orchestration
│   │   ├── claude_integration.rs# Claude API integration
│   │   ├── risk_calculation.rs  # Risk scoring and calculation
│   │   ├── visualization.rs     # Chart and graph generation
│   │   ├── report_generation.rs # Report creation (MD/HTML/PDF)
│   │   ├── notification.rs      # Alert notifications
│   │   ├── database.rs          # Database operations
│   │   └── scheduling.rs        # Automated scheduling
│   │
│   ├── collectors/              # Data source collectors
│   │   ├── mod.rs
│   │   ├── traits.rs            # DataCollector trait
│   │   ├── news/                # News source collectors
│   │   │   ├── reuters.rs
│   │   │   ├── ap.rs
│   │   │   └── bbc.rs
│   │   ├── research/            # Research institution collectors
│   │   │   ├── sipri.rs
│   │   │   ├── bulletin.rs
│   │   │   └── carnegie.rs
│   │   ├── government/          # Government source collectors
│   │   │   ├── state_dept.rs
│   │   │   └── iaea.rs
│   │   └── social/              # Social media collectors
│   │       └── twitter.rs
│   │
│   ├── analyzers/               # Risk analysis components
│   │   ├── mod.rs
│   │   ├── traits.rs            # RiskAnalyzer trait
│   │   ├── arsenal.rs           # Arsenal change analyzer
│   │   ├── doctrine.rs          # Doctrine/posture analyzer
│   │   ├── conflicts.rs         # Regional conflict analyzer
│   │   ├── leadership.rs        # Leadership/rhetoric analyzer
│   │   ├── technical.rs         # Technical incident analyzer
│   │   ├── communication.rs     # Communication breakdown analyzer
│   │   ├── technology.rs        # Emerging tech analyzer
│   │   └── economic.rs          # Economic factor analyzer
│   │
│   ├── visualizers/             # Visualization generators
│   │   ├── mod.rs
│   │   ├── traits.rs            # Visualizer trait
│   │   ├── doomsday_clock.rs    # Clock diagram
│   │   ├── trend_chart.rs       # Time-series charts
│   │   ├── heatmap.rs           # Risk factor heatmap
│   │   └── category_breakdown.rs# Category composition
│   │
│   ├── models/                  # Data models and types
│   │   ├── mod.rs
│   │   ├── assessment.rs        # RiskAssessment struct
│   │   ├── risk_factor.rs       # RiskFactor struct
│   │   ├── data_point.rs        # DataPoint struct
│   │   ├── config.rs            # Configuration types
│   │   └── enums.rs             # Shared enums
│   │
│   ├── utils/                   # Utilities and helpers
│   │   ├── mod.rs
│   │   ├── config.rs            # Configuration loading
│   │   ├── cache.rs             # Caching utilities
│   │   ├── deduplication.rs     # Content deduplication
│   │   ├── validation.rs        # Input validation
│   │   └── time.rs              # Time/date utilities
│   │
│   ├── error.rs                 # Error types (thiserror)
│   ├── types.rs                 # Type aliases
│   └── constants.rs             # System constants
│
├── tests/                       # Integration tests
│   ├── integration/
│   │   ├── assessment_flow.rs
│   │   ├── data_collection.rs
│   │   └── database_ops.rs
│   └── fixtures/                # Test data
│
├── benches/                     # Benchmarks (criterion)
│   ├── risk_calculation.rs
│   └── data_processing.rs
│
├── migrations/                  # Database migrations (SQLx)
│   ├── 20250101_initial_schema.sql
│   └── ...
│
└── config/                      # Configuration files
    ├── default_config.toml
    └── dev_config.toml
```

### 3.2 Core Traits and Interfaces

#### DataCollector Trait

All data sources must implement:

```rust
/// Core trait for data collection from external sources
#[async_trait]
pub trait DataCollector: Send + Sync {
    /// Collect data from this source
    async fn collect(&self) -> Result<Vec<DataPoint>>;

    /// Source identifier for logging and attribution
    fn source_name(&self) -> &str;

    /// Reliability score (0.0-1.0)
    fn reliability_score(&self) -> f64;

    /// Data category classification
    fn category(&self) -> DataCategory;

    /// Optional health check (default: always healthy)
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }

    /// Rate limit (requests per hour)
    fn rate_limit(&self) -> Option<u32> {
        None
    }

    /// Collection timeout
    fn timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}
```

**Implementation pattern:**

```rust
pub struct NewSourceCollector {
    http_client: reqwest::Client,
    cache: Arc<RwLock<TimedCache<String, Vec<DataPoint>>>>,
    config: SourceConfig,
}

impl NewSourceCollector {
    pub fn new(config: SourceConfig) -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            cache: Arc::new(RwLock::new(TimedCache::new())),
            config,
        }
    }
}

#[async_trait]
impl DataCollector for NewSourceCollector {
    async fn collect(&self) -> Result<Vec<DataPoint>> {
        // 1. Check cache
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&self.config.cache_key) {
                return Ok(cached.clone());
            }
        }

        // 2. Fetch data
        let response = self.http_client
            .get(&self.config.endpoint_url)
            .send()
            .await
            .map_err(|e| Error::Collection {
                collector: self.source_name().to_string(),
                source: Box::new(e),
            })?;

        // 3. Parse data
        let raw_data = response.json::<RawDataFormat>().await?;

        // 4. Transform to DataPoints
        let data_points = raw_data.items
            .into_iter()
            .map(|item| self.transform_item(item))
            .collect::<Result<Vec<_>>>()?;

        // 5. Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(
                self.config.cache_key.clone(),
                data_points.clone(),
                Duration::from_secs(3600),
            );
        }

        Ok(data_points)
    }

    fn source_name(&self) -> &str {
        "New Source Name"
    }

    fn reliability_score(&self) -> f64 {
        0.80  // Adjust based on source credibility
    }

    fn category(&self) -> DataCategory {
        DataCategory::NewsMedia  // Or appropriate category
    }

    fn rate_limit(&self) -> Option<u32> {
        Some(100)  // 100 requests per hour
    }
}

impl NewSourceCollector {
    fn transform_item(&self, item: RawItem) -> Result<DataPoint> {
        Ok(DataPoint {
            id: Uuid::new_v4(),
            source: self.source_name().to_string(),
            source_url: Some(item.url),
            title: Some(item.title),
            content: item.content,
            published_at: item.published_date.map(|d| d.with_timezone(&Utc)),
            collected_at: Utc::now(),
            category: self.category(),
            reliability: self.reliability_score(),
            metadata: HashMap::new(),
        })
    }
}
```

#### RiskAnalyzer Trait

```rust
/// Trait for risk analysis components
#[async_trait]
pub trait RiskAnalyzer: Send + Sync {
    /// Analyze aggregated data and produce risk assessment
    async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis>;

    /// Risk category this analyzer handles
    fn risk_category(&self) -> RiskCategory;

    /// Weight of this analyzer in overall calculation
    fn weight(&self) -> f64;

    /// Minimum confidence threshold
    fn min_confidence(&self) -> ConfidenceLevel {
        ConfidenceLevel::Low
    }

    /// Name of this analyzer
    fn name(&self) -> &str;
}
```

**Implementation pattern:**

```rust
pub struct NewRiskAnalyzer {
    config: AnalyzerConfig,
    historical_db: Arc<dyn DatabaseEngine>,
}

#[async_trait]
impl RiskAnalyzer for NewRiskAnalyzer {
    async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis> {
        // 1. Filter relevant data
        let relevant_data = data.filter_by_category(self.relevant_categories());

        // 2. Extract risk indicators
        let indicators = self.extract_indicators(&relevant_data)?;

        // 3. Compare with historical baseline
        let baseline = self.historical_db
            .get_baseline_for_category(self.risk_category())
            .await?;

        // 4. Calculate risk factors
        let mut risk_factors = Vec::new();
        for indicator in indicators {
            let risk_value = self.calculate_indicator_risk(&indicator, &baseline);
            let confidence = self.assess_confidence(&indicator);

            risk_factors.push(RiskFactor {
                category: self.risk_category(),
                name: indicator.name.clone(),
                value: risk_value,
                confidence,
                data_sources: indicator.sources.clone(),
                timestamp: Utc::now(),
                trend: indicator.trend,
            });
        }

        // 5. Aggregate into analysis
        let overall_score = self.aggregate_risk_factors(&risk_factors);
        let confidence = self.calculate_overall_confidence(&risk_factors);

        Ok(RiskAnalysis {
            category: self.risk_category(),
            overall_score,
            confidence,
            risk_factors,
            summary: self.generate_summary(&risk_factors),
            recommendations: self.generate_recommendations(&risk_factors),
        })
    }

    fn risk_category(&self) -> RiskCategory {
        RiskCategory::YourNewCategory
    }

    fn weight(&self) -> f64 {
        0.10  // 10% of overall risk
    }

    fn name(&self) -> &str {
        "New Risk Analyzer"
    }
}
```

#### Visualizer Trait

```rust
/// Trait for visualization generation
pub trait Visualizer: Send + Sync {
    /// Generate visualization from assessment
    fn visualize(&self, assessment: &RiskAssessment) -> Result<Visualization>;

    /// Visualization name
    fn name(&self) -> &str;

    /// Supported output formats
    fn supported_formats(&self) -> &[VisualizationFormat];

    /// Default format
    fn default_format(&self) -> VisualizationFormat {
        VisualizationFormat::Svg
    }
}
```

### 3.3 Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        COMPLETE DATA FLOW                               │
└─────────────────────────────────────────────────────────────────────────┘

User Command: joshua assess
         │
         ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ CLI Layer (src/cli/)                                                    │
│  • Parse arguments with clap                                            │
│  • Load configuration                                                   │
│  • Initialize WarGamesSystem                                            │
└─────────────────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Orchestration Layer (src/engines/data_collection.rs)                   │
│  • Spawn parallel collector tasks                                      │
│  • Apply rate limiting                                                 │
│  • Handle retries and failures                                         │
│  • Deduplicate collected data                                          │
└─────────────────────────────────────────────────────────────────────────┘
         │
         ├──────────┬──────────┬──────────┬──────────┐
         ▼          ▼          ▼          ▼          ▼
    ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
    │Reuters │ │ SIPRI  │ │Bulletin│ │ State  │ │Carnegie│
    │Collect.│ │Collect.│ │Collect.│ │ Dept   │ │Collect.│
    └────────┘ └────────┘ └────────┘ └────────┘ └────────┘
         │          │          │          │          │
         └──────────┴──────────┴──────────┴──────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Aggregated Data                                                         │
│  • 150+ DataPoints from multiple sources                               │
│  • Deduplicated and quality-filtered                                   │
│  • Timestamped and categorized                                         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ AI Analysis Layer (src/engines/claude_integration.rs)                  │
│  • Build analysis prompt from data                                     │
│  • Call Claude API (with retry logic)                                  │
│  • Parse structured JSON response                                      │
│  • Extract risk factors and confidence                                 │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Claude Analysis Result                                                  │
│  • Risk factors identified and scored                                  │
│  • Confidence levels assigned                                          │
│  • Evidence and reasoning provided                                     │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Risk Calculation Layer (src/engines/risk_calculation.rs)               │
│  • Apply category weights                                              │
│  • Bayesian network adjustment                                         │
│  • Monte Carlo simulation (10,000 iterations)                          │
│  • Convert to seconds-to-midnight scale                                │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Risk Assessment Complete                                                │
│  • Final risk score (seconds to midnight)                              │
│  • Confidence level                                                    │
│  • Trend direction and magnitude                                       │
│  • Contributing factors ranked                                         │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
┌─────────────────────────────┐ ┌─────────────────────────────┐
│ Visualization Engine        │ │ Report Generation Engine    │
│ • Doomsday Clock diagram    │ │ • Markdown report           │
│ • Trend charts              │ │ • HTML report               │
│ • Category breakdown        │ │ • PDF report                │
└─────────────────────────────┘ └─────────────────────────────┘
                    │                   │
                    └─────────┬─────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Persistence Layer (src/engines/database.rs)                            │
│  • Store assessment in PostgreSQL                                      │
│  • Store risk factors                                                  │
│  • Update historical trends                                            │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│ Output to User                                                          │
│  • Display results in terminal                                         │
│  • Generate reports                                                    │
│  • Send notifications (if configured)                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Code Style and Standards

### 4.1 Rust API Guidelines

Follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/). Key points:

**Naming:**
- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

**Documentation:**
- All public items require rustdoc comments
- Include examples in documentation
- Document errors, panics, and safety

**Error Handling:**
- Use `Result<T, Error>` for fallible operations
- Use custom error types with `thiserror`
- Provide context with error messages

**API Design:**
- Prefer iterators over returning `Vec`
- Use `&str` for input, `String` for output
- Implement standard traits (Debug, Clone, etc.)

### 4.2 Async/Await Patterns

**Use `async-trait` for trait methods:**

```rust
#[async_trait]
pub trait DataCollector: Send + Sync {
    async fn collect(&self) -> Result<Vec<DataPoint>>;
}
```

**Spawn concurrent tasks with `tokio::spawn`:**

```rust
let tasks: Vec<_> = collectors
    .iter()
    .map(|collector| {
        let collector = collector.clone();
        tokio::spawn(async move {
            collector.collect().await
        })
    })
    .collect();

let results = futures::future::join_all(tasks).await;
```

**Use `tokio::select!` for timeouts:**

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(30),
    some_async_operation()
).await;

match result {
    Ok(Ok(value)) => println!("Success: {:?}", value),
    Ok(Err(e)) => eprintln!("Operation failed: {}", e),
    Err(_) => eprintln!("Timeout"),
}
```

### 4.3 Error Handling Patterns

**Define comprehensive error types:**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database operation failed: {operation}")]
    Database {
        operation: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Claude API error: {message}")]
    ClaudeApi {
        message: String,
        status_code: Option<u16>,
    },

    #[error("Data collection failed: {collector}")]
    Collection {
        collector: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Provide context when propagating errors:**

```rust
// Good: Add context
let config = load_config(&config_path)
    .map_err(|e| Error::Configuration(
        format!("Failed to load config from {:?}: {}", config_path, e)
    ))?;

// Better: Use context helpers
let config = load_config(&config_path)
    .context(format!("Loading config from {:?}", config_path))?;
```

### 4.4 Logging and Instrumentation

**Use `tracing` for structured logging:**

```rust
use tracing::{info, warn, error, debug, trace, instrument};

#[instrument(skip(self, data))]
pub async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis> {
    info!(
        data_points = data.len(),
        category = ?self.risk_category(),
        "Starting risk analysis"
    );

    let indicators = self.extract_indicators(data)?;
    debug!(indicators_count = indicators.len(), "Indicators extracted");

    // ... analysis logic ...

    info!(
        risk_score = analysis.overall_score,
        confidence = ?analysis.confidence,
        "Risk analysis complete"
    );

    Ok(analysis)
}
```

**Log levels:**
- `error!`: Errors that require immediate attention
- `warn!`: Warnings about unusual conditions
- `info!`: Important state changes and milestones
- `debug!`: Detailed debugging information
- `trace!`: Very verbose, fine-grained details

---

## 5. Implementing New Features

### 5.1 Adding a New Data Collector

**Step 1: Create collector module**

File: `src/collectors/news/new_source.rs`

```rust
use crate::{
    collectors::traits::DataCollector,
    models::{DataPoint, DataCategory},
    error::{Error, Result},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

pub struct NewSourceCollector {
    http_client: reqwest::Client,
    config: NewSourceConfig,
}

#[derive(Debug, Clone)]
pub struct NewSourceConfig {
    pub api_url: String,
    pub api_key: Option<String>,
    pub cache_ttl: Duration,
}

impl NewSourceCollector {
    pub fn new(config: NewSourceConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("WarGames-JOSHUA/1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
            config,
        }
    }
}

#[async_trait]
impl DataCollector for NewSourceCollector {
    async fn collect(&self) -> Result<Vec<DataPoint>> {
        // Implementation
        todo!("Implement data collection logic")
    }

    fn source_name(&self) -> &str {
        "New Source Name"
    }

    fn reliability_score(&self) -> f64 {
        0.80
    }

    fn category(&self) -> DataCategory {
        DataCategory::NewsMedia
    }

    fn rate_limit(&self) -> Option<u32> {
        Some(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collect_success() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_collect_handles_error() {
        // Test error handling
    }
}
```

**Step 2: Register in collector module**

File: `src/collectors/news/mod.rs`

```rust
pub mod reuters;
pub mod ap;
pub mod new_source;  // Add your new collector

pub use new_source::{NewSourceCollector, NewSourceConfig};
```

**Step 3: Add to data collection engine**

File: `src/engines/data_collection.rs`

```rust
use crate::collectors::news::NewSourceCollector;

impl DataCollectionEngine {
    pub fn new(config: CollectionConfig) -> Self {
        let mut collectors: Vec<Box<dyn DataCollector>> = vec![
            Box::new(ReutersCollector::new()),
            Box::new(APCollector::new()),
            Box::new(NewSourceCollector::new(new_source_config)),  // Add here
        ];

        // ... rest of implementation
    }
}
```

**Step 4: Add tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_new_source_collector() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Setup mock response
        Mock::given(method("GET"))
            .and(path("/api/articles"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "articles": [
                    {
                        "title": "Test Article",
                        "content": "Test content",
                        "url": "https://example.com/article",
                        "published": "2025-10-27T12:00:00Z"
                    }
                ]
            })))
            .mount(&mock_server)
            .await;

        // Create collector with mock server URL
        let config = NewSourceConfig {
            api_url: mock_server.uri(),
            api_key: None,
            cache_ttl: Duration::from_secs(60),
        };

        let collector = NewSourceCollector::new(config);

        // Test collection
        let result = collector.collect().await;
        assert!(result.is_ok());

        let data_points = result.unwrap();
        assert_eq!(data_points.len(), 1);
        assert_eq!(data_points[0].title.as_deref(), Some("Test Article"));
    }
}
```

### 5.2 Adding a New Risk Analyzer

Similar pattern to data collectors. See Architecture guide (doc 06) for detailed examples.

### 5.3 Adding a New Visualization

**Step 1: Create visualizer**

File: `src/visualizers/new_chart.rs`

```rust
use crate::{
    visualizers::traits::Visualizer,
    models::{RiskAssessment, Visualization, VisualizationFormat},
    error::{Error, Result},
};
use plotters::prelude::*;

pub struct NewChartVisualizer {
    config: ChartConfig,
}

impl Visualizer for NewChartVisualizer {
    fn visualize(&self, assessment: &RiskAssessment) -> Result<Visualization> {
        // Create drawing area
        let root = SVGBackend::new("new_chart.svg", (1200, 800))
            .into_drawing_area();

        root.fill(&WHITE)?;

        // Draw chart
        // ... implementation ...

        root.present()?;

        Ok(Visualization {
            name: self.name().to_string(),
            format: VisualizationFormat::Svg,
            file_path: PathBuf::from("new_chart.svg"),
            metadata: VisualizationMetadata {
                width: 1200,
                height: 800,
                generated_at: Utc::now(),
            },
        })
    }

    fn name(&self) -> &str {
        "New Chart Visualization"
    }

    fn supported_formats(&self) -> &[VisualizationFormat] {
        &[VisualizationFormat::Svg, VisualizationFormat::Png]
    }
}
```

**Step 2: Register and use**

---

## 6. Testing Guidelines

### 6.1 Test Coverage Requirements

**Target: 95%+ test coverage**

Check coverage:

```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open report
open coverage/index.html
```

### 6.2 Unit Testing

**Test organization:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod risk_factor_tests {
        use super::*;

        #[test]
        fn test_creation_with_valid_values() {
            let factor = RiskFactor::new(
                "Test Factor",
                0.75,
                ConfidenceLevel::High,
            );

            assert_eq!(factor.name, "Test Factor");
            assert_eq!(factor.value, 0.75);
        }

        #[test]
        #[should_panic(expected = "Invalid risk value")]
        fn test_creation_rejects_invalid_value() {
            RiskFactor::new("Invalid", 1.5, ConfidenceLevel::High);
        }
    }

    mod calculation_tests {
        use super::*;

        #[test]
        fn test_weighted_score_calculation() {
            // Test implementation
        }
    }
}
```

### 6.3 Integration Testing

File: `tests/integration/assessment_flow.rs`

```rust
use wargames_joshua::prelude::*;

#[tokio::test]
async fn test_complete_assessment_flow() {
    // Setup test database
    let db_url = "postgresql://localhost:5432/joshua_test";
    let pool = setup_test_database(db_url).await;

    // Initialize system with test configuration
    let system = WarGamesSystem::new_with_config(test_config()).await.unwrap();

    // Run assessment
    let assessment = system.run_assessment().await.unwrap();

    // Verify results
    assert!(assessment.seconds_to_midnight > 0);
    assert!(assessment.seconds_to_midnight <= 1440);
    assert!(!assessment.risk_factors.is_empty());
    assert!(!assessment.executive_summary.is_empty());

    // Cleanup
    cleanup_test_database(pool).await;
}
```

### 6.4 Property-Based Testing

Use `proptest` for complex logic:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_risk_score_always_in_range(
        weight in 0.0f64..1.0,
        base_value in 0.0f64..1.0,
    ) {
        let score = calculate_weighted_score(base_value, weight);
        prop_assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_seconds_conversion_roundtrip(
        risk_score in 0.0f64..1.0,
    ) {
        let seconds = risk_score_to_seconds(risk_score);
        let converted_back = seconds_to_risk_score(seconds);
        prop_assert!((risk_score - converted_back).abs() < 0.001);
    }
}
```

### 6.5 Mock Testing

Use `mockall` for trait mocks:

```rust
use mockall::predicate::*;
use mockall::mock;

mock! {
    DataCollectorImpl {}

    #[async_trait]
    impl DataCollector for DataCollectorImpl {
        async fn collect(&self) -> Result<Vec<DataPoint>>;
        fn source_name(&self) -> &str;
        fn reliability_score(&self) -> f64;
        fn category(&self) -> DataCategory;
    }
}

#[tokio::test]
async fn test_with_mock_collector() {
    let mut mock_collector = MockDataCollectorImpl::new();

    mock_collector
        .expect_collect()
        .times(1)
        .returning(|| Ok(vec![test_data_point()]));

    mock_collector
        .expect_source_name()
        .return_const("Mock Source");

    // Use mock in test
    let result = mock_collector.collect().await.unwrap();
    assert_eq!(result.len(), 1);
}
```

### 6.6 Benchmark Testing

File: `benches/risk_calculation.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wargames_joshua::engines::risk_calculation::calculate_bayesian_score;

fn benchmark_bayesian_calculation(c: &mut Criterion) {
    let factors = create_benchmark_factors();

    c.bench_function("bayesian_score_calculation", |b| {
        b.iter(|| {
            calculate_bayesian_score(black_box(&factors))
        })
    });
}

criterion_group!(benches, benchmark_bayesian_calculation);
criterion_main!(benches);
```

Run benchmarks:

```bash
cargo bench
```

---

## 7. Documentation Requirements

### 7.1 Rustdoc Standards

**All public APIs must have documentation:**

```rust
/// Calculates the overall nuclear risk score using weighted factors.
///
/// This function implements the core risk calculation algorithm, combining:
/// - Weighted scoring across risk categories
/// - Bayesian adjustment based on historical correlations
/// - Monte Carlo simulation for uncertainty quantification
///
/// # Arguments
///
/// * `factors` - A slice of [`RiskFactor`] instances to aggregate
/// * `weights` - Category weights that must sum to 1.0
/// * `config` - Calculation configuration including simulation parameters
///
/// # Returns
///
/// Returns a [`Result`] containing the calculated risk score (0.0-1.0) or
/// an error if calculation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - Category weights don't sum to 1.0 (within 0.001 tolerance)
/// - Any risk factor has invalid values (outside 0.0-1.0 range)
/// - Monte Carlo simulation fails to converge
///
/// # Examples
///
/// ```
/// use wargames_joshua::prelude::*;
///
/// let factors = vec![
///     RiskFactor::new("Test Factor", 0.75, ConfidenceLevel::High),
/// ];
///
/// let weights = CategoryWeights::default();
/// let config = CalculationConfig::default();
///
/// let score = calculate_risk_score(&factors, &weights, &config)?;
/// assert!(score >= 0.0 && score <= 1.0);
/// # Ok::<(), Error>(())
/// ```
///
/// # Performance
///
/// Typical execution time: 10-50ms for standard simulations (10,000 iterations).
/// Complexity: O(n * m) where n = number of factors, m = Monte Carlo iterations.
///
/// # See Also
///
/// - [`RiskFactor`] - Individual risk factor representation
/// - [`CategoryWeights`] - Weight configuration
/// - [`calculate_bayesian_score`] - Bayesian adjustment implementation
pub fn calculate_risk_score(
    factors: &[RiskFactor],
    weights: &CategoryWeights,
    config: &CalculationConfig,
) -> Result<f64> {
    // Implementation
}
```

### 7.2 Module Documentation

```rust
//! # Data Collection Module
//!
//! This module provides the data collection engine and individual collector
//! implementations for gathering nuclear risk data from multiple sources.
//!
//! ## Architecture
//!
//! The module is organized around the [`DataCollector`] trait, which all
//! data sources must implement. The [`DataCollectionEngine`] orchestrates
//! parallel collection from multiple sources with retry logic, rate limiting,
//! and error recovery.
//!
//! ## Usage
//!
//! ```no_run
//! use wargames_joshua::collectors::DataCollectionEngine;
//! use wargames_joshua::utils::config::CollectionConfig;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = CollectionConfig::load("config.toml")?;
//! let engine = DataCollectionEngine::new(config);
//!
//! let data = engine.collect_all().await?;
//! println!("Collected {} data points", data.data_points.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Collectors
//!
//! - News sources: Reuters, AP, BBC, Al Jazeera
//! - Research institutions: SIPRI, Bulletin of Atomic Scientists, Carnegie
//! - Government sources: State Department, IAEA
//!
//! ## Adding New Collectors
//!
//! See the [Contributing Guide](../../CONTRIBUTING.md#adding-a-new-data-collector)
//! for instructions on implementing new data sources.
```

---

## 8. Pull Request Process

### 8.1 Before Submitting

**1. Run full CI checks locally:**

```bash
just ci
# Or manually:
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

**2. Update CHANGELOG.md:**

```markdown
## [Unreleased]

### Added
- New Reuters RSS feed collector for nuclear news (#123)

### Changed
- Improved Claude API retry logic with exponential backoff (#124)

### Fixed
- Database connection pool exhaustion under high load (#125)
```

**3. Update documentation:**

- Rustdoc for new/changed APIs
- User documentation if behavior changes
- README.md for new features

**4. Test on multiple platforms (if possible):**

```bash
# Use cross for cross-compilation testing
cargo install cross
cross test --target x86_64-unknown-linux-gnu
cross test --target x86_64-apple-darwin
```

### 8.2 Pull Request Template

When opening a PR, use this template:

```markdown
## Description

Brief description of what this PR accomplishes.

Fixes #(issue number)

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring (no functional changes)

## Testing

Describe the tests you ran and how to reproduce them.

- [ ] Unit tests pass (`cargo test`)
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] Benchmarks show no regression (if applicable)

## Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated (rustdoc, README, CHANGELOG)
- [ ] Tests added/updated for new functionality
- [ ] Commit messages follow conventional commits
- [ ] No breaking changes (or clearly documented)
- [ ] Related issue referenced

## Additional Context

Add any other context about the PR here.
```

### 8.3 Review Process

**Reviewers will check:**

1. **Code quality** - Style, readability, maintainability
2. **Correctness** - Logic, edge cases, error handling
3. **Testing** - Coverage, quality, realistic scenarios
4. **Documentation** - Completeness, clarity, examples
5. **Performance** - Efficiency, resource usage
6. **Security** - Input validation, API key handling, SQL injection
7. **Architecture** - Alignment with project structure

**Expected timeline:**

- Initial review: 1-3 business days
- Follow-up reviews: 1-2 business days
- Merge after approval and CI passing

---

## 9. Phase-Based Development

### 9.1 Current Phase Status

**Phase 0: Foundation (COMPLETE)**
- Project structure established
- Documentation suite complete (docs 00-15)
- Architecture defined
- Technology stack selected

**Phase 1: Core Infrastructure (IN PROGRESS)**

Focus areas:
- Data collection engine
- RSS feed parsers
- News API integrations
- Basic database operations

**Phase 2: AI Integration (UPCOMING)**

Focus areas:
- Claude API integration
- Prompt engineering
- Response parsing
- Cost optimization

**Phase 3: Risk Calculation (FUTURE)**

Focus areas:
- Weighted scoring implementation
- Bayesian networks
- Monte Carlo simulation
- Historical validation

### 9.2 Contributing to Current Phase

**Phase 1 priorities:**

1. **Data collector implementations**
   - Implement collectors for news sources (Reuters, AP, BBC)
   - Implement research institution collectors (SIPRI, Carnegie)
   - Add error handling and retry logic

2. **Data processing pipeline**
   - Content deduplication
   - Quality scoring
   - Relevance filtering

3. **Basic database operations**
   - CRUD operations for assessments
   - Historical data storage
   - Query optimization

**How to help:**

```bash
# Check open issues for Phase 1
https://github.com/yourusername/wargames-joshua/labels/phase-1

# Pick an issue or propose new work
# Create feature branch
git checkout -b feat/phase1-reuters-collector

# Implement, test, document
# Submit PR with "Phase 1:" prefix
```

---

## 10. Security Considerations

### 10.1 Secrets Management

**NEVER commit secrets:**

```bash
# .gitignore
config/local_config.toml
config/*_secret.toml
*.key
*.pem
.env
```

**Use environment variables:**

```rust
use std::env;

let api_key = env::var("ANTHROPIC_API_KEY")
    .map_err(|_| Error::Configuration(
        "ANTHROPIC_API_KEY environment variable not set".to_string()
    ))?;
```

**Or secrets management services:**

```rust
// Example with AWS Secrets Manager
async fn load_api_key_from_aws() -> Result<String> {
    let client = aws_sdk_secretsmanager::Client::new(&aws_config::load_from_env().await);
    let response = client
        .get_secret_value()
        .secret_id("joshua/claude-api-key")
        .send()
        .await?;

    Ok(response.secret_string().unwrap().to_string())
}
```

### 10.2 Input Validation

**Validate all external inputs:**

```rust
pub fn validate_risk_score(score: f64) -> Result<f64> {
    if !score.is_finite() {
        return Err(Error::Validation("Risk score must be finite".to_string()));
    }

    if !(0.0..=1.0).contains(&score) {
        return Err(Error::Validation(
            format!("Risk score {} out of valid range [0.0, 1.0]", score)
        ));
    }

    Ok(score)
}
```

### 10.3 SQL Injection Prevention

**Always use parameterized queries:**

```rust
// Good: Parameterized query
let assessment = sqlx::query_as!(
    AssessmentRecord,
    "SELECT * FROM assessments WHERE id = $1",
    assessment_id
)
.fetch_one(&pool)
.await?;

// BAD: String concatenation (NEVER DO THIS)
let query = format!("SELECT * FROM assessments WHERE id = '{}'", assessment_id);
sqlx::query(&query).fetch_one(&pool).await?;
```

---

## 11. Performance Guidelines

### 11.1 Profiling

**CPU profiling:**

```bash
# Install flamegraph
cargo install flamegraph

# Profile assessment
cargo flamegraph --bin joshua -- assess

# Open flamegraph.svg
open flamegraph.svg
```

**Memory profiling:**

```bash
# Use valgrind on Linux
valgrind --tool=massif --massif-out-file=massif.out target/release/joshua assess

# Analyze
ms_print massif.out
```

### 11.2 Database Optimization

**Use proper indexes:**

```sql
-- Index on frequently queried columns
CREATE INDEX idx_assessments_date_desc ON assessments(assessment_date DESC);

-- Composite index for common query patterns
CREATE INDEX idx_risk_factors_assessment_category
    ON risk_factors(assessment_id, factor_category);
```

**Batch operations:**

```rust
// Good: Batch insert
let mut transaction = pool.begin().await?;

for factor in risk_factors {
    sqlx::query!(
        "INSERT INTO risk_factors (...) VALUES (...)",
        // ... parameters
    )
    .execute(&mut transaction)
    .await?;
}

transaction.commit().await?;
```

### 11.3 Caching Strategies

**Implement multi-level caching:**

```rust
// L1: In-memory cache
if let Some(cached) = self.memory_cache.get(&key) {
    return Ok(cached.clone());
}

// L2: Redis cache (if enabled)
if let Some(redis) = &self.redis {
    if let Some(cached) = redis.get(&key).await? {
        self.memory_cache.insert(key.clone(), cached.clone());
        return Ok(cached);
    }
}

// L3: Database
let value = self.database.fetch(&key).await?;

// Update caches
if let Some(redis) = &self.redis {
    redis.set(&key, &value, ttl).await?;
}
self.memory_cache.insert(key, value.clone());

Ok(value)
```

See [Performance Optimization Guide](15_Performance_Optimization_Guide.md) for more details.

---

## 12. Release Process

### 12.1 Versioning

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### 12.2 Release Checklist

```markdown
- [ ] All tests pass on CI
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Documentation updated
- [ ] Security audit clean (`cargo audit`)
- [ ] Performance benchmarks reviewed
- [ ] Migration guide (if breaking changes)
- [ ] Release notes drafted
- [ ] Git tag created
- [ ] Binary artifacts built
- [ ] crates.io published
- [ ] GitHub release created
- [ ] Announcements sent
```

### 12.3 Hotfix Process

For critical bugs in production:

1. Create hotfix branch from latest release tag
2. Fix the bug with minimal changes
3. Test thoroughly
4. Bump PATCH version
5. Create release
6. Merge back to main

---

## 13. Getting Help

### 13.1 Resources

- **Documentation**: `docs/` directory
- **API reference**: `cargo doc --open`
- **Examples**: `examples/` directory
- **Tests**: Comprehensive examples in `tests/`

### 13.2 Communication

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: Questions, ideas
- **Pull Request comments**: Code-specific discussions

### 13.3 Asking Good Questions

Include:
1. What you're trying to accomplish
2. What you've tried
3. Minimal reproducible example
4. Error messages and logs
5. Environment details (OS, Rust version, etc.)

---

**Document Version:** 1.0.0
**Last Updated:** October 27, 2025
**Maintained By:** WarGames/JOSHUA Development Team

---

*"The only winning move is not to play - but if we must build the monitoring system, let's build it with excellence."*

---

Thank you for contributing to WarGames/JOSHUA! Your efforts help create a more transparent and accountable approach to nuclear risk assessment.
