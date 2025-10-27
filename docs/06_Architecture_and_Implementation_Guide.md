╔══════════════════════════════════════════════════════════════════════════════╗
║                        WARGAMES/JOSHUA SYSTEM                                ║
║              Architecture & Implementation Guide                             ║
║                                                                              ║
║                        Version 1.0.0 | October 2025                          ║
║                                                                              ║
║        "The only winning move is not to play - but we must monitor"          ║
╚══════════════════════════════════════════════════════════════════════════════╝


═══════════════════════════════════════════════════════════════════════════════
  EXECUTIVE SUMMARY
═══════════════════════════════════════════════════════════════════════════════

This document provides the complete architectural blueprint and implementation 
specifications for the WarGames/JOSHUA nuclear risk assessment system. It serves 
as the definitive technical reference for developers implementing the system in 
Rust, covering module design, data flow, API integration patterns, and critical 
implementation details.

┌─────────────────────────────────────────────────────────────────────────────┐
│ DOCUMENT PURPOSE                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ • Primary Audience : Rust developers implementing WarGames/JOSHUA           │
│ • Scope           : Complete system architecture from CLI to persistence    │
│ • Level           : Implementation-ready specifications with examples       │
│ • Usage           : Reference during all development phases                 │
└─────────────────────────────────────────────────────────────────────────────┘


╔══════════════════════════════════════════════════════════════════════════════╗
║                           TABLE OF CONTENTS                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝

 1. System Architecture Overview
 2. Module Hierarchy and Organization
 3. Core Data Flow
 4. Critical Implementation Patterns
 5. Error Handling Strategy
 6. Concurrency and Async Patterns
 7. Configuration Management
 8. Logging and Observability
 9. Testing Architecture
10. Performance Considerations


═══════════════════════════════════════════════════════════════════════════════
  1. SYSTEM ARCHITECTURE OVERVIEW
═══════════════════════════════════════════════════════════════════════════════

┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                          HIGH-LEVEL COMPONENT DIAGRAM                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                         CLI Entry Point (main.rs)                         ║
║                                                                           ║
║           joshua assess | report | trend | schedule | interactive         ║
╚═══════════════════════════════════════╤═══════════════════════════════════╝
                                        │
                              ┌─────────▼─────────┐
                              │  Command Parser   │
                              │      (clap)       │
                              └─────────┬─────────┘
                                        │
╔═══════════════════════════════════════▼═══════════════════════════════════╗
║                    WarGamesSystem Orchestrator (lib.rs)                   ║
║                                                                           ║
║  ┌──────────────────── Coordinates All Engines ──────────────────────┐    ║
║  │                                                                   │    ║
║  │  • Data Collection  • Risk Calculation  • AI Analysis             │    ║
║  │  • Visualization    • Report Generation • Notifications           │    ║
║  │  • Storage          • Scheduling        • Terminal UI             │    ║
║  │                                                                   │    ║
║  └───────────────────────────────────────────────────────────────────┘    ║
╚═══════════════════════════════════════╤═══════════════════════════════════╝
                                        │
                     ┌──────────────────┴──────────────────┐
                     │                                     │
            ┌────────▼────────┐                   ┌────────▼────────┐
            │  Processing     │                   │   External      │
            │    Engines      │                   │   Systems       │
            └─────────────────┘                   └─────────────────┘
                     │                                     │
     ┌───────────────┼───────────────┐           ┌────────┼────────┐
     │               │               │           │        │        │
┏━━━━▼━━━━┓   ┏━━━━━▼━━━━━┓   ┏━━━━▼━━━━┓   ┌──▼──┐  ┌─▼──┐  ┌──▼──┐
┃  Data   ┃   ┃   Risk    ┃   ┃   AI    ┃   │News │  │Gov │  │Think│
┃  Coll.  ┃   ┃   Calc.   ┃   ┃  Eng.   ┃   │APIs │  │Src │  │Tank │
┗━━━━┬━━━━┛   ┗━━━━┬━━━━━━┛   ┗━━━━┬━━━━┛   └─────┘  └────┘  └─────┘
     │             │               │             ▲
┏━━━━▼━━━━┓   ┏━━━━━▼━━━━━┓   ┏━━━━▼━━━━┓        │
┃   Viz   ┃   ┃  Report   ┃   ┃   DB    ┃        │  Claude API
┃  Eng.   ┃   ┃   Gen.    ┃   ┃  Eng.   ┃        │  (Anthropic)
┗━━━━┬━━━━┛   ┗━━━━┬━━━━━━┛   ┗━━━━┬━━━━┛        │
     │             │               │             │
┏━━━━▼━━━━━━━━━━━━━━▼━━━━━━━━━━━━━━━▼━━━━━┓      │
┃            Shared Infrastructure        ┃←─────┘
┃  • Models  • Utils  • Config  • Cache   ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

┌─────────────────────────────────────────────────────────────────────────────┐
│ ARCHITECTURE PRINCIPLES                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│ ✓ Modular Design      : Each engine is self-contained and replaceable       │
│ ✓ Async-First         : Tokio runtime for efficient I/O operations          │
│ ✓ Type Safety         : Leverage Rust's type system for correctness         │
│ ✓ Error Transparency  : Comprehensive error types with context              │
│ ✓ Observable          : Structured logging and instrumentation              │
│ ✓ Testable            : Trait-based design enables easy mocking             │
│ ✓ Performant          : Parallel processing, caching, optimization          │
└─────────────────────────────────────────────────────────────────────────────┘


