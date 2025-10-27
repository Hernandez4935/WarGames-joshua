# WarGames/JOSHUA

[![Build Status](https://github.com/doublegate/wargames-joshua/workflows/CI/badge.svg)](https://github.com/doublegate/wargames-joshua/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Phase](https://img.shields.io/badge/phase-0%20complete-brightgreen.svg)](docs/01_Development_Roadmap_and_Sprint_Planning.md)

*Global Thermonuclear War Risk Assessment System*

> "A strange game. The only winning move is not to play. How about a nice game of chess?"
> — WarGames (1983)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Current Status](#current-status)
- [Quick Start](#quick-start)
- [Documentation](#documentation)
- [Architecture](#architecture)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact & Support](#contact--support)

---

## Overview

**WarGames/JOSHUA** is a comprehensive nuclear war risk assessment system inspired by the 1983 film *WarGames*. The project leverages AI-powered analysis through Claude (Anthropic) to perform periodic, detailed evaluations of global nuclear threats, providing continuous monitoring and risk quantification.

### Purpose

In a world where the [Bulletin of Atomic Scientists' Doomsday Clock](https://thebulletin.org/doomsday-clock/) stands at **89 seconds to midnight** (as of January 2025), this system aims to:

- **Monitor global nuclear threats** through multi-source data aggregation
- **Analyze risk factors** using AI-powered assessment (Claude API)
- **Track historical trends** with persistent database storage
- **Visualize threats** through charts, heat maps, and the iconic Doomsday Clock
- **Generate actionable intelligence** with clear recommendations and early warnings

### Key Capabilities

- **Multi-Source Data Collection**: News APIs (Reuters, AP, BBC), think tanks (SIPRI, Carnegie, RAND), government reports, social media intelligence
- **AI-Powered Analysis**: Claude/Anthropic API for consistent, contextual risk assessment with ensemble consensus
- **Statistical Modeling**: Bayesian adjustment, Monte Carlo simulation (10,000+ iterations), time-series analysis
- **Advanced Visualization**: Trend charts, risk matrices, heat maps, interactive Doomsday Clock
- **Historical Tracking**: PostgreSQL database with comprehensive schema (10 tables)
- **Retro Terminal UI**: WarGames-inspired interface with ratatui ("SHALL WE PLAY A GAME?")

---

## Features

### Phase 0: Foundation & Architecture ✅ (Complete)

- [x] **Complete Rust project architecture** with proper module organization
- [x] **Core traits and type system** (6 traits, 4 enums with conversions)
- [x] **Comprehensive error handling** (17 error types with thiserror)
- [x] **CLI framework** with 7 commands (clap derive)
- [x] **Database schema** (PostgreSQL with 10 tables, 3 migrations)
- [x] **Configuration management** (TOML with validation)
- [x] **Logging infrastructure** (tracing framework)
- [x] **Testing framework** (25 tests: 16 unit + 7 integration + 2 doc)
- [x] **CI/CD pipeline** (GitHub Actions with multi-platform support)
- [x] **16 comprehensive documentation files** (25,769 lines total)
- [x] **Repository standards** (licenses, security policy, code of conduct, templates)

### Phase 1: Data Collection Engine 🔄 (Planned - Weeks 5-10)

- [ ] **RSS feed aggregation** with structured parsing
- [ ] **News API integration** (Reuters, Associated Press, BBC, Al Jazeera, RT, Xinhua)
- [ ] **Think tank data collection** (SIPRI, Carnegie Endowment, RAND Corporation)
- [ ] **Government source monitoring** (State Department, IAEA, UN Security Council)
- [ ] **Social media intelligence** (Twitter/X geopolitical monitoring, Reddit analysis)
- [ ] **Rate limiting and caching** (6-hour TTL, exponential backoff)
- [ ] **Data quality scoring** (source reliability weighting)
- [ ] **Real-time monitoring** with scheduled collection

### Phase 2: Claude AI Analysis Engine ⭐ (Planned - Weeks 11-16) **CRITICAL**

- [ ] **Claude API integration** (claude-sonnet-4+ with structured prompts)
- [ ] **Advanced prompt engineering** with JOSHUA persona
- [ ] **Ensemble consensus building** (3-5 independent analyses)
- [ ] **Response parsing and validation** (JSON schema enforcement)
- [ ] **Multi-turn dialogue** for complex assessments
- [ ] **Cost optimization** (caching, batching, prompt tuning)
- [ ] **Fallback strategies** for API failures
- [ ] **Analysis confidence scoring** with uncertainty quantification

### Phase 3: Risk Calculation & Modeling (Planned - Weeks 17-22)

- [ ] **Multi-factor weighted scoring** (50+ factors across 8 categories)
- [ ] **Bayesian adjustment** with historical correlation priors
- [ ] **Monte Carlo simulation** (10,000+ iterations for uncertainty)
- [ ] **Time-series trend analysis** (Mann-Kendall tests, ARIMA models)
- [ ] **Historical pattern recognition** (Cuban Missile Crisis, 1983 false alarm parallels)
- [ ] **Confidence interval calculation** (95% CI with bootstrap resampling)
- [ ] **Scenario modeling** (escalation pathways, de-escalation opportunities)

### Phase 4: Visualization & Reporting (Planned - Weeks 23-28)

- [ ] **Doomsday Clock visualization** (ASCII art + plotters)
- [ ] **Trend charts** (time-series with moving averages)
- [ ] **Risk heat maps** (category × region matrices)
- [ ] **Geospatial mapping** (nuclear nations, incident locations)
- [ ] **Report generation** (Markdown, HTML, PDF with embedded charts)
- [ ] **Interactive terminal UI** (ratatui with retro WarGames aesthetic)
- [ ] **Export capabilities** (CSV, JSON, XML for external analysis)

### Phase 5: Integration & Testing (Planned - Weeks 29-34)

- [ ] **End-to-end integration** with full workflow testing
- [ ] **Comprehensive test coverage** (95%+ target with tarpaulin)
- [ ] **Property-based testing** (proptest for risk calculations)
- [ ] **Performance benchmarking** (criterion for optimization)
- [ ] **Chaos testing** (resilience validation with fault injection)
- [ ] **Security audit** (dependency scanning, SAST/DAST)
- [ ] **Documentation completion** (API docs, user guides, tutorials)

### Phase 6: Production Readiness & Launch (Planned - Weeks 35-40)

- [ ] **Security hardening** (key encryption, input sanitization, audit logging)
- [ ] **Performance optimization** (profiling, caching, parallelization)
- [ ] **Deployment automation** (Docker, Kubernetes, systemd)
- [ ] **Monitoring and alerting** (Prometheus, Grafana, PagerDuty)
- [ ] **Disaster recovery** (backup strategies, failover procedures)
- [ ] **Production launch** with v1.0.0 release

---

## Current Status

**Phase**: 0 (Foundation & Architecture) ✅ **COMPLETE**
**Version**: v0.1.0
**Next Phase**: Phase 1 (Data Collection Engine) - Starting Weeks 5-10
**Production Readiness**: 100% (Phase 0 scope only)

### Project Statistics

| Metric | Value |
|--------|-------|
| **Total Files** | 67 |
| **Rust Source Code** | 2,899 lines across 22 files |
| **Documentation** | 25,769 lines across 16 core documents |
| **Reference Materials** | ~4,000 lines (4 reference docs) |
| **Total Project Lines** | ~32,000 lines |
| **Tests** | 25 (100% passing: 16 unit + 7 integration + 2 doc) |
| **Core Modules** | 10 modules |
| **Key Traits** | 6 trait definitions |
| **Error Types** | 17 comprehensive variants |
| **Database Tables** | 10 tables (3 migrations) |
| **Git Commits** | 4 commits |
| **Phase 0 Completion** | 100% |

### Build & Quality Status

- ✅ **Release Build**: SUCCESS
- ✅ **Test Pass Rate**: 25/25 (100%)
- ✅ **Code Formatted**: rustfmt compliant
- ✅ **Documentation**: Builds successfully with rustdoc
- ⚠️ **Clippy**: 33 pedantic warnings (acceptable for Phase 0 stubs)

### Deliverables Completed

1. **Architecture**: Trait-based design with async-first patterns (Tokio)
2. **Error Handling**: 17 error types with context-rich messages
3. **Type System**: 4 core enums (RiskLevel, RiskCategory, ConfidenceLevel, AlertLevel)
4. **Database Schema**: Normalized 3NF design with 10 tables
5. **CLI Framework**: 7 commands with clap derive
6. **Testing Infrastructure**: Unit, integration, doc, property-based, benchmarks
7. **CI/CD**: GitHub Actions with multi-platform matrix
8. **Documentation Suite**: 16 comprehensive docs (see [Documentation](#documentation))

---

## Quick Start

### Prerequisites

- **Rust**: 1.75 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **PostgreSQL**: 14+ (optional, SQLite supported for development)
- **Claude API Key**: Required for Phase 2+ (obtain from [Anthropic](https://www.anthropic.com))

### Installation

**Clone the repository:**

```bash
git clone https://github.com/doublegate/wargames-joshua.git
cd wargames-joshua
```

**Build the project:**

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

**Run tests:**

```bash
cargo test --all
```

**Check code quality:**

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

### Configuration

1. **Copy default configuration:**

```bash
cp config/default_config.toml config/local_config.toml
```

2. **Set your Claude API key** (for Phase 2+):

```bash
export ANTHROPIC_API_KEY="your-api-key-here"
```

3. **Configure database connection** in `config/local_config.toml`:

```toml
[database]
url = "postgresql://user:password@localhost/joshua"
# Or use SQLite for development:
# url = "sqlite://joshua.db"
```

### Basic Usage

```bash
# System diagnostics
cargo run --release -- diagnose

# Interactive mode (WarGames-style TUI)
cargo run --release -- interactive

# Initialize database (when implemented in Phase 1+)
cargo run --release -- init-db

# Run risk assessment (when implemented in Phase 2+)
cargo run --release -- assess --output markdown

# View assessment history (when implemented)
cargo run --release -- history --count 10

# Analyze trends (when implemented)
cargo run --release -- trends --period 30days

# Run scenario simulation (when implemented)
cargo run --release -- simulate --scenario cuban-missile-crisis

# View help
cargo run --release -- --help
```

---

## Documentation

Complete documentation suite with **25,769 lines** across 16 core documents:

### Core Specifications

- **[00 - Documentation Index](docs/00_Documentation_Index.md)** (717 lines) - Navigation guide and version tracking
- **[01 - Development Roadmap](docs/01_Development_Roadmap_and_Sprint_Planning.md)** (1,855 lines) - 40-week development plan with 6 phases
- **[02 - Risk Calculation Methodology](docs/02_Risk_Calculation_Methodology.md)** (1,451 lines) - Statistical models, Bayesian methods, Monte Carlo
- **[03 - Data Collection](docs/03_Data_Collection_and_Source_Integration.md)** (1,216 lines) - Multi-source integration strategies
- **[04 - Testing & QA](docs/04_Testing_and_Quality_Assurance_Plan.md)** (1,352 lines) - Comprehensive testing strategy (95%+ coverage)
- **[05 - Database Design](docs/05_Database_Design_and_Schema.md)** (1,068 lines) - PostgreSQL schema with 10 tables
- **[06 - Architecture Guide](docs/06_Architecture_and_Implementation_Guide.md)** (1,549 lines) - System design and implementation patterns

### Operations & Security

- **[07 - Deployment](docs/07_Deployment_and_Operations_Guide.md)** (1,730 lines) - Production deployment (Docker, K8s, systemd)
- **[08 - Security Specifications](docs/08_Security_Implementation_Specifications.md)** (1,576 lines) - Security architecture and hardening
- **[09 - API Reference](docs/09_API_Reference.md)** (1,664 lines) - Complete REST API specifications
- **[10 - Claude Integration](docs/10_Claude_Integration_Specifications.md)** (2,051 lines) ⭐ **CRITICAL** - AI integration guide
- **[11 - Monitoring & Alerting](docs/11_Monitoring_and_Alerting.md)** (1,400 lines) - Observability (Prometheus, Grafana)
- **[12 - Disaster Recovery](docs/12_Disaster_Recovery_and_Business_Continuity.md)** (974 lines) - DR/BC procedures

### User & Developer Guides

- **[13 - User Documentation](docs/13_User_Documentation.md)** (2,486 lines) - End-user guide with examples
- **[14 - Contributing Guide](docs/14_Contributing_Guide.md)** (1,936 lines) - Developer contribution workflow
- **[15 - Performance Optimization](docs/15_Performance_Optimization_Guide.md)** (2,274 lines) - Profiling and optimization strategies

### Additional Resources

- **[PHASE0_CLOSURE_REPORT.md](PHASE0_CLOSURE_REPORT.md)** - Comprehensive Phase 0 completion report
- **[ref-docs/WarGames-joshua_AppSpec.md](ref-docs/WarGames-joshua_AppSpec.md)** - Original application specification (1,300+ lines)
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and release notes
- **[SECURITY.md](SECURITY.md)** - Security policy and vulnerability reporting
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines

---

## Architecture

### System Overview

```
╔═══════════════════════════════════════════════════════════════════════╗
║                      CLI Entry Point (main.rs)                        ║
║   joshua assess | history | trends | simulate | interactive | ...    ║
╚═════════════════════════════╤═════════════════════════════════════════╝
                              │
                  ┌───────────▼──────────┐
                  │  WarGamesSystem      │
                  │  Core Orchestrator   │
                  └───────────┬──────────┘
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
  ┏━━━━━━━▼━━━━━━━┓   ┏━━━━━━▼━━━━━━━┓   ┏━━━━━━▼━━━━━━━┓
  ┃  Data         ┃   ┃  Claude       ┃   ┃  Risk         ┃
  ┃  Collection   ┃───┃  Analysis     ┃───┃  Calculation  ┃
  ┃  Engine       ┃   ┃  Engine       ┃   ┃  Engine       ┃
  ┗━━━━━━━┬━━━━━━━┛   ┗━━━━━━━━━━━━━━━┛   ┗━━━━━━━┬━━━━━━━┛
          │                                        │
          │          ┏━━━━━━━━━━━━━━━┓            │
          └──────────┃   Database    ┃◀───────────┘
                     ┃   Engine      ┃
                     ┗━━━━━━━┬━━━━━━━┛
                             │
              ┌──────────────┴──────────────┐
              │                             │
      ┏━━━━━━━▼━━━━━━━┓            ┏━━━━━━━▼━━━━━━━┓
      ┃ Visualization ┃            ┃  Notification ┃
      ┃ & Reporting   ┃            ┃  & Alerting   ┃
      ┗━━━━━━━━━━━━━━━┛            ┗━━━━━━━━━━━━━━━┛
```

### Core Components

1. **Data Collection Engine** (`src/engines/data_collection.rs`)
   - Parallel collection from 10+ diverse sources
   - Rate limiting (exponential backoff, 6-hour cache TTL)
   - Source reliability scoring (weighted aggregation)

2. **Claude Analysis Engine** (`src/engines/claude_integration.rs`) ⭐ **CRITICAL**
   - AI-powered risk assessment via Anthropic API
   - Ensemble consensus (3-5 independent analyses)
   - Contextual memory and multi-turn dialogue

3. **Risk Calculation Engine** (`src/engines/risk_calculation.rs`)
   - Multi-factor weighted scoring (50+ factors, 8 categories)
   - Bayesian adjustment with historical priors
   - Monte Carlo simulation (10,000+ iterations)

4. **Visualization Engine** (`src/visualizers/`)
   - Plotters-based chart generation
   - Doomsday Clock (ASCII art + graphical)
   - Heat maps, trend charts, risk matrices

5. **Report Generation Engine**
   - Markdown/HTML/PDF reports with embedded visualizations
   - Templated output with customizable formatting

6. **Database Engine** (`src/engines/database.rs`)
   - PostgreSQL with SQLx (type-safe queries)
   - 10-table normalized schema (3NF)
   - Historical tracking and time-series storage

7. **Terminal UI** (`src/cli/`)
   - Retro WarGames-style interface (ratatui + crossterm)
   - Interactive mode with typewriter effects
   - Amber/green terminal themes

8. **Notification Engine** (`src/engines/notification.rs`)
   - Multi-channel alerts (email, webhooks, terminal)
   - Threshold-based triggering
   - Alert escalation policies

### Technology Stack

- **Language**: Rust (edition 2021)
- **Runtime**: Tokio (async I/O)
- **Database**: SQLx with PostgreSQL/SQLite
- **HTTP Client**: reqwest with connection pooling
- **CLI**: clap with derive features
- **Terminal UI**: ratatui + crossterm
- **Visualization**: plotters + resvg
- **Testing**: proptest (property-based), criterion (benchmarks), mockall (mocks)
- **Error Handling**: thiserror
- **Serialization**: serde with JSON/TOML

---

## Development

### Setup Development Environment

1. **Clone the repository:**

```bash
git clone https://github.com/doublegate/wargames-joshua.git
cd wargames-joshua
```

2. **Install dependencies:**

```bash
# Rust toolchain (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# PostgreSQL (optional for development)
# macOS:
brew install postgresql@14
brew services start postgresql@14

# Ubuntu/Debian:
sudo apt-get install postgresql-14

# Fedora:
sudo dnf install postgresql-server postgresql-contrib
sudo postgresql-setup --initdb
sudo systemctl start postgresql
```

3. **Configure environment:**

```bash
cp config/default_config.toml config/local_config.toml
# Edit config/local_config.toml with your settings

# Set API key (for Phase 2+)
export ANTHROPIC_API_KEY="your-api-key-here"
```

4. **Initialize database:**

```bash
# Create database
createdb joshua

# Run migrations (when implemented)
cargo run -- init-db
```

5. **Run tests:**

```bash
cargo test --all
```

6. **Run locally:**

```bash
cargo run -- diagnose
```

### Project Structure

```
wargames-joshua/
├── .github/                  # GitHub configuration
│   ├── ISSUE_TEMPLATE/       # 5 issue templates
│   ├── workflows/            # CI/CD (ci.yml)
│   ├── FUNDING.yml           # GitHub Sponsors
│   └── PULL_REQUEST_TEMPLATE.md
├── benches/                  # Criterion benchmarks
│   └── risk_calculation.rs   # Performance benchmarks
├── config/                   # Configuration files
│   └── default_config.toml   # Default configuration
├── docs/                     # Documentation (16 core docs)
│   ├── 00_Documentation_Index.md
│   ├── 01-15_*.md            # Core documentation files
│   └── PHASE0_Verification.md
├── migrations/               # Database migrations (SQLx)
│   ├── 20250101000001_initial_schema.sql
│   ├── 20250101000002_risk_factors.sql
│   └── 20250101000003_collected_data.sql
├── ref-docs/                 # Reference materials
│   ├── Nuclear Exchange Survival Guide.md
│   ├── Nuclear Precipice.md
│   ├── WarGames-joshua_AppSpec.md
│   └── WarGames-joshua_DIAGRAMS.md
├── src/                      # Rust source code (2,899 lines)
│   ├── analyzers/            # Risk analysis modules
│   │   └── mod.rs            # RiskAnalysis trait
│   ├── cli/                  # Command-line interface
│   │   └── mod.rs            # Clap commands
│   ├── collectors/           # Data collection modules
│   │   └── mod.rs            # DataCollector trait
│   ├── engines/              # Core engine implementations
│   │   ├── claude_integration.rs  # Claude AI engine
│   │   ├── data_collection.rs     # Data collection engine
│   │   ├── database.rs            # Database engine
│   │   ├── notification.rs        # Notification engine
│   │   ├── risk_calculation.rs    # Risk calculation engine
│   │   └── mod.rs
│   ├── models/               # Data models
│   │   ├── assessment.rs     # Assessment model
│   │   ├── data_point.rs     # DataPoint model
│   │   ├── risk_factor.rs    # RiskFactor model
│   │   └── mod.rs
│   ├── utils/                # Utility functions
│   │   ├── config.rs         # Configuration loading
│   │   ├── logging.rs        # Logging initialization
│   │   └── mod.rs
│   ├── visualizers/          # Visualization modules
│   │   └── mod.rs            # Visualization trait
│   ├── constants.rs          # System constants
│   ├── error.rs              # Error types (17 variants)
│   ├── types.rs              # Type definitions
│   ├── lib.rs                # Library root
│   └── main.rs               # Application entry point
├── tests/                    # Integration tests
│   └── integration_test.rs   # E2E tests
├── AUTHORS.md                # Contributors list
├── CHANGELOG.md              # Version history
├── CLAUDE.md                 # Claude Code guidance
├── CODE_OF_CONDUCT.md        # Community standards
├── CONTRIBUTING.md           # Contribution guidelines
├── Cargo.toml                # Rust package manifest
├── Cargo.lock                # Dependency lock
├── LICENSE-MIT               # MIT license
├── LICENSE-APACHE            # Apache 2.0 license
├── README.md                 # This file
├── SECURITY.md               # Security policy
└── WARP.md                   # Workspace guidance
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check

# Build documentation
cargo doc --open
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Audit dependencies
cargo audit

# Generate documentation
cargo doc --open
```

---

## Testing

### Run All Tests

```bash
cargo test --all
```

### Run Specific Test Types

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_test

# Doc tests only
cargo test --doc

# With output
cargo test -- --nocapture

# Run specific test
cargo test test_risk_factor_creation
```

### Code Coverage

```bash
# Install tarpaulin (one-time)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir target/coverage

# Open coverage report
open target/coverage/index.html
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench risk_calculation

# Generate flamegraph
cargo bench --bench risk_calculation -- --profile-time=5
```

### Current Test Status

- **Total Tests**: 25
- **Unit Tests**: 16 (100% passing)
- **Integration Tests**: 7 (100% passing)
- **Doc Tests**: 2 (100% passing)
- **Pass Rate**: 100%
- **Coverage**: ~95% (estimated, Phase 0 scope)

---

## Contributing

We welcome contributions! Please see our **[Contributing Guide](CONTRIBUTING.md)** for details.

### Quick Contribution Workflow

1. **Fork the repository**

```bash
gh repo fork doublegate/wargames-joshua --clone
```

2. **Create a feature branch**

```bash
git checkout -b feat/amazing-feature
```

3. **Make your changes**

4. **Run tests and quality checks**

```bash
cargo test --all
cargo clippy -- -D warnings
cargo fmt
```

5. **Commit your changes** (follow [Conventional Commits](https://www.conventionalcommits.org/))

```bash
git commit -m 'feat: add amazing feature'
```

6. **Push to your fork**

```bash
git push origin feat/amazing-feature
```

7. **Open a Pull Request**

```bash
gh pr create --title "feat: add amazing feature" --body "Description of changes"
```

### Development Phases & Contribution Opportunities

**Current Phase**: Phase 0 (Foundation) ✅ **COMPLETE**

**Next Phase**: **Phase 1** (Data Collection Engine) - Great opportunity to contribute!

See [Development Roadmap](docs/01_Development_Roadmap_and_Sprint_Planning.md) for detailed phase breakdown.

### Contribution Areas

- **Phase 1**: Data collectors (RSS, News APIs, Think Tanks)
- **Phase 2**: Claude AI integration and prompt engineering
- **Phase 3**: Risk calculation algorithms (Bayesian, Monte Carlo)
- **Phase 4**: Visualization and reporting
- **Documentation**: Improve docs, add examples, fix typos
- **Testing**: Add tests, improve coverage, property-based testing
- **Performance**: Optimize hot paths, reduce allocations
- **Security**: Security audits, vulnerability fixes

### Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

---

## License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Acknowledgments

- **WarGames (1983 film)**: Inspiration for project name, aesthetic, and JOSHUA persona
- **Bulletin of Atomic Scientists**: Doomsday Clock framework and risk assessment methodology
- **Anthropic**: Claude AI API for advanced risk analysis capabilities
- **Rust Community**: Excellent ecosystem, tools, and support
- **Open Source Contributors**: All future contributors to this project

### Inspiration & Context

This project was inspired by the 1983 film *WarGames*, where the WOPR (War Operation Plan Response) supercomputer simulates global thermonuclear war scenarios. The famous quote captures the essence of nuclear deterrence:

> *"A strange game. The only winning move is not to play. How about a nice game of chess?"* — WOPR

In 2025, with the Doomsday Clock at 89 seconds to midnight, continuous monitoring of nuclear risk factors is more critical than ever.

### Ethical Considerations

This system is designed to **monitor and assess nuclear war risk** to promote awareness and potentially prevent catastrophic outcomes. It is **not** intended to:

- Glorify or encourage nuclear weapons development
- Provide classified or sensitive information
- Serve as the sole basis for policy decisions
- Replace qualified expert analysis

**Principles**:
- **Scientific Rigor**: Data-driven analysis with transparent methodology
- **Transparency**: Open-source code and documentation
- **Responsible Disclosure**: Ethical reporting of findings
- **Ethical AI Use**: Responsible use of AI for beneficial purposes
- **Peace Advocacy**: Ultimate goal of reducing nuclear war risk

### Disclaimer

This is an **educational and monitoring tool**. It does not have access to classified information and should not be used as the sole basis for policy decisions. Always consult with qualified experts in nuclear security, international relations, and military strategy.

---

## Contact & Support

### Getting Help

- **Issues**: [GitHub Issues](https://github.com/doublegate/wargames-joshua/issues)
- **Discussions**: [GitHub Discussions](https://github.com/doublegate/wargames-joshua/discussions)
- **Documentation**: [docs/](docs/) directory

### Reporting Issues

- **Bugs**: Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.yml)
- **Feature Requests**: Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.yml)
- **Documentation**: Use the [Documentation template](.github/ISSUE_TEMPLATE/documentation.yml)

### Security Vulnerabilities

Please report security vulnerabilities **privately** to the maintainers. See [SECURITY.md](SECURITY.md) for responsible disclosure procedures.

**Do NOT** create public GitHub issues for security vulnerabilities.

---

## Project Status & Roadmap

### Current Status

**Phase 0** (Foundation & Architecture): ✅ **COMPLETE** (100%)

- Comprehensive documentation (25,769 lines)
- Production-ready architecture (2,899 lines Rust)
- Robust test framework (25 tests, 100% passing)
- CI/CD pipeline with multi-platform support

### Next Steps

1. **Phase 1** (Data Collection Engine) - Weeks 5-10
   - Implement RSS feed aggregation
   - Integrate news APIs (Reuters, AP, BBC)
   - Add think tank data collection (SIPRI, Carnegie, RAND)
   - Build real-time monitoring system

2. **Phase 2** (Claude Analysis Engine) - Weeks 11-16 ⭐ **CRITICAL**
   - Integrate Claude API (Anthropic)
   - Develop advanced prompt engineering
   - Implement ensemble consensus building
   - Add cost optimization strategies

3. **Continue through Phase 6** (40-week plan)

See [Development Roadmap](docs/01_Development_Roadmap_and_Sprint_Planning.md) for complete timeline.

---

**Current Doomsday Clock: 89 seconds to midnight** (January 2025)

*"SHALL WE PLAY A GAME?"* — JOSHUA

---

**End of README**

*WarGames/JOSHUA v0.1.0 - Foundation Complete*
