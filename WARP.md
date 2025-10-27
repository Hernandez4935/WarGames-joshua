# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**WarGames/JOSHUA** is a nuclear war risk assessment system that combines multi-source data collection, AI-powered analysis (Claude/Anthropic), and statistical modeling to monitor global nuclear threats.

- **Status**: Phase 0 (Foundation & Architecture) complete — core structure established; moving to Phase 1 (Data Collection Engine implementation)
- **Language**: Rust (edition 2021)
- **Binary name**: `joshua`
- **Inspiration**: 1983 film *WarGames* and the Bulletin of Atomic Scientists' Doomsday Clock framework

## Common Development Commands

### Build
```bash
cargo build              # Development build
cargo build --release    # Optimized release build
cargo check              # Check without building
```

### Test
```bash
cargo test                                  # Run all tests
cargo test -- --nocapture                   # Run tests with output
cargo test test_name                        # Run specific test
cargo test --test integration_test          # Run integration tests only
cargo tarpaulin --out Html                  # Generate code coverage
```

### Lint & Format
```bash
cargo clippy -- -D warnings    # Lint with all warnings as errors (CI standard)
cargo fmt                      # Format code
cargo fmt --check              # Check formatting without modifying
```

### Documentation
```bash
cargo doc --open    # Generate and open documentation
```

### Run
```bash
cargo run -- diagnose      # System diagnostics
cargo run -- assess        # Run risk assessment (stub)
cargo run -- history       # View assessment history (stub)
cargo run -- interactive   # Interactive TUI mode (stub)
cargo run -- init-db       # Initialize database (stub)
```

## High-Level Architecture

### Core Data Flow Pipeline
```
Data Collection → Claude Analysis → Risk Calculation → Visualization & Reporting
```

1. **Data Collection Engine**: Parallel collection from 10+ diverse sources (news APIs, think tanks, government reports, social media)
2. **Claude Analysis Engine**: AI-powered risk assessment with contextual memory using Anthropic's Claude API
3. **Risk Calculation Engine**: Multi-factor weighted scoring with Bayesian adjustment and Monte Carlo simulation
4. **Visualization Engine**: Comprehensive charts (Doomsday Clock, trends, heat maps) using Plotters
5. **Report Generator**: Markdown/HTML/PDF reports with embedded visualizations

### System Orchestrator
- **WarGamesSystem** in `src/lib.rs` is the main entry point that coordinates all engines
- Designed as an async-first system with Tokio runtime
- Each engine is self-contained and independently testable

### Trait-Based Extensibility
The system uses trait-based abstractions for all major components:

- **DataCollector**: Interface for data sources (RSS feeds, APIs, databases)
- **RiskAnalyzer**: Risk assessment for specific categories (arsenal changes, regional conflicts, etc.)
- **Visualizer**: Visualization generation (Doomsday Clock, trend charts, heat maps)
- **Reporter**: Report generation (Markdown, HTML, PDF)
- **DatabaseEngine**: Persistence layer (PostgreSQL/SQLite)
- **NotificationSender**: Alert delivery (webhooks, email, etc.)

### Module Organization
```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root with WarGamesSystem
├── cli/                 # Command-line interface (clap)
├── collectors/          # Data collector implementations
├── analyzers/           # Risk analyzer implementations
├── engines/             # Processing engines
│   ├── data_collection.rs
│   ├── claude_integration.rs
│   ├── risk_calculation.rs
│   ├── database.rs
│   └── notification.rs
├── models/              # Data models (Assessment, RiskFactor, DataPoint)
├── visualizers/         # Visualization generators
├── utils/               # Utilities (config, logging)
├── error.rs             # Comprehensive error types
├── types.rs             # Type definitions (enums, aliases)
└── constants.rs         # System constants
```

## Key Implementation Patterns

### Error Handling
- Comprehensive error types using `thiserror` in `src/error.rs`
- Context-rich errors with source chains
- Graceful degradation for non-critical failures
- Retry logic with exponential backoff for external API calls

### Async Patterns
- Tokio-based async runtime for efficient I/O
- Parallel data collection with `futures::future::join_all`
- Rate limiting and timeout handling for external APIs
- Lock-free patterns where possible (e.g., Arc<RwLock<>>)

### Data Collection
- Parallel collection from multiple sources
- Caching with TTL (6-hour default)
- Rate limiting per source
- Deduplication of collected data points
- Retry logic with exponential backoff

### Claude API Integration
- Structured JSON response parsing
- Exponential backoff retry logic (max 3 attempts)
- System prompt defines JOSHUA persona and risk assessment framework
- Context management for historical continuity
- Response validation and confidence scoring

### Database
- SQLx with compile-time query checking
- PostgreSQL as primary database; SQLite for development
- Migration files in `migrations/`
- Schema includes:
  - `assessments`: Risk scores, confidence levels, trends, executive summaries
  - `risk_factors`: Individual risk factors per assessment
  - `collected_data`: Raw data points from sources
- Indexes optimized for time-series queries

### Configuration
- TOML-based configuration in `config/`
- `default_config.toml`: Template with defaults
- `local_config.toml`: Local overrides (gitignored)
- Environment variable support (e.g., `ANTHROPIC_API_KEY`)

### Logging & Observability
- Structured logging with `tracing` and `tracing-subscriber`
- Configurable log levels (trace, debug, info, warn, error)
- JSON format support for production
- Context propagation through async boundaries

## Testing Strategy