───────────────────────────────────────────────────────────────────────────────
  1.2 Technology Stack
───────────────────────────────────────────────────────────────────────────────

```toml
# Core Dependencies (Cargo.toml)
[dependencies]

# ┌─ Async Runtime ────────────────────────────────────────────┐
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# ┌─ HTTP Client & Server ─────────────────────────────────────┐
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
axum = "0.7"

# ┌─ Serialization ────────────────────────────────────────────┐
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# ┌─ Date/Time ────────────────────────────────────────────────┐
chrono = { version = "0.4", features = ["serde"] }

# ┌─ CLI ──────────────────────────────────────────────────────┐
clap = { version = "4.4", features = ["derive", "cargo"] }

# ┌─ Database ─────────────────────────────────────────────────┐
sqlx = { version = "0.7", features = [
    "runtime-tokio-rustls", "postgres", "sqlite",
    "chrono", "uuid", "json"
] }

# ┌─ Caching ──────────────────────────────────────────────────┐
redis = { version = "0.24", features = ["tokio-comp"] }

# ┌─ Visualization ────────────────────────────────────────────┐
plotters = { version = "0.3", features = ["svg", "bitmap"] }
resvg = "0.37"

# ┌─ Terminal UI ──────────────────────────────────────────────┐
ratatui = "0.25"
crossterm = "0.27"

# ┌─ Templating ───────────────────────────────────────────────┐
handlebars = "5.1"

# ┌─ Utilities ────────────────────────────────────────────────┐
uuid = { version = "1.6", features = ["v4", "serde"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# ┌─ Cryptography ─────────────────────────────────────────────┐
argon2 = "0.5"
base64 = "0.21"
sha2 = "0.10"

# ┌─ Statistics ───────────────────────────────────────────────┐
statrs = "0.16"
ndarray = "0.15"

# ┌─ Parallelism ──────────────────────────────────────────────┐
rayon = "1.8"

[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
mockall = "0.12"
wiremock = "0.6"
```


═══════════════════════════════════════════════════════════════════════════════
  2. MODULE HIERARCHY AND ORGANIZATION
═══════════════════════════════════════════════════════════════════════════════

───────────────────────────────────────────────────────────────────────────────
  2.1 Core Library Structure (lib.rs)
───────────────────────────────────────────────────────────────────────────────

```rust
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
//! use wargames::WarGamesSystem;
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
//!     println!("Risk: {} seconds to midnight", assessment.risk_score);
//!     println!("Report: {}", assessment.report_path);
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

// ═══════════════════════════════════════════════════════════════════════
// Module Declarations
// ═══════════════════════════════════════════════════════════════════════

pub mod cli;           // Command-line interface
pub mod collectors;    // Data collectors
pub mod analyzers;     // Risk analyzers
pub mod engines;       // Processing engines
pub mod models;        // Data models
pub mod visualizers;   // Visualization
pub mod utils;         // Utilities
pub mod error;         // Error types
pub mod types;         // Type aliases
pub mod constants;     // Constants

// ═══════════════════════════════════════════════════════════════════════
// Public Re-exports
// ═══════════════════════════════════════════════════════════════════════

pub use error::{Error, Result};
pub use models::{Assessment, RiskFactor, RiskScore};

// ═══════════════════════════════════════════════════════════════════════
// Prelude - Commonly Used Types
// ═══════════════════════════════════════════════════════════════════════

pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::models::*;
    pub use crate::engines::*;
    pub use crate::utils::config::Config;
    pub use chrono::{DateTime, Utc};
    pub use uuid::Uuid;
}
```


═══════════════════════════════════════════════════════════════════════════════
  3. CORE DATA FLOW
═══════════════════════════════════════════════════════════════════════════════

```
┌──────────────────────── DATA TRANSFORMATION PIPELINE ─────────────────────────┐
│                                                                               │
│  ┏━━━━━━━━━━━━━┓    ┏━━━━━━━━━━━━━┓    ┏━━━━━━━━━━━━━┓    ┏━━━━━━━━━━━━━┓     │
│  ┃   Raw Data  ┃───▶┃  Processed  ┃───▶┃    Risk     ┃───▶┃    Risk     ┃     │
│  ┃   Points    ┃    ┃    Data     ┃    ┃   Factors   ┃    ┃   Scores    ┃     │
│  ┗━━━━━━━━━━━━━┛    ┗━━━━━━━━━━━━━┛    ┗━━━━━━━━━━━━━┛    ┗━━━━━━━━━━━━━┛     │
│       │                    │                    │                    │        │
│       │                    │                    │                    │        │
│  Collector          Validation            AI Analysis         Calculation     │
│   Output           + Quality            via Claude          + Bayesian        │
│   └─ Source         Scoring             └─ Extraction        Adjustment       │
│   └─ Timestamp      └─ Relevance        └─ Confidence       └─ Monte          │
│   └─ Content        └─ Sentiment        └─ Evidence           Carlo           │
│                                                                               │
│  ┏━━━━━━━━━━━━━┓    ┏━━━━━━━━━━━━━┓    ┏━━━━━━━━━━━━━┓                        │
│  ┃   Reports   ┃◀───┃Visualization┃◀───┃ Assessment  ┃                        │
│  ┃ (MD/PDF)    ┃    ┃  (Charts)   ┃    ┃  Complete   ┃                        │
│  ┗━━━━━━━━━━━━━┛    ┗━━━━━━━━━━━━━┛    ┗━━━━━━━━━━━━━┛                        │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
```


