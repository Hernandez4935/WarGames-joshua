# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**WarGames/JOSHUA** is a comprehensive nuclear war risk assessment system inspired by the 1983 film. The project aims to create a sophisticated Rust-based application that monitors global nuclear threats using AI-powered analysis (Claude API), multi-source data aggregation, and statistical modeling.

**Status**: Planning/Design Phase - No implementation code yet, only comprehensive documentation and specifications.

**Language**: Rust (planned)
**Command**: `joshua` (planned CLI name)
**Version**: 1.0.0 (target)

## Repository Structure

This is a **planning and documentation repository** containing:

- `ref-docs/` - Reference materials including application specifications, diagrams, and nuclear risk context
- `docs/` - Detailed planning documents covering development roadmap, risk methodology, data collection, testing, database design, and architecture

**IMPORTANT**: This repository contains NO actual implementation code yet. All content is planning, specifications, and design documentation.

## Key Documentation Files

### Primary Specifications
- `ref-docs/WarGames-joshua_AppSpec.md` - Complete 1,300+ line application specification with architecture, features, and implementation details
- `docs/06_Architecture_and_Implementation_Guide.md` - Technical architecture, module organization, and implementation patterns
- `docs/01_Development_Roadmap_and_Sprint_Planning.md` - 6-phase development roadmap with 40-week timeline

### Technical Details
- `docs/02_Risk_Calculation_Methodology.md` - Statistical foundations, Bayesian modeling, Monte Carlo simulation
- `docs/03_Data_Collection_and_Source_Integration.md` - Multi-source data aggregation strategies
- `docs/04_Testing_and_Quality_Assurance_Plan.md` - Comprehensive testing strategy
- `docs/05_Database_Design_and_Schema.md` - PostgreSQL schema and data models

## Architecture Overview

### System Components (Planned)
```
WarGamesSystem
├── DataCollectionEngine     - Multi-source aggregation (news, think tanks, government)
├── ClaudeAnalysisEngine     - AI-powered risk analysis via Anthropic API
├── RiskCalculationEngine    - Weighted scoring, Bayesian adjustment, Monte Carlo
├── VisualizationEngine      - Doomsday Clock, trends, heat maps, charts
├── ReportGenerationEngine   - Markdown/HTML/PDF reports
├── PersistentStorageEngine  - PostgreSQL database
├── AlertNotificationEngine  - Multi-channel alerts
├── TaskSchedulingEngine     - Automated assessments
└── TerminalInterfaceEngine  - Retro-style TUI (WarGames-inspired)
```

### Technology Stack (Planned)
- **Runtime**: Tokio async runtime
- **HTTP**: reqwest, axum
- **Database**: SQLx with PostgreSQL/SQLite
- **CLI**: clap with derive features
- **Visualization**: plotters, resvg
- **Terminal UI**: ratatui, crossterm
- **Testing**: proptest, criterion, mockall

## Development Phases

The project follows a 6-phase development plan:

1. **Phase 0 (Weeks 1-4)**: Foundation & Architecture
2. **Phase 1 (Weeks 5-10)**: Data Collection Engine
3. **Phase 2 (Weeks 11-16)**: Claude Analysis Engine
4. **Phase 3 (Weeks 17-22)**: Risk Calculation & Modeling
5. **Phase 4 (Weeks 23-28)**: Visualization & Reporting
6. **Phase 5 (Weeks 29-34)**: Integration & Testing
7. **Phase 6 (Weeks 35-40)**: Production Readiness & Launch

## Risk Assessment Methodology

### Core Approach
- **Multi-factor weighted scoring** - 50+ risk factors across 7 categories
- **Bayesian adjustment** - Historical correlations and conditional probabilities
- **Monte Carlo simulation** - 10,000+ iterations for uncertainty quantification
- **Time-series analysis** - Trend detection with Mann-Kendall tests
- **Pattern recognition** - Historical parallel identification (Cuban Missile Crisis, 1983 false alarm, etc.)