### Test Pyramid
- **Unit Tests (95%+ coverage target)**: Individual functions and methods
- **Integration Tests**: Module combinations in `tests/`
- **E2E Tests**: Full system tests (planned)
- **Property-Based Tests**: Using `proptest` for fuzz testing
- **Performance Benchmarks**: Using `criterion` (in `benches/`)

### Test Utilities
- `mockall` for mocking traits
- `wiremock` for HTTP mock servers
- `tokio-test` for async test utilities

## Project Status & Roadmap

### Phase 0: Foundation & Architecture ✅ **COMPLETE**
- Core Rust project structure
- Module organization
- Error handling system
- Database schema design
- Configuration management
- Testing framework
- CI/CD pipeline

### Phase 1: Data Collection Engine (Weeks 5-10) — **NEXT**
- RSS feed aggregation
- News API integration
- Research institution data collection
- Real-time monitoring

### Subsequent Phases
- **Phase 2 (Weeks 11-16)**: Claude Analysis Engine
- **Phase 3 (Weeks 17-22)**: Risk Calculation & Modeling
- **Phase 4 (Weeks 23-28)**: Visualization & Reporting
- **Phase 5 (Weeks 29-34)**: Integration & Testing
- **Phase 6 (Weeks 35-40)**: Production Readiness

Full roadmap: `docs/01_Development_Roadmap_and_Sprint_Planning.md`

## Risk Assessment Methodology

### Risk Categories (with weights)
1. Nuclear Arsenal Changes (15%)
2. Doctrine and Posture (15%)
3. Regional Conflicts (20%)
4. Leadership & Rhetoric (10%)
5. Technical Incidents (15%)
6. Communication Breakdown (10%)
7. Emerging Technology (10%)
8. Economic Factors (5%)

### Analysis Approach
- Multi-factor weighted scoring: 50+ risk factors across 8 categories
- Bayesian adjustment: Historical correlations and conditional probabilities
- Monte Carlo simulation: 10,000+ iterations for uncertainty quantification
- Time-series analysis: Trend detection with Mann-Kendall tests
- Pattern recognition: Historical parallel identification (Cuban Missile Crisis, 1983 false alarm, etc.)

### Doomsday Clock Framework
- Risk scale: 0 (midnight/nuclear war) to 1440 (noon/minimal risk)
- Current baseline: 89 seconds to midnight (as of January 2025)
- Confidence levels: Very Low, Low, Moderate, High, Very High

## Important Files & Documentation

### Core Documentation
- **`CLAUDE.md`**: Comprehensive guidance for Claude Code with project overview, architecture, and development phases
- **`README.md`**: Quick start, installation, usage examples, and project structure
- **`docs/06_Architecture_and_Implementation_Guide.md`**: Detailed technical architecture (1500+ lines) with trait specifications and implementation patterns
- **`docs/05_Database_Design_and_Schema.md`**: Complete database schema and data model specifications
- **`docs/02_Risk_Calculation_Methodology.md`**: Statistical foundations, Bayesian modeling, Monte Carlo simulation
- **`docs/01_Development_Roadmap_and_Sprint_Planning.md`**: 6-phase development roadmap with 40-week timeline

### Configuration
- **`Cargo.toml`**: Dependencies, project metadata, and feature flags
- **`config/default_config.toml`**: Default configuration template
- **`.github/workflows/ci.yml`**: CI/CD pipeline (test, fmt, clippy, docs, coverage)

### Reference Materials
- **`ref-docs/WarGames-joshua_AppSpec.md`**: Complete application specification (1,300+ lines)
- **`ref-docs/WarGames-joshua_DIAGRAMS.md`**: System diagrams and visual architecture
- **`ref-docs/Nuclear Precipice - Earth at 89 Seconds to Midnight.md`**: Context on current nuclear risk

## Technology Stack

### Core Dependencies
- **Async Runtime**: `tokio` (full features)
- **HTTP Client**: `reqwest` (JSON, rustls-tls)
- **CLI**: `clap` (derive features)
- **Database**: `sqlx` (PostgreSQL, SQLite, migrations)
- **Serialization**: `serde`, `serde_json`, `toml`
- **Date/Time**: `chrono` (serde features)
- **Visualization**: `plotters` (SVG, bitmap)
- **Terminal UI**: `ratatui`, `crossterm`
- **Templating**: `handlebars`
- **Error Handling**: `thiserror`, `anyhow`
- **Logging**: `tracing`, `tracing-subscriber`
- **Statistics**: `statrs`, `ndarray`
- **Parallelism**: `rayon`

### Dev Dependencies
- **Testing**: `proptest`, `mockall`, `wiremock`, `tokio-test`
- **Benchmarking**: `criterion`

## Notes for Future Instances

### Security Considerations
- API keys must be encrypted at rest
- Use `ANTHROPIC_API_KEY` environment variable (never hardcode)
- Rate limiting on all external API calls
- Input validation and sanitization
- Audit logging of all operations
- SQL injection prevention via prepared statements (SQLx)

### Performance Targets
- Complete assessment: <5 minutes
- Memory usage: <500MB
- Database queries: Optimized with proper indexes
- API response caching: 6-hour TTL

### Development Workflow
1. Follow trait-based design patterns for extensibility
2. Write tests before implementation (TDD approach)
3. Use comprehensive error types with context
4. Run `cargo clippy -- -D warnings` before committing (CI will enforce)
5. Run `cargo fmt` before committing
6. Ensure tests pass: `cargo test`
7. Document public APIs with doc comments

### Ethical Context
This system monitors nuclear war risk. Development should prioritize accuracy, scientific rigor, and responsible use. The famous quote from *WarGames* applies: *"The only winning move is not to play. How about a nice game of chess?"*