═══════════════════════════════════════════════════════════════════════════════
  4. CRITICAL IMPLEMENTATION PATTERNS
═══════════════════════════════════════════════════════════════════════════════

───────────────────────────────────────────────────────────────────────────────
  4.1 Trait-Based Extensibility
───────────────────────────────────────────────────────────────────────────────

The WarGames/JOSHUA system uses trait-based abstraction for all major components,
enabling modularity, testability, and extensibility.

┌─────────────────────────────────────────────────────────────────────────────┐
│ DataCollector Trait - Foundation for All Data Sources                       │
└─────────────────────────────────────────────────────────────────────────────┘

```rust
/// Core trait for all data collection sources
#[async_trait]
pub trait DataCollector: Send + Sync {
    /// Collect data from this source
    async fn collect(&self) -> Result<Vec<DataPoint>>;

    /// Get the source name for logging
    fn source_name(&self) -> &str;

    /// Get the reliability score for this source (0.0-1.0)
    fn reliability_score(&self) -> f64;

    /// Get the collection category
    fn category(&self) -> DataCategory;

    /// Health check with default implementation
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }

    /// Rate limit for this collector (requests per hour)
    fn rate_limit(&self) -> Option<u32> {
        None
    }

    /// Timeout duration for collection operations
    fn timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}

/// Example implementation: Reuters RSS Feed Collector
pub struct ReutersFeedCollector {
    feed_url: String,
    cache: Arc<RwLock<TimedCache<String, Vec<DataPoint>>>>,
    http_client: reqwest::Client,
}

#[async_trait]
impl DataCollector for ReutersFeedCollector {
    async fn collect(&self) -> Result<Vec<DataPoint>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&self.feed_url) {
                return Ok(cached.clone());
            }
        }

        // Fetch RSS feed
        let response = self.http_client
            .get(&self.feed_url)
            .timeout(self.timeout())
            .send()
            .await
            .map_err(|e| Error::Collection {
                collector: self.source_name().to_string(),
                source: Box::new(e),
            })?;

        let content = response.text().await?;

        // Parse RSS feed
        let feed = rss::Channel::read_from(content.as_bytes())?;

        // Convert to DataPoints
        let data_points: Vec<DataPoint> = feed.items()
            .iter()
            .filter_map(|item| {
                Some(DataPoint {
                    id: Uuid::new_v4(),
                    source: self.source_name().to_string(),
                    source_url: item.link().map(|s| s.to_string()),
                    title: item.title().map(|s| s.to_string()),
                    content: item.description().map(|s| s.to_string())?,
                    published_at: item.pub_date()
                        .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
                        .map(|d| d.with_timezone(&Utc)),
                    collected_at: Utc::now(),
                    category: self.category(),
                    reliability: self.reliability_score(),
                    metadata: HashMap::new(),
                })
            })
            .collect();

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(
                self.feed_url.clone(),
                data_points.clone(),
                Duration::from_secs(3600),
            );
        }

        Ok(data_points)
    }

    fn source_name(&self) -> &str {
        "Reuters Nuclear News Feed"
    }

    fn reliability_score(&self) -> f64 {
        0.85
    }

    fn category(&self) -> DataCategory {
        DataCategory::NewsMedia
    }

    fn rate_limit(&self) -> Option<u32> {
        Some(100) // 100 requests per hour
    }
}
```

┌─────────────────────────────────────────────────────────────────────────────┐
│ RiskAnalyzer Trait - Risk Assessment Framework                              │
└─────────────────────────────────────────────────────────────────────────────┘

```rust
/// Trait for risk analysis components
#[async_trait]
pub trait RiskAnalyzer: Send + Sync {
    /// Analyze risk factors and return assessment
    async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis>;

    /// Get the risk category this analyzer covers
    fn risk_category(&self) -> RiskCategory;

    /// Weight of this analyzer's contribution to overall risk
    fn weight(&self) -> f64;

    /// Minimum confidence level required for analysis
    fn min_confidence(&self) -> ConfidenceLevel {
        ConfidenceLevel::Low
    }
}

/// Example: Arsenal Analyzer
pub struct ArsenalAnalyzer {
    historical_data: Arc<HistoricalDatabase>,
    threshold_config: ArsenalThresholds,
}

#[async_trait]
impl RiskAnalyzer for ArsenalAnalyzer {
    async fn analyze(&self, data: &AggregatedData) -> Result<RiskAnalysis> {
        // Extract arsenal-related data points
        let arsenal_data = data.filter_by_category(DataCategory::NuclearArsenal);

        // Parse SIPRI data, government reports, etc.
        let current_arsenals = self.parse_arsenal_data(&arsenal_data)?;

        // Compare with historical baselines
        let historical = self.historical_data
            .get_arsenal_history(Utc::now() - Duration::days(365))
            .await?;

        // Identify significant changes
        let changes = self.identify_significant_changes(&current_arsenals, &historical);

        // Calculate risk scores
        let mut risk_factors = Vec::new();

        for change in changes {
            let risk_value = self.calculate_change_risk(&change);
            let confidence = self.assess_confidence(&change);

            risk_factors.push(RiskFactor {
                category: self.risk_category(),
                name: format!("Arsenal: {}", change.description),
                value: risk_value,
                confidence,
                data_sources: change.sources.clone(),
                timestamp: Utc::now(),
                trend: change.trend,
            });
        }

        // Aggregate into analysis
        let overall_score = self.aggregate_risk_factors(&risk_factors);

        Ok(RiskAnalysis {
            category: self.risk_category(),
            overall_score,
            confidence: self.calculate_overall_confidence(&risk_factors),
            risk_factors,
            summary: self.generate_summary(&risk_factors),
            recommendations: self.generate_recommendations(&risk_factors),
        })
    }

    fn risk_category(&self) -> RiskCategory {
        RiskCategory::NuclearArsenalChanges
    }

    fn weight(&self) -> f64 {
        0.15 // 15% of overall risk calculation
    }
}
```

