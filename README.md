# WarGames/JOSHUA: Global Nuclear War Risk Assessment System

> *"The only winning move is not to play. How about a nice game of chess?"*

[![CI](https://github.com/yourusername/wargames-joshua/workflows/CI/badge.svg)](https://github.com/yourusername/wargames-joshua/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

A comprehensive nuclear war risk assessment system that leverages AI-powered analysis through Claude (Anthropic) to perform periodic, detailed evaluations of global nuclear threats. Inspired by the 1983 film *WarGames* and developed in response to the current geopolitical reality where the Doomsday Clock stands at 89 seconds to midnight.

## ⚠️ Project Status: Phase 0 Complete (Foundation)

**Current Version:** v0.1.0 (Phase 0 - Foundation & Architecture)

Phase 0 (Foundation & Architecture) is now **COMPLETE**. The project has established:

✅ Complete Rust project structure with proper module organization
✅ Core architectural patterns and trait-based abstractions
✅ Comprehensive error handling system
✅ Database schema design and migrations
✅ Configuration management system
✅ Logging infrastructure
✅ Testing framework with example tests
✅ CI/CD pipeline with GitHub Actions

**Next Steps:** Implementation of Phase 1 (Data Collection Engine) - Weeks 5-10

## 🎯 Overview

WarGames/JOSHUA combines:

- **Multi-source Data Collection**: News APIs, think tanks, government reports, social media
- **AI-Powered Analysis**: Claude/Anthropic API for consistent, deep risk assessment
- **Historical Tracking**: Persistent database of risk trends over time
- **Advanced Visualization**: Trend charts, heat maps, risk matrices, Doomsday Clock
- **Actionable Intelligence**: Clear recommendations and early warning indicators

## 🏗️ Architecture

```
┌─────────────┐   ┌──────────────┐   ┌───────────────┐   ┌──────────────┐
│   Collect   │──▶│   Analyze    │──▶│   Calculate   │──▶│  Visualize   │
│    Data     │   │   (Claude)   │   │     Risk      │   │  & Report    │
└─────────────┘   └──────────────┘   └───────────────┘   └──────────────┘
```

### Core Components

- **Data Collection Engine**: Parallel collection from 10+ diverse sources
- **Claude Analysis Engine**: AI-powered risk assessment with contextual memory
- **Risk Calculation Engine**: Multi-factor weighted scoring with Bayesian adjustment
- **Visualization Engine**: Comprehensive charts using Plotters
- **Report Generator**: Markdown/HTML/PDF reports with embedded visualizations
- **Database Engine**: PostgreSQL with SQLx for historical tracking
- **Terminal UI**: Retro WarGames-style interface with ratatui

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (`rustup update`)
- PostgreSQL 14+ (optional for development)
- Anthropic API key (for Claude integration)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/wargames-joshua.git
cd wargames-joshua

# Build the project
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

### Configuration

1. Copy the default configuration:
```bash
cp config/default_config.toml config/local_config.toml
```

2. Set your Anthropic API key:
```bash
export ANTHROPIC_API_KEY="your-api-key-here"
```

3. Configure database connection in `config/local_config.toml`

### Usage

```bash
# Run system diagnostics
cargo run -- diagnose

# Initialize database (when implemented)
cargo run -- init-db

# Run risk assessment (when implemented)
cargo run -- assess

# View assessment history (when implemented)
cargo run -- history --count 10

# Interactive mode (when implemented)
cargo run -- interactive
```

## 📖 Documentation

- **[Development Roadmap](docs/01_Development_Roadmap_and_Sprint_Planning.md)** - Complete project timeline
- **[Architecture Guide](docs/06_Architecture_and_Implementation_Guide.md)** - System design and patterns
- **[Database Design](docs/05_Database_Design_and_Schema.md)** - Schema specifications
- **[Risk Methodology](docs/02_Risk_Calculation_Methodology.md)** - Risk calculation approach
- **[Data Sources](docs/03_Data_Collection_and_Source_Integration.md)** - Integration specifications
- **[Testing Plan](docs/04_Testing_and_Quality_Assurance_Plan.md)** - QA strategy
- **[Security Specs](docs/08_Security_Implementation_Specifications.md)** - Security measures

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_risk_factor_creation

# Run integration tests only
cargo test --test integration_test

# Generate code coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## 📊 Project Structure

```
wargames-joshua/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── cli/                 # Command-line interface
│   ├── collectors/          # Data collectors
│   ├── analyzers/           # Risk analyzers
│   ├── engines/             # Processing engines
│   ├── models/              # Data models
│   ├── visualizers/         # Visualization generators
│   ├── utils/               # Utilities
│   ├── error.rs             # Error types
│   ├── types.rs             # Type definitions
│   └── constants.rs         # System constants
├── tests/                   # Integration tests
├── migrations/              # Database migrations
├── config/                  # Configuration files
└── docs/                    # Documentation
```

## 🛠️ Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linters
5. Commit your changes (`git commit -m 'feat: add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📅 Development Roadmap

### Phase 0: Foundation & Architecture ✅ **COMPLETE**
- [x] Project setup and module structure
- [x] Core architectural patterns
- [x] Error handling and type system
- [x] Database schema design
- [x] Configuration management
- [x] Testing framework

### Phase 1: Data Collection Engine (Weeks 5-10)
- [ ] RSS feed aggregation
- [ ] News API integration
- [ ] Research institution data collection
- [ ] Real-time monitoring

### Phase 2: Claude Analysis Engine (Weeks 11-16)
- [ ] Claude API integration
- [ ] Prompt engineering
- [ ] Response parsing and validation
- [ ] Multi-analysis consensus

### Phase 3: Risk Calculation & Modeling (Weeks 17-22)
- [ ] Multi-factor risk calculation
- [ ] Monte Carlo simulation
- [ ] Historical pattern recognition

### Phase 4: Visualization & Reporting (Weeks 23-28)
- [ ] Doomsday Clock visualization
- [ ] Trend charts and heat maps
- [ ] Report generation (MD/HTML/PDF)
- [ ] Interactive terminal UI

### Phase 5: Integration & Testing (Weeks 29-34)
- [ ] End-to-end integration
- [ ] Comprehensive testing (95%+ coverage)
- [ ] Documentation completion

### Phase 6: Production Readiness (Weeks 35-40)
- [ ] Security hardening
- [ ] Performance optimization
- [ ] Production deployment

## 🔒 Security

- API keys encrypted at rest
- Rate limiting on all external API calls
- Input validation and sanitization
- Audit logging of all operations
- SQL injection prevention via prepared statements

## 📝 License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## 🙏 Acknowledgments

- Inspired by the 1983 film *WarGames*
- Bulletin of Atomic Scientists for the Doomsday Clock framework
- Anthropic for Claude AI capabilities
- The Rust community for excellent tooling

## ⚠️ Disclaimer

This is an educational and monitoring tool. It does not have access to classified information and should not be used as the sole basis for policy decisions. Always consult with qualified experts in nuclear security and international relations.

---

*"A strange game. The only winning move is not to play."* - WOPR, WarGames (1983)

**Current Doomsday Clock: 89 seconds to midnight** (as of January 2025)