### Risk Categories (with weights)
1. Nuclear Arsenal Changes (15%)
2. Doctrine and Posture (15%)
3. Regional Conflicts (20%)
4. Leadership & Rhetoric (10%)
5. Technical Incidents (15%)
6. Communication Breakdown (10%)
7. Emerging Technology (10%)
8. Economic Factors (5%)

## Key Implementation Patterns

### Trait-Based Design
```rust
#[async_trait]
pub trait DataCollector: Send + Sync {
    async fn collect(&self) -> Result<Vec<DataPoint>>;
    fn source_name(&self) -> &str;
    fn reliability_score(&self) -> f64;
}
```

### Error Handling
- Uses `thiserror` for comprehensive error types
- Context-rich errors with source chains
- Graceful degradation for non-critical failures

### Testing Strategy
- 95%+ test coverage target
- Unit, integration, and E2E tests
- Property-based testing with `proptest`
- Performance benchmarks with `criterion`
- Chaos testing for resilience

## Data Sources (Planned)

### News & Media
- Reuters API, AP, BBC, Al Jazeera, RT, Xinhua

### Research Institutions
- SIPRI (Stockholm International Peace Research Institute)
- Carnegie Endowment, RAND, Bulletin of Atomic Scientists
- Arms Control Association, Chatham House

### Government Sources
- State Department reports, IAEA, UN Security Council

### Social Media Intelligence
- Twitter/X geopolitical monitoring, Reddit (r/worldnews, r/geopolitics)

## Claude API Integration

### Analysis Workflow
1. Multi-source data collection and aggregation
2. Claude API call with structured prompts (claude-sonnet-4+)
3. JSON response parsing with validation
4. Consensus building from 3-5 independent analyses
5. Risk score calculation with confidence intervals

### System Prompt
Instructs Claude to act as JOSHUA, maintaining objectivity and using the Bulletin of Atomic Scientists framework. Current baseline: 89 seconds to midnight (January 2025).

## Terminal UI Aesthetic

Retro WarGames-inspired interface with:
- Typewriter text effects
- Amber/green terminal themes
- ASCII art Doomsday Clock
- Interactive TUI with ratatui
- Famous greeting: "GREETINGS PROFESSOR FALKEN. SHALL WE PLAY A GAME?"

## Commands (Planned CLI)

```bash
joshua assess [--force] [--output FORMAT] [--interactive]
joshua history [--count N] [--from DATE] [--to DATE]
joshua trends [--period PERIOD] [--factors LIST]
joshua simulate --scenario NAME [--iterations N]
joshua schedule --cron EXPR [--enable]
joshua interactive  # Full TUI mode
joshua diagnose     # System health check
```

## Important Notes for Development

1. **No Code Yet**: This repository is currently documentation-only. When implementing:
   - Follow the architecture specified in `docs/06_Architecture_and_Implementation_Guide.md`
   - Use the module structure defined in the AppSpec
   - Implement trait-based extensibility patterns

2. **Security**: API keys must be encrypted at rest, audit logging required, input validation essential

3. **Performance Targets**:
   - Complete assessment: <5 minutes
   - Memory usage: <500MB
   - Database queries: optimized with proper indexes
   - API response caching: 6-hour TTL

4. **Ethical Considerations**: This system monitors nuclear war risk. Development should prioritize accuracy, scientific rigor, and responsible use.

## When Starting Implementation

1. Create Cargo workspace structure per specifications
2. Set up PostgreSQL schema from `docs/05_Database_Design_and_Schema.md`
3. Implement core traits and error types first
4. Build mock data generators for testing
5. Follow TDD approach with comprehensive test coverage
6. Reference the 40-week roadmap for sprint planning

## Context

Inspired by the Bulletin of Atomic Scientists' Doomsday Clock (currently 89 seconds to midnight as of January 2025), this system aims to provide continuous, AI-powered nuclear risk monitoring through data-driven analysis and visualization.

The famous quote from WarGames applies: *"A strange game. The only winning move is not to play. How about a nice game of chess?"*