┌─────────────────────────────────────────────────────────────────────────────┐
│ Visualizer Trait - Visualization Generation                                 │
└─────────────────────────────────────────────────────────────────────────────┘

```rust
/// Trait for visualization generation
pub trait Visualizer: Send + Sync {
    /// Generate visualization from assessment data
    fn visualize(&self, assessment: &RiskAssessment) -> Result<Visualization>;

    /// Name of this visualization
    fn name(&self) -> &str;

    /// Output format(s) supported
    fn supported_formats(&self) -> &[VisualizationFormat];

    /// Default output format
    fn default_format(&self) -> VisualizationFormat {
        VisualizationFormat::Svg
    }
}

/// Example: Doomsday Clock Visualizer
pub struct DoomsdayClockVisualizer {
    config: ClockVisualizationConfig,
}

impl Visualizer for DoomsdayClockVisualizer {
    fn visualize(&self, assessment: &RiskAssessment) -> Result<Visualization> {
        use plotters::prelude::*;

        // Create drawing area
        let root = SVGBackend::new("doomsday_clock.svg", (800, 800))
            .into_drawing_area();
        root.fill(&WHITE)?;

        // Draw clock face
        let clock_center = (400, 400);
        let clock_radius = 350.0;

        // Draw outer circle
        root.draw(&Circle::new(
            clock_center,
            clock_radius as i32,
            &BLACK.mix(0.3).filled(),
        ))?;

        // Calculate hand position from seconds to midnight
        let seconds = assessment.seconds_to_midnight as f64;
        let minutes_to_midnight = seconds / 60.0;

        // Convert to angle (12 o'clock = 0°, clockwise)
        let angle = (90.0 - minutes_to_midnight * 6.0).to_radians();

        // Draw minute hand
        let hand_end = (
            clock_center.0 + (clock_radius * 0.8 * angle.cos()) as i32,
            clock_center.1 - (clock_radius * 0.8 * angle.sin()) as i32,
        );

        root.draw(&PathElement::new(
            vec![clock_center, hand_end],
            &RED.stroke_width(5),
        ))?;

        // Add risk level text
        let risk_level = self.calculate_risk_level(seconds);
        root.draw(&Text::new(
            format!("{} seconds to midnight", seconds),
            (400, 700),
            ("sans-serif", 24).into_font().color(&BLACK),
        ))?;

        root.draw(&Text::new(
            format!("Risk Level: {}", risk_level),
            (400, 730),
            ("sans-serif", 20).into_font().color(&RED),
        ))?;

        root.present()?;

        Ok(Visualization {
            name: self.name().to_string(),
            format: VisualizationFormat::Svg,
            file_path: PathBuf::from("doomsday_clock.svg"),
            metadata: VisualizationMetadata {
                width: 800,
                height: 800,
                generated_at: Utc::now(),
            },
        })
    }

    fn name(&self) -> &str {
        "Doomsday Clock"
    }

    fn supported_formats(&self) -> &[VisualizationFormat] {
        &[VisualizationFormat::Svg, VisualizationFormat::Png]
    }
}
```

┌─────────────────────────────────────────────────────────────────────────────┐
│ Reporter Trait - Report Generation                                          │
└─────────────────────────────────────────────────────────────────────────────┘

```rust
/// Trait for report generation
pub trait Reporter: Send + Sync {
    /// Generate report from assessment
    fn generate(&self, assessment: &RiskAssessment) -> Result<Report>;

    /// Report format
    fn format(&self) -> ReportFormat;

    /// Template engine (if applicable)
    fn template_engine(&self) -> Option<&TemplateEngine> {
        None
    }
}

/// Markdown Report Generator
pub struct MarkdownReporter {
    template_engine: HandlebarsTemplateEngine,
    config: ReportConfig,
}

impl Reporter for MarkdownReporter {
    fn generate(&self, assessment: &RiskAssessment) -> Result<Report> {
        // Prepare template context
        let context = json!({
            "assessment_id": assessment.id,
            "assessment_date": assessment.assessment_date.format("%Y-%m-%d %H:%M:%S UTC"),
            "seconds_to_midnight": assessment.seconds_to_midnight,
            "risk_level": self.categorize_risk(assessment.seconds_to_midnight),
            "executive_summary": assessment.executive_summary,
            "trend_direction": format!("{:?}", assessment.trend_direction),
            "trend_magnitude": format!("{:.2}%", assessment.trend_magnitude * 100.0),
            "confidence": format!("{:?}", assessment.overall_confidence),
            "risk_factors": assessment.risk_factors.iter()
                .take(10)
                .map(|rf| json!({
                    "name": rf.name,
                    "value": format!("{:.3}", rf.value),
                    "category": format!("{:?}", rf.category),
                    "contribution": format!("{:.2}%", rf.contribution_to_risk * 100.0),
                }))
                .collect::<Vec<_>>(),
            "critical_warnings": assessment.critical_warnings,
            "recommendations": assessment.recommendations,
        });

        // Render template
        let content = self.template_engine
            .render("assessment_report", &context)?;

        // Write to file
        let file_path = format!(
            "reports/assessment_{}.md",
            assessment.assessment_date.format("%Y%m%d_%H%M%S")
        );

        std::fs::write(&file_path, &content)?;

        Ok(Report {
            format: self.format(),
            file_path: PathBuf::from(file_path),
            content,
            generated_at: Utc::now(),
            metadata: ReportMetadata {
                assessment_id: assessment.id,
                page_count: None,
                word_count: Some(content.split_whitespace().count()),
            },
        })
    }

    fn format(&self) -> ReportFormat {
        ReportFormat::Markdown
    }

    fn template_engine(&self) -> Option<&TemplateEngine> {
        Some(&self.template_engine.engine)
    }
}
```


───────────────────────────────────────────────────────────────────────────────
  4.2 Database Engine Trait
───────────────────────────────────────────────────────────────────────────────

```rust
/// Trait for database operations
#[async_trait]
pub trait DatabaseEngine: Send + Sync {
    /// Store a new assessment
    async fn store_assessment(&self, assessment: &RiskAssessment) -> Result<Uuid>;

    /// Retrieve assessment by ID
    async fn get_assessment(&self, id: Uuid) -> Result<RiskAssessment>;

    /// Get latest assessment
    async fn get_latest_assessment(&self) -> Result<RiskAssessment>;

    /// Get assessment history within date range
    async fn get_assessment_history(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<RiskAssessment>>;

    /// Store risk factors for an assessment
    async fn store_risk_factors(
        &self,
        assessment_id: Uuid,
        factors: &[RiskFactor],
    ) -> Result<()>;

    /// Health check
    async fn health_check(&self) -> Result<bool>;
}

/// PostgreSQL implementation
pub struct PostgresDatabase {
    pool: sqlx::PgPool,
}

#[async_trait]
impl DatabaseEngine for PostgresDatabase {
    async fn store_assessment(&self, assessment: &RiskAssessment) -> Result<Uuid> {
        let id = assessment.id;

        sqlx::query!(
            r#"
            INSERT INTO assessments (
                id, assessment_date, seconds_to_midnight, raw_risk_score,
                bayesian_adjusted_score, overall_confidence, trend_direction,
                trend_magnitude, delta_from_previous, executive_summary,
                detailed_analysis, claude_model_version
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
            id,
            assessment.assessment_date,
            assessment.seconds_to_midnight as i32,
            assessment.raw_risk_score,
            assessment.bayesian_adjusted_score,
            format!("{:?}", assessment.overall_confidence),
            format!("{:?}", assessment.trend_direction),
            assessment.trend_magnitude,
            assessment.delta_from_previous.map(|d| d as i32),
            assessment.executive_summary,
            assessment.detailed_analysis,
            "claude-3-opus-20240229",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Database {
            operation: "store_assessment".to_string(),
            source: e,
        })?;

        Ok(id)
    }

    async fn get_latest_assessment(&self) -> Result<RiskAssessment> {
        let record = sqlx::query_as!(
            AssessmentRecord,
            r#"
            SELECT * FROM assessments
            ORDER BY assessment_date DESC
            LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Database {
            operation: "get_latest_assessment".to_string(),
            source: e,
        })?;

        // Convert record to RiskAssessment
        Ok(record.into())
    }

    async fn health_check(&self) -> Result<bool> {
        sqlx::query!("SELECT 1 as check")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
            .map_err(|e| Error::Database {
                operation: "health_check".to_string(),
                source: e,
            })
    }
}
```

───────────────────────────────────────────────────────────────────────────────
  4.3 Notification Sender Trait
───────────────────────────────────────────────────────────────────────────────

```rust
/// Trait for notification delivery
#[async_trait]
pub trait NotificationSender: Send + Sync {
    /// Send notification
    async fn send(&self, notification: &Notification) -> Result<()>;

    /// Notification channel name
    fn channel_name(&self) -> &str;

    /// Check if channel is available
    async fn is_available(&self) -> bool {
        true
    }
}

/// Webhook notification sender
pub struct WebhookNotificationSender {
    webhook_url: String,
    http_client: reqwest::Client,
}

#[async_trait]
impl NotificationSender for WebhookNotificationSender {
    async fn send(&self, notification: &Notification) -> Result<()> {
        let payload = json!({
            "title": notification.title,
            "message": notification.message,
            "severity": format!("{:?}", notification.severity),
            "timestamp": notification.timestamp.to_rfc3339(),
            "assessment_id": notification.assessment_id,
        });

        self.http_client
            .post(&self.webhook_url)
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| Error::Notification {
                channel: self.channel_name().to_string(),
                source: Box::new(e),
            })?;

        Ok(())
    }

    fn channel_name(&self) -> &str {
        "Webhook"
    }
}
```


═══════════════════════════════════════════════════════════════════════════════
  5. MODULE SPECIFICATIONS
═══════════════════════════════════════════════════════════════════════════════

This section provides detailed specifications for each major module in the system.

───────────────────────────────────────────────────────────────────────────────
  5.1 Data Collection Engine (engines/data_collection.rs)
───────────────────────────────────────────────────────────────────────────────

**Purpose**: Orchestrate parallel data collection from multiple sources with
caching, rate limiting, and error recovery.

**Public API**:

```rust
/// ┌──────────────────────────────────────────────────────────────────┐
/// │ Data Collection Engine - Primary Entry Point                     │
/// └──────────────────────────────────────────────────────────────────┘
pub struct DataCollectionEngine {
    collectors: Vec<Box<dyn DataCollector>>,
    cache: Arc<RwLock<TimedCache<String, Vec<DataPoint>>>>,
    rate_limiters: HashMap<String, RateLimiter>,
    deduplicator: ContentDeduplicator,
    config: CollectionConfig,
}

impl DataCollectionEngine {
    /// Create new data collection engine with configured collectors
    pub fn new(config: CollectionConfig) -> Self {
        let collectors: Vec<Box<dyn DataCollector>> = vec![
            Box::new(ReutersFeedCollector::new()),
            Box::new(SipriDatabaseCollector::new()),
            Box::new(BulletinAtomicScientistsCollector::new()),
            Box::new(StateDepReportsCollector::new()),
            // ... additional collectors
        ];

        Self {
            collectors,
            cache: Arc::new(RwLock::new(TimedCache::new())),
            rate_limiters: HashMap::new(),
            deduplicator: ContentDeduplicator::new(0.85),
            config,
        }
    }

    /// Collect data from all sources in parallel
    pub async fn collect_all(&self) -> Result<AggregatedData> {
        tracing::info!(
            "Starting data collection from {} sources",
            self.collectors.len()
        );

        let start = Instant::now();

        // Spawn parallel collection tasks
        let futures: Vec<_> = self.collectors
            .iter()
            .map(|collector| {
                let collector = collector.clone();
                let cache = self.cache.clone();

                async move {
                    // Check rate limit
                    if let Some(limiter) = self.rate_limiters.get(collector.source_name()) {
                        limiter.check_and_wait().await?;
                    }

                    // Attempt collection with retries
                    let mut attempts = 0;
                    let max_attempts = 3;

                    loop {
                        attempts += 1;

                        match collector.collect().await {
                            Ok(data) => {
                                tracing::info!(
                                    source = collector.source_name(),
                                    data_points = data.len(),
                                    "Data collection succeeded"
                                );
                                return Ok(data);
                            }
                            Err(e) if attempts < max_attempts => {
                                tracing::warn!(
                                    source = collector.source_name(),
                                    attempt = attempts,
                                    error = %e,
                                    "Collection failed, retrying"
                                );
                                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                            }
                            Err(e) => {
                                tracing::error!(
                                    source = collector.source_name(),
                                    error = %e,
                                    "Data collection failed after {} attempts",
                                    max_attempts
                                );
                                return Err(e);
                            }
                        }
                    }
                }
            })
            .collect();

        // Execute all collections concurrently with timeout
        let results = tokio::time::timeout(
            Duration::from_secs(self.config.global_timeout_seconds),
            futures::future::join_all(futures),
        )
        .await
        .map_err(|_| Error::Collection {
            collector: "GlobalTimeout".to_string(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Global collection timeout"
            )),
        })?;

        // Aggregate successful results
        let mut all_data_points = Vec::new();
        let mut failed_collectors = Vec::new();

        for (idx, result) in results.into_iter().enumerate() {
            match result {
                Ok(data) => all_data_points.extend(data),
                Err(e) => {
                    failed_collectors.push(self.collectors[idx].source_name());
                    tracing::warn!(
                        collector = self.collectors[idx].source_name(),
                        error = %e,
                        "Collector failed"
                    );
                }
            }
        }

        // Deduplicate collected data
        let deduplicated = self.deduplicator.deduplicate(all_data_points)?;

        let duration = start.elapsed();

        tracing::info!(
            total_points = deduplicated.len(),
            failed = failed_collectors.len(),
            duration_ms = duration.as_millis(),
            "Data collection completed"
        );

        Ok(AggregatedData {
            data_points: deduplicated,
            collection_timestamp: Utc::now(),
            sources_count: self.collectors.len() - failed_collectors.len(),
            failed_sources: failed_collectors,
            collection_duration: duration,
        })
    }

    /// Collect from specific category of sources
    pub async fn collect_category(&self, category: DataCategory) -> Result<Vec<DataPoint>> {
        let filtered_collectors: Vec<_> = self.collectors
            .iter()
            .filter(|c| c.category() == category)
            .collect();

        // Similar parallel collection logic...
        todo!("Implement category-specific collection")
    }
}
```

**Key Structs**:

```rust
pub struct AggregatedData {
    pub data_points: Vec<DataPoint>,
    pub collection_timestamp: DateTime<Utc>,
    pub sources_count: usize,
    pub failed_sources: Vec<String>,
    pub collection_duration: Duration,
}

pub struct DataPoint {
    pub id: Uuid,
    pub source: String,
    pub source_url: Option<String>,
    pub title: Option<String>,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub collected_at: DateTime<Utc>,
    pub category: DataCategory,
    pub reliability: f64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataCategory {
    NewsMedia,
    NuclearArsenal,
    RegionalConflict,
    DiplomaticRelations,
    MilitaryExercises,
    TreatyCompliance,
    LeadershipStatements,
    TechnicalIncident,
}
```

───────────────────────────────────────────────────────────────────────────────
  5.2 Claude Integration Engine (engines/claude_integration.rs)
───────────────────────────────────────────────────────────────────────────────

**Purpose**: Interface with Anthropic's Claude API for AI-powered risk analysis,
context management, and structured response parsing.

**Public API**:

```rust
/// ┌──────────────────────────────────────────────────────────────────┐
/// │ Claude Integration Engine - AI Analysis Core                     │
/// └──────────────────────────────────────────────────────────────────┘
pub struct ClaudeIntegrationEngine {
    client: ClaudeClient,
    context_manager: ContextManager,
    prompt_builder: PromptBuilder,
    response_parser: ResponseParser,
    config: ClaudeConfig,
}

impl ClaudeIntegrationEngine {
    /// Analyze aggregated data and produce risk assessment
    pub async fn analyze_risk(
        &self,
        data: &AggregatedData,
    ) -> Result<ClaudeRiskAnalysis> {
        tracing::info!("Starting Claude AI risk analysis");

        // Load historical context for continuity
        let historical_context = self.context_manager
            .load_recent_assessments(5)
            .await?;

        // Build comprehensive analysis prompt
        let prompt = self.prompt_builder
            .build_risk_analysis_prompt(data, &historical_context)?;

        // Execute Claude API call with retry logic
        let response = self.call_claude_with_retry(&prompt, 3).await?;

        // Parse structured response
        let analysis = self.response_parser
            .parse_risk_analysis(&response)?;

        // Validate analysis completeness
        self.validate_analysis(&analysis)?;

        tracing::info!(
            factors_identified = analysis.risk_factors.len(),
            confidence = ?analysis.confidence_level,
            "Claude analysis completed"
        );

        Ok(analysis)
    }

    /// Call Claude API with exponential backoff retry
    async fn call_claude_with_retry(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<String> {
        let mut attempts = 0;

        loop {
            attempts += 1;

            match self.call_claude_api(prompt).await {
                Ok(response) => return Ok(response),
                Err(e) if attempts < max_retries => {
                    let delay = Duration::from_secs(2_u64.pow(attempts));
                    tracing::warn!(
                        attempt = attempts,
                        delay_secs = delay.as_secs(),
                        error = %e,
                        "Claude API call failed, retrying"
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    tracing::error!(
                        attempts = attempts,
                        error = %e,
                        "Claude API call failed permanently"
                    );
                    return Err(e);
                }
            }
        }
    }

    /// Execute single Claude API call
    async fn call_claude_api(&self, prompt: &str) -> Result<String> {
        let request = json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature,
            "system": SYSTEM_PROMPT,
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| Error::ClaudeApi {
                message: format!("Request failed: {}", e),
                status_code: None,
            })?;

        if !response.status().is_success() {
            return Err(Error::ClaudeApi {
                message: format!("API returned error: {}", response.status()),
                status_code: Some(response.status().as_u16()),
            });
        }

        let response_body: serde_json::Value = response.json().await
            .map_err(|e| Error::ClaudeApi {
                message: format!("Failed to parse response: {}", e),
                status_code: None,
            })?;

        // Extract content from response
        let content = response_body
            .get("content")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| Error::ClaudeApi {
                message: "Invalid response format".to_string(),
                status_code: None,
            })?;

        Ok(content.to_string())
    }

    /// Explain changes from previous assessment
    pub async fn explain_risk_delta(
        &self,
        current: &RiskAssessment,
        previous: &RiskAssessment,
    ) -> Result<String> {
        let prompt = self.prompt_builder.build_delta_explanation_prompt(
            current,
            previous,
        )?;

        let response = self.call_claude_with_retry(&prompt, 3).await?;

        Ok(response)
    }
}

/// System prompt for consistent Claude analysis
const SYSTEM_PROMPT: &str = r#"
You are JOSHUA, an advanced nuclear war risk assessment system created to monitor
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

Your responses MUST be valid JSON matching the specified schema exactly.
"#;
```

**Prompt Building**:

```rust
pub struct PromptBuilder {
    templates: HandlebarsTemplateEngine,
}

impl PromptBuilder {
    pub fn build_risk_analysis_prompt(
        &self,
        data: &AggregatedData,
        historical: &[RiskAssessment],
    ) -> Result<String> {
        // Summarize collected data by category
        let data_summary = self.summarize_data_by_category(data)?;

        // Format historical context
        let historical_summary = self.format_historical_context(historical)?;

        let context = json!({
            "current_date": Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "data_points_count": data.data_points.len(),
            "sources_count": data.sources_count,
            "data_summary": data_summary,
            "historical_context": historical_summary,
            "previous_assessment": historical.first().map(|a| json!({
                "date": a.assessment_date,
                "seconds_to_midnight": a.seconds_to_midnight,
                "risk_score": a.raw_risk_score,
                "trend": format!("{:?}", a.trend_direction),
            })),
        });

        self.templates.render("risk_analysis_prompt", &context)
            .map_err(Into::into)
    }

    fn summarize_data_by_category(
        &self,
        data: &AggregatedData,
    ) -> Result<HashMap<DataCategory, Vec<String>>> {
        let mut summary: HashMap<DataCategory, Vec<String>> = HashMap::new();

        for point in &data.data_points {
            let entry = summary.entry(point.category).or_insert_with(Vec::new);

            // Create concise summary of this data point
            let summary_text = format!(
                "[{}] {} - {}",
                point.source,
                point.title.as_deref().unwrap_or(""),
                point.content.chars().take(200).collect::<String>()
            );

            entry.push(summary_text);
        }

        Ok(summary)
    }
}
```

**Response Parsing**:

```rust
pub struct ResponseParser;

impl ResponseParser {
    pub fn parse_risk_analysis(&self, response: &str) -> Result<ClaudeRiskAnalysis> {
        // Claude should return JSON, but may include markdown code blocks
        let json_content = self.extract_json(response)?;

        // Parse JSON into structured analysis
        let parsed: ClaudeRiskAnalysis = serde_json::from_str(&json_content)
            .map_err(|e| Error::ClaudeApi {
                message: format!("Failed to parse Claude response: {}", e),
                status_code: None,
            })?;

        Ok(parsed)
    }

    fn extract_json(&self, text: &str) -> Result<String> {
        // Remove markdown code blocks if present
        let cleaned = text
            .trim()
            .strip_prefix("```json").unwrap_or(text)
            .strip_prefix("```").unwrap_or(text)
            .strip_suffix("```").unwrap_or(text)
            .trim();

        Ok(cleaned.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct ClaudeRiskAnalysis {
    pub seconds_to_midnight: u32,
    pub confidence_level: ConfidenceLevel,
    pub risk_delta: RiskDelta,
    pub critical_developments: Vec<CriticalDevelopment>,
    pub risk_factors: HashMap<String, f64>,
    pub early_warning_indicators: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub executive_summary: String,
    pub detailed_analysis: String,
}

#[derive(Debug, Deserialize)]
pub struct RiskDelta {
    pub change_in_seconds: i32,
    pub primary_drivers: Vec<String>,
    pub trend: TrendDirection,
}

#[derive(Debug, Deserialize)]
pub struct CriticalDevelopment {
    pub event: String,
    pub impact: ImpactLevel,
    pub affected_regions: Vec<String>,
    pub escalation_potential: f64,
}
```


═══════════════════════════════════════════════════════════════════════════════
  6. ERROR HANDLING STRATEGY
═══════════════════════════════════════════════════════════════════════════════

```rust
/// ┌──────────────────────────────────────────────────────────────────┐
/// │ Comprehensive Error Types                                        │
/// └──────────────────────────────────────────────────────────────────┘
#[derive(Debug, thiserror::Error)]
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

    #[error("Notification delivery failed: {channel}")]
    Notification {
        channel: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Parsing error: {0}")]
    Parsing(String),

    #[error("Visualization error: {0}")]
    Visualization(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```


═══════════════════════════════════════════════════════════════════════════════
  6. TESTING ARCHITECTURE
═══════════════════════════════════════════════════════════════════════════════

```
┌─────────────────────────── TEST PYRAMID ──────────────────────────────┐
│                                                                       │
│                            ┌────────────┐                             │
│                            │    E2E     │  ← Full system tests        │
│                            │   Tests    │                             │
│                            └────────────┘                             │
│                                                                       │
│                    ┌───────────────────────────┐                      │
│                    │  Integration Tests        │  ← Module combos     │
│                    └───────────────────────────┘                      │
│                                                                       │
│        ┌───────────────────────────────────────────────┐              │
│        │           Unit Tests (95%+)                   │  ← Functions │
│        └───────────────────────────────────────────────┘              │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```


═══════════════════════════════════════════════════════════════════════════════
  CONCLUSION
═══════════════════════════════════════════════════════════════════════════════

This architecture guide provides the foundational specifications needed for 
implementing the WarGames/JOSHUA system. The modular design, clear separation 
of concerns, and robust error handling patterns ensure the system is 
maintainable, testable, and extensible.

┌─────────────────────────────────────────────────────────────────────────────┐
│ KEY TAKEAWAYS                                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ ✓ Modularity       : Each engine is self-contained and independently        │
│                      testable with clear interfaces                         │
│                                                                             │
│ ✓ Async-First      : Tokio-based async runtime enables efficient I/O        │
│                      operations and parallel processing                     │
│                                                                             │
│ ✓ Error Handling   : Comprehensive error types with retry logic and         │
│                      contextual information                                 │
│                                                                             │
│ ✓ Configuration    : Environment-aware configuration management with        │
│                      sensible defaults                                      │
│                                                                             │
│ ✓ Observability    : Structured logging and instrumentation throughout      │
│                      for debugging and monitoring                           │
│                                                                             │
│ ✓ Testing          : Multi-level testing strategy from unit to              │
│                      integration with property-based tests                  │
│                                                                             │
│ ✓ Performance      : Caching, rate limiting, and optimized database         │
│                      queries for sub-5-minute assessments                   │
└─────────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────────┐
│ NEXT STEPS                                                                 │
├────────────────────────────────────────────────────────────────────────────┤
│ 1. Review this architecture with the development team                      │
│ 2. Begin Phase 0 implementation following the Development Roadmap          │
│ 3. Set up CI/CD pipeline based on these specifications                     │
│ 4. Create initial module skeletons with proper documentation               │
│ 5. Implement core abstractions and traits                                  │
│ 6. Establish testing framework and initial test suite                      │
│ 7. Set up database migrations and initial schema                           │
└────────────────────────────────────────────────────────────────────────────┘

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║  "The only winning move is not to play. But if we must monitor the game,     ║
║   let us do so with precision, vigilance, and unwavering technical           ║
║   excellence."                                                               ║
║                                                                              ║
║                                     - WarGames/JOSHUA Development Team       ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

───────────────────────────────────────────────────────────────────────────────
Document Version: 1.0.0
Last Updated: October 2025  
Maintained By: WarGames/JOSHUA Development Team
Review Frequency: After each major sprint or architectural change
───────────────────────────────────────────────────────────────────────────────
