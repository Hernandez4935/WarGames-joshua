# WarGames/JOSHUA: Complete User Documentation
## End-User Guide for Nuclear Risk Assessment System
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Getting Started](#2-getting-started)
3. [CLI Command Reference](#3-cli-command-reference)
4. [Understanding Risk Assessments](#4-understanding-risk-assessments)
5. [Reading Reports](#5-reading-reports)
6. [Configuration](#6-configuration)
7. [FAQ and Troubleshooting](#7-faq-and-troubleshooting)
8. [Best Practices](#8-best-practices)
9. [API Usage](#9-api-usage)
10. [Glossary](#10-glossary)

---

## 1. Introduction

### 1.1 What is WarGames/JOSHUA?

WarGames/JOSHUA is an advanced nuclear war risk assessment system that continuously monitors global nuclear threats through AI-powered analysis, multi-source data aggregation, and statistical modeling. Named after the iconic 1983 film "WarGames," the system provides data-driven insights into nuclear war risk using the same framework as the Bulletin of Atomic Scientists' Doomsday Clock.

```
┌───────────────────────────────────────────────────────────────┐
│                    WarGames/JOSHUA System                     │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  Data Collection   →  AI Analysis   →  Risk Calculation      │
│  (50+ sources)        (Claude AI)       (Statistical)         │
│                                                               │
│                           ↓                                   │
│                                                               │
│  Visualization  ←  Report Generation  ←  Assessment          │
│  (Doomsday Clock)    (MD/HTML/PDF)        Complete           │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

### 1.2 Who Should Use This System?

**Primary Users:**
- Policy analysts and researchers studying nuclear risk
- Think tank researchers and academic institutions
- Government security analysts
- International relations professionals
- Journalists covering nuclear policy
- Risk management professionals

**Secondary Users:**
- Data scientists interested in risk modeling
- Developers integrating nuclear risk data
- Educational institutions teaching security studies

### 1.3 The Doomsday Clock Framework

The Bulletin of Atomic Scientists' Doomsday Clock represents how close humanity is to catastrophic destruction (midnight). The clock has stood at various times throughout history:

- **Current Time**: 89 seconds to midnight (January 2025)
- **Historical Range**: 2 minutes (1953, hydrogen bomb tests) to 17 minutes (1991, Cold War end)
- **Interpretation**: Closer to midnight = higher nuclear war risk

WarGames/JOSHUA uses this same framework to provide continuous risk monitoring.

### 1.4 System Capabilities

**Core Features:**
- Real-time nuclear risk assessment
- Multi-source data aggregation (news, government, think tanks)
- AI-powered analysis via Claude API (Anthropic)
- Statistical risk modeling (Bayesian networks, Monte Carlo simulation)
- Historical trend analysis
- Customizable alert notifications
- Comprehensive reporting (Markdown, HTML, PDF)
- Interactive terminal UI (WarGames-inspired aesthetic)

**Data Sources:**
- News media (Reuters, AP, BBC, Al Jazeera)
- Research institutions (SIPRI, Carnegie, RAND, Bulletin of Atomic Scientists)
- Government sources (State Department, IAEA, UN Security Council)
- Social media intelligence (Twitter/X geopolitical monitoring)

---

## 2. Getting Started

### 2.1 System Requirements

**Minimum Requirements:**
- Operating System: Linux (Ubuntu 20.04+), macOS (11+), or Windows (10+)
- CPU: 2 cores
- RAM: 2GB
- Disk Space: 5GB (for database and cached data)
- Internet Connection: Required for data collection

**Recommended Requirements:**
- Operating System: Linux (Ubuntu 22.04+) or macOS (12+)
- CPU: 4+ cores
- RAM: 8GB
- Disk Space: 20GB
- Internet: High-speed connection for faster data collection

**Software Dependencies:**
- Rust 1.75+ (for building from source)
- PostgreSQL 14+ (or SQLite for lightweight deployments)
- Redis (optional, for caching)
- Anthropic API key (required for AI analysis)

### 2.2 Installation

#### Option 1: Binary Installation (Recommended)

Download the latest release for your platform:

```bash
# Linux (x86_64)
curl -LO https://github.com/yourusername/wargames-joshua/releases/latest/download/joshua-linux-amd64
chmod +x joshua-linux-amd64
sudo mv joshua-linux-amd64 /usr/local/bin/joshua

# macOS (Apple Silicon)
curl -LO https://github.com/yourusername/wargames-joshua/releases/latest/download/joshua-darwin-arm64
chmod +x joshua-darwin-arm64
sudo mv joshua-darwin-arm64 /usr/local/bin/joshua

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/yourusername/wargames-joshua/releases/latest/download/joshua-windows-amd64.exe" -OutFile "joshua.exe"
Move-Item joshua.exe C:\Windows\System32\
```

Verify installation:
```bash
joshua --version
# Output: joshua 1.0.0
```

#### Option 2: Install via Cargo

```bash
cargo install wargames-joshua

# Verify
joshua --version
```

#### Option 3: Build from Source

```bash
# Clone repository
git clone https://github.com/yourusername/wargames-joshua.git
cd wargames-joshua

# Build release binary
cargo build --release

# Install to system
sudo cp target/release/joshua /usr/local/bin/

# Verify
joshua --version
```

#### Option 4: Docker

```bash
# Pull image
docker pull ghcr.io/yourusername/wargames-joshua:latest

# Run container
docker run -d \
  --name joshua \
  -e ANTHROPIC_API_KEY=your_key_here \
  -e DATABASE_URL=postgresql://user:pass@host:5432/joshua \
  -v joshua-data:/var/lib/joshua \
  -p 8080:8080 \
  ghcr.io/yourusername/wargames-joshua:latest

# Use CLI via docker exec
docker exec joshua joshua assess
```

### 2.3 Initial Configuration

#### Step 1: Initialize Database

```bash
# PostgreSQL (recommended)
createdb wargames_joshua
joshua init-db --connection postgresql://localhost:5432/wargames_joshua

# SQLite (lightweight)
joshua init-db --connection sqlite://joshua.db
```

Output:
```
Initializing WarGames/JOSHUA database...
✓ Creating tables
✓ Creating indexes
✓ Loading initial data
✓ Database initialized successfully

Database ready at: postgresql://localhost:5432/wargames_joshua
```

#### Step 2: Configure API Keys

Create a configuration file:

```bash
mkdir -p ~/.config/joshua
joshua config init

# Edit configuration
nano ~/.config/joshua/config.toml
```

**Configuration file structure:**

```toml
# ~/.config/joshua/config.toml

[general]
log_level = "info"
data_directory = "~/.local/share/joshua"

[database]
connection_string = "postgresql://localhost:5432/wargames_joshua"
pool_size = 10
connection_timeout = 30

[claude]
api_key = "sk-ant-api03-..."  # Get from https://console.anthropic.com
model = "claude-sonnet-4-20250514"
max_tokens = 16000
temperature = 0.7
timeout = 120

[data_collection]
enabled_sources = ["reuters", "sipri", "bulletin", "state_dept"]
cache_duration = 3600
max_parallel_collectors = 10
collection_timeout = 300

[notifications]
enabled = true
channels = ["email", "webhook"]
email_to = "alerts@example.com"
webhook_url = "https://example.com/webhooks/joshua"

[scheduling]
enabled = false
cron_expression = "0 0 * * *"  # Daily at midnight
```

**Alternative: Environment Variables**

```bash
export ANTHROPIC_API_KEY="sk-ant-api03-..."
export DATABASE_URL="postgresql://localhost:5432/wargames_joshua"
export JOSHUA_LOG_LEVEL="info"
```

#### Step 3: Verify Installation

Run a health check:

```bash
joshua diagnose
```

Expected output:
```
WarGames/JOSHUA System Diagnostics
================================================================================

System Information:
  Version:           1.0.0
  Platform:          linux-x86_64
  Rust Version:      1.75.0

Configuration:
  ✓ Config file found:      ~/.config/joshua/config.toml
  ✓ Data directory exists:  ~/.local/share/joshua
  ✓ Log level:              info

Database:
  ✓ Connection successful:  postgresql://localhost:5432/wargames_joshua
  ✓ Schema version:         1.0.0
  ✓ Tables:                 12
  ✓ Historical records:     0 assessments

Claude API:
  ✓ API key configured:     sk-ant-api03-***
  ✓ Connection successful:  https://api.anthropic.com
  ✓ Model:                  claude-sonnet-4-20250514

Data Sources:
  ✓ Reuters RSS:            accessible
  ✓ SIPRI Database:         accessible
  ✓ Bulletin:               accessible
  ✓ State Dept:             accessible
  ⚠ Twitter API:            not configured (optional)

Notifications:
  ✓ Email:                  configured
  ✓ Webhook:                configured
  ✗ Slack:                  not configured

Overall Status: READY
================================================================================
```

### 2.4 First Assessment Walkthrough

Let's run your first nuclear risk assessment:

#### Step 1: Start Interactive Assessment

```bash
joshua assess --interactive
```

#### Step 2: Watch the Process

```
╔══════════════════════════════════════════════════════════════════════════╗
║                      WarGames/JOSHUA Assessment                          ║
║                  Nuclear War Risk Evaluation System                      ║
╚══════════════════════════════════════════════════════════════════════════╝

GREETINGS PROFESSOR FALKEN.

SHALL WE PLAY A GAME?

[Initializing system...]

Phase 1: Data Collection
────────────────────────────────────────────────────────────────────────────
[████████████████████████████████████████] 100%

  ✓ Reuters Nuclear News Feed        (127 articles)
  ✓ SIPRI Arsenal Database            (9 countries)
  ✓ Bulletin of Atomic Scientists     (15 updates)
  ✓ State Department Reports          (8 documents)
  ✓ Carnegie Endowment Analysis       (12 papers)

  Collected: 171 data points in 47 seconds

Phase 2: AI Analysis (Claude)
────────────────────────────────────────────────────────────────────────────
[████████████████████████████████████████] 100%

  ✓ Analyzing 171 data points with Claude AI
  ✓ Extracting risk factors
  ✓ Building consensus (3/3 analyses complete)
  ✓ Calculating confidence levels

  Identified: 43 risk factors in 89 seconds

Phase 3: Risk Calculation
────────────────────────────────────────────────────────────────────────────
[████████████████████████████████████████] 100%

  ✓ Weighted scoring across 8 categories
  ✓ Bayesian network adjustment
  ✓ Monte Carlo simulation (10,000 iterations)
  ✓ Trend analysis (comparing to 30-day history)

  Calculated: Risk score in 12 seconds

Phase 4: Report Generation
────────────────────────────────────────────────────────────────────────────
[████████████████████████████████████████] 100%

  ✓ Doomsday Clock visualization
  ✓ Trend charts
  ✓ Risk factor heatmap
  ✓ Markdown report
  ✓ HTML report
  ✓ PDF report

  Generated: 3 reports in 8 seconds

════════════════════════════════════════════════════════════════════════════
                           ASSESSMENT COMPLETE
════════════════════════════════════════════════════════════════════════════

Current Risk Level: 91 seconds to midnight
Risk Change:        +2 seconds (INCREASED from previous)
Confidence Level:   High
Trend Direction:    Increasing

Assessment ID:      asmt_2025_10_27_120000
Assessment Date:    2025-10-27 12:00:00 UTC
Total Duration:     156 seconds

Reports Available:
  - Markdown: ~/.local/share/joshua/reports/assessment_20251027_120000.md
  - HTML:     ~/.local/share/joshua/reports/assessment_20251027_120000.html
  - PDF:      ~/.local/share/joshua/reports/assessment_20251027_120000.pdf

View report: joshua report --latest
View trends: joshua trends --period 30d

════════════════════════════════════════════════════════════════════════════
```

#### Step 3: Review the Report

```bash
# View in terminal
joshua report --latest --format text

# Open HTML in browser
joshua report --latest --open

# Or manually open
open ~/.local/share/joshua/reports/assessment_20251027_120000.html
```

**Congratulations!** You've completed your first nuclear risk assessment.

---

## 3. CLI Command Reference

### 3.1 `joshua assess` - Run Risk Assessment

**Purpose:** Execute a complete nuclear war risk assessment.

**Usage:**
```bash
joshua assess [OPTIONS]
```

**Options:**
- `--force` - Force new assessment even if recent one exists
- `--output <FORMAT>` - Output format: text, json, quiet (default: text)
- `--interactive` - Show interactive progress display
- `--no-cache` - Disable data collection caching
- `--dry-run` - Simulate assessment without storing results
- `--sources <LIST>` - Comma-separated list of data sources to use
- `--model <MODEL>` - Override Claude model (e.g., claude-opus-4)

**Examples:**

```bash
# Standard assessment
joshua assess

# Force new assessment with interactive display
joshua assess --force --interactive

# Assessment using only specific sources
joshua assess --sources reuters,sipri,bulletin

# Dry-run with JSON output
joshua assess --dry-run --output json

# Use higher-quality Claude model
joshua assess --model claude-opus-4-20250514
```

**Output Interpretation:**

```
Current Risk Level: 91 seconds to midnight
```
- **60-90 seconds**: Extreme risk (immediate concerns)
- **90-180 seconds**: Very high risk (significant tensions)
- **180-360 seconds**: High risk (elevated concerns)
- **360-720 seconds**: Moderate risk (baseline monitoring)
- **720+ seconds**: Low risk (relative stability)

```
Risk Change: +2 seconds (INCREASED from previous)
```
- **Positive change** (+): Risk increased (moved closer to midnight)
- **Negative change** (-): Risk decreased (moved away from midnight)
- **No change** (0): Risk stable

```
Confidence Level: High
```
- **VeryHigh**: 95%+ confidence in assessment
- **High**: 80-95% confidence
- **Moderate**: 60-80% confidence
- **Low**: 40-60% confidence
- **VeryLow**: <40% confidence (limited data)

**Exit Codes:**
- `0` - Assessment completed successfully
- `1` - Assessment failed (check logs)
- `2` - Configuration error
- `3` - Database error
- `4` - API error (Claude)

### 3.2 `joshua history` - View Assessment History

**Purpose:** View and query historical risk assessments.

**Usage:**
```bash
joshua history [OPTIONS]
```

**Options:**
- `--count <N>` - Number of assessments to show (default: 10)
- `--from <DATE>` - Start date (YYYY-MM-DD or relative: 7d, 1m, 1y)
- `--to <DATE>` - End date (YYYY-MM-DD or relative)
- `--format <FORMAT>` - Output format: table, json, csv (default: table)
- `--compare` - Show comparison between assessments
- `--filter <TREND>` - Filter by trend: increasing, decreasing, stable

**Examples:**

```bash
# Show last 10 assessments
joshua history

# Show last 30 assessments
joshua history --count 30

# Show assessments from last 7 days
joshua history --from 7d

# Show assessments in specific date range
joshua history --from 2025-10-01 --to 2025-10-27

# Export to CSV
joshua history --count 100 --format csv > assessments.csv

# Show only assessments where risk increased
joshua history --filter increasing --count 20

# Compare consecutive assessments
joshua history --compare --count 5
```

**Sample Output (Table Format):**

```
Assessment History (Last 10)
════════════════════════════════════════════════════════════════════════════

Date                 Seconds    Change  Trend        Confidence  Top Factor
──────────────────────────────────────────────────────────────────────────
2025-10-27 12:00     91         +2      Increasing   High        Russia-Ukraine Escalation
2025-10-26 12:00     89         +1      Increasing   High        Iranian Nuclear Program
2025-10-25 12:00     88         -1      Decreasing   Moderate    US-China Dialogue
2025-10-24 12:00     89         0       Stable       High        Arsenal Modernization
2025-10-23 12:00     89         +3      Increasing   VeryHigh    North Korea ICBM Test
2025-10-22 12:00     86         -2      Decreasing   High        Arms Control Talks
2025-10-21 12:00     88         +1      Increasing   Moderate    Middle East Tensions
2025-10-20 12:00     87         0       Stable       High        Routine Monitoring
2025-10-19 12:00     87         -1      Decreasing   High        Treaty Renewal
2025-10-18 12:00     88         +2      Increasing   VeryHigh    False Alarm Incident
──────────────────────────────────────────────────────────────────────────

Summary:
  Average Risk:      88.2 seconds to midnight
  Trend (30d):       Increasing (+3 seconds over 30 days)
  Max Risk (30d):    91 seconds (2025-10-27)
  Min Risk (30d):    86 seconds (2025-10-22)
  Volatility:        Low (±2.1 seconds standard deviation)
```

**Sample Output (Comparison Format):**

```bash
joshua history --compare --count 3
```

```
Assessment Comparison
════════════════════════════════════════════════════════════════════════════

2025-10-27 12:00 UTC → 2025-10-26 12:00 UTC
────────────────────────────────────────────────────────────────────────────
Risk Change:        89 → 91 seconds (+2, +2.2%)
Trend:              Increasing → Increasing
Confidence:         High → High

New Risk Factors:
  + Russia-Ukraine: Artillery strikes on border region (+0.15)
  + Israel-Iran: Retaliatory strike threat escalation (+0.12)

Changed Risk Factors:
  ↑ North Korea: ICBM development progress (0.23 → 0.31, +35%)
  ↓ US-China: Military exercise frequency (0.18 → 0.14, -22%)

Removed Risk Factors:
  - France-UK: Joint naval exercise (de-escalatory)

Primary Drivers:
  1. Regional conflict escalation (Russia-Ukraine, Israel-Iran)
  2. North Korean weapons development acceleration
  3. Decreased diplomatic engagement

════════════════════════════════════════════════════════════════════════════
```

### 3.3 `joshua trends` - Analyze Trends

**Purpose:** Generate trend analysis and visualizations from historical data.

**Usage:**
```bash
joshua trends [OPTIONS]
```

**Options:**
- `--period <PERIOD>` - Analysis period: 7d, 30d, 90d, 1y, all (default: 30d)
- `--factors <LIST>` - Specific risk factors to analyze (comma-separated)
- `--categories <LIST>` - Risk categories to focus on
- `--output <PATH>` - Output directory for charts (default: current directory)
- `--format <FORMAT>` - Chart format: svg, png, both (default: svg)
- `--statistical` - Include statistical analysis (correlation, regression)

**Examples:**

```bash
# Standard 30-day trend analysis
joshua trends

# 90-day trend with all charts
joshua trends --period 90d --format both

# Analyze specific risk factors
joshua trends --factors "Russia-Ukraine,North Korea,Iran Nuclear"

# Category-focused analysis
joshua trends --categories "Regional Conflicts,Nuclear Arsenals"

# Statistical analysis
joshua trends --period 1y --statistical

# Custom output location
joshua trends --period 30d --output ~/joshua-reports/trends/
```

**Generated Visualizations:**

1. **Overall Risk Trend Chart** (`risk_trend.svg`)
   - Line chart showing seconds to midnight over time
   - Moving average overlay
   - Critical events annotated

2. **Category Breakdown** (`category_breakdown.svg`)
   - Stacked area chart of risk categories
   - Shows changing composition of risk

3. **Top Risk Factors** (`top_factors.svg`)
   - Bar chart of most significant risk factors
   - Ranked by contribution to overall risk

4. **Volatility Analysis** (`volatility.svg`)
   - Chart showing risk volatility over time
   - Identifies periods of instability

**Sample Output:**

```
Trend Analysis: Last 30 Days
════════════════════════════════════════════════════════════════════════════

Period:             2025-09-27 to 2025-10-27
Assessments:        30 assessments analyzed
Data Quality:       High (average confidence: 82%)

Overall Trend:      ↗ INCREASING RISK
────────────────────────────────────────────────────────────────────────────
Start Risk:         86 seconds to midnight (2025-09-27)
End Risk:           91 seconds to midnight (2025-10-27)
Change:             +5 seconds (+5.8%)
Rate of Change:     +0.17 seconds/day

Statistical Summary:
────────────────────────────────────────────────────────────────────────────
Mean:               88.2 seconds
Median:             88.0 seconds
Std Deviation:      2.1 seconds
Range:              86-91 seconds
Volatility:         Low (2.4% coefficient of variation)

Trend Direction:    Increasing (24 days), Decreasing (4 days), Stable (2 days)
Confidence:         High (Mann-Kendall test: tau=0.67, p<0.001)

Top Contributing Categories (30-day average):
────────────────────────────────────────────────────────────────────────────
1. Regional Conflicts           23.5%  ↑ Increasing
2. Nuclear Arsenal Changes      18.2%  ↑ Increasing
3. Doctrine and Posture         16.8%  → Stable
4. Technical Incidents          14.1%  ↓ Decreasing
5. Leadership & Rhetoric        12.3%  ↑ Increasing
6. Communication Breakdown       7.9%  → Stable
7. Emerging Technology           4.6%  ↑ Increasing
8. Economic Factors              2.6%  ↓ Decreasing

Top 10 Risk Factors (by average contribution):
────────────────────────────────────────────────────────────────────────────
1. Russia-Ukraine conflict escalation              0.187  ↑
2. North Korea ICBM development                    0.145  ↑
3. Iranian nuclear program advancement             0.132  ↑
4. US-Russia arms control breakdown                0.118  →
5. China military modernization                    0.095  ↑
6. India-Pakistan tensions                         0.079  →
7. Israel regional military operations             0.068  ↑
8. Russian nuclear doctrine changes                0.063  ↑
9. US nuclear posture review delays                0.056  →
10. North Korea-South Korea DMZ incidents          0.047  ↑

Critical Events Identified:
────────────────────────────────────────────────────────────────────────────
2025-10-23  North Korea ICBM test (risk +3 seconds)
2025-10-18  False alarm incident in early warning system (risk +2 seconds)
2025-10-15  Russia-Ukraine border escalation (risk +2 seconds)
2025-10-08  Iran uranium enrichment breakthrough (risk +2 seconds)

Forecasting (next 7 days):
────────────────────────────────────────────────────────────────────────────
If current trend continues:
  Projected Risk (7d):   92-94 seconds to midnight
  Confidence Interval:   ±3 seconds (80% CI)
  Probability of:
    - Further increase:  67%
    - Stabilization:     24%
    - Decrease:           9%

Visualizations Generated:
────────────────────────────────────────────────────────────────────────────
  ✓ risk_trend.svg              (Overall risk timeline)
  ✓ category_breakdown.svg      (Risk category composition)
  ✓ top_factors.svg             (Top 10 risk factors)
  ✓ volatility.svg              (Risk volatility over time)

════════════════════════════════════════════════════════════════════════════
```

### 3.4 `joshua simulate` - Monte Carlo Simulation

**Purpose:** Run scenario simulations using Monte Carlo methods.

**Usage:**
```bash
joshua simulate --scenario <NAME> [OPTIONS]
```

**Options:**
- `--scenario <NAME>` - Scenario to simulate (required)
- `--iterations <N>` - Number of Monte Carlo iterations (default: 10,000)
- `--confidence <LEVEL>` - Confidence interval: 80, 90, 95, 99 (default: 95)
- `--output <PATH>` - Output file for results
- `--visualize` - Generate distribution charts

**Built-in Scenarios:**
- `baseline` - Current risk trajectory continuation
- `escalation` - Regional conflict escalation
- `deescalation` - Diplomatic breakthrough
- `technical-incident` - False alarm or technical failure
- `arsenal-expansion` - Major arsenal modernization
- `arms-control-breakdown` - Treaty collapse
- `custom` - Define custom scenario parameters

**Examples:**

```bash
# Baseline scenario (current trajectory)
joshua simulate --scenario baseline

# Escalation scenario with 50,000 iterations
joshua simulate --scenario escalation --iterations 50000

# Custom scenario with visualization
joshua simulate --scenario custom --visualize

# High confidence interval
joshua simulate --scenario technical-incident --confidence 99
```

**Sample Output:**

```
Monte Carlo Simulation: Regional Escalation Scenario
════════════════════════════════════════════════════════════════════════════

Scenario Parameters:
  Name:                     Regional Escalation
  Iterations:               10,000
  Confidence Interval:      95%
  Initial Risk:             91 seconds to midnight
  Time Horizon:             30 days

Simulation Results:
────────────────────────────────────────────────────────────────────────────
Final Risk Distribution (30 days):

  Median:                   97 seconds to midnight
  Mean:                     98.3 seconds to midnight
  Mode:                     96 seconds to midnight

  95% Confidence Interval:  88-112 seconds
  90% Confidence Interval:  90-108 seconds
  80% Confidence Interval:  92-105 seconds

  Standard Deviation:       6.8 seconds
  Skewness:                 0.43 (slight right skew)
  Kurtosis:                 2.87 (near-normal distribution)

Risk Level Probabilities:
────────────────────────────────────────────────────────────────────────────
  Extreme Risk (< 90s):     12.3%
  Very High Risk (90-180s): 79.1%
  High Risk (180-360s):      8.4%
  Moderate Risk (360-720s):  0.2%
  Low Risk (> 720s):         0.0%

Worst-Case Scenarios (99th percentile):
────────────────────────────────────────────────────────────────────────────
  Risk Level:               62 seconds to midnight
  Probability:              1%
  Key Drivers:
    - Multiple regional conflicts converge
    - Communication breakdown between superpowers
    - Technical incident during high tensions

Best-Case Scenarios (1st percentile):
────────────────────────────────────────────────────────────────────────────
  Risk Level:               125 seconds to midnight
  Probability:              1%
  Key Drivers:
    - Rapid diplomatic intervention
    - De-escalation agreements
    - Regional conflict resolution

Key Insights:
────────────────────────────────────────────────────────────────────────────
1. 87.7% probability risk remains in "Very High" range
2. Strong correlation between regional conflicts and overall risk (r=0.83)
3. Low probability of natural de-escalation without intervention
4. Risk trajectory highly sensitive to diplomatic engagement
5. Multiple escalation pathways identified (need mitigation)

Recommendations:
────────────────────────────────────────────────────────────────────────────
1. Immediate diplomatic engagement to prevent worst-case outcomes
2. Enhanced communication channels between nuclear states
3. Confidence-building measures in conflict regions
4. Technical safeguards to prevent incidents during tensions
5. Public messaging to manage escalation dynamics

Visualization saved to: simulation_regional_escalation.svg
════════════════════════════════════════════════════════════════════════════
```

### 3.5 `joshua interactive` - Terminal UI Mode

**Purpose:** Launch full-screen interactive terminal user interface.

**Usage:**
```bash
joshua interactive
```

**Features:**
- Real-time assessment monitoring
- Interactive data exploration
- Live trend visualization
- Historical browsing
- WarGames-inspired retro aesthetic

**Example Session:**

```
╔══════════════════════════════════════════════════════════════════════════╗
║                        WarGames/JOSHUA SYSTEM                            ║
║                  Nuclear War Risk Assessment Monitor                     ║
╚══════════════════════════════════════════════════════════════════════════╝

GREETINGS PROFESSOR FALKEN.

┌─ Current Assessment ─────────────────────────────────────────────────────┐
│                                                                          │
│                          DOOMSDAY CLOCK                                  │
│                                                                          │
│                              ┌─────┐                                     │
│                          ╱       XII    ╲                                │
│                        ╱   ●────────●    ╲                               │
│                       │   ╱          │    │                              │
│                       │  ╱           │    │                              │
│                       │ ╱      ●     │    │                              │
│                       │╱             │    │                              │
│                        ╲             │   ╱                               │
│                         ╲        ●──┼──●                                 │
│                          ╲           ╱                                   │
│                            ╲       ╱                                     │
│                              └───┘                                       │
│                                                                          │
│                     91 SECONDS TO MIDNIGHT                               │
│                                                                          │
│  Assessment Date:    2025-10-27 12:00 UTC                                │
│  Risk Level:         VERY HIGH                                           │
│  Trend:              ↗ INCREASING (+2 seconds from previous)             │
│  Confidence:         HIGH (82%)                                          │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘

┌─ Top Risk Factors ───────────────────────────────────────────────────────┐
│                                                                          │
│  1. Russia-Ukraine Conflict Escalation        ████████████████  0.187   │
│  2. North Korea ICBM Development              ████████████     0.145    │
│  3. Iranian Nuclear Program Advancement       ███████████      0.132    │
│  4. US-Russia Arms Control Breakdown          ██████████       0.118    │
│  5. China Military Modernization              ████████         0.095    │
│  6. India-Pakistan Border Tensions            ██████           0.079    │
│  7. Israel Regional Military Operations       █████            0.068    │
│  8. Russian Nuclear Doctrine Changes          █████            0.063    │
│  9. US Nuclear Posture Review Delays          ████             0.056    │
│  10. North Korea-South Korea DMZ Incidents    ████             0.047    │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘

┌─ 30-Day Trend ───────────────────────────────────────────────────────────┐
│                                                                          │
│  95 ┤                                                            ●       │
│  93 ┤                                                        ●   │       │
│  91 ┤                                                    ●   │   │       │
│  89 ┤                        ●●●●●●●●●●●●●●●●●●●●●●●●●●   │   │   │       │
│  87 ┤                    ●●●                                 │   │       │
│  85 ┤                ●●●                                     │   │       │
│  83 ┤            ●●●                                         │   │       │
│  81 ┤        ●●●                                             │   │       │
│  79 ┤    ●●●                                                 │   │       │
│  77 ┤●●●                                                     │   │       │
│     └┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬─┤
│      Sep27      Oct04      Oct11      Oct18      Oct25              Oct27│
│                                                                          │
│  Direction: ↗ INCREASING    Rate: +0.17 sec/day    Volatility: LOW      │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘

[A]ssess  [H]istory  [T]rends  [S]imulate  [R]eport  [C]onfig  [Q]uit

> _
```

**Keyboard Controls:**
- `A` - Run new assessment
- `H` - Browse assessment history
- `T` - View trend analysis
- `S` - Run Monte Carlo simulation
- `R` - Generate/view reports
- `C` - Edit configuration
- `Q` - Quit
- Arrow keys - Navigate
- Enter - Select
- Esc - Go back

### 3.6 `joshua diagnose` - System Health Check

**Purpose:** Diagnose system configuration and component health.

**Usage:**
```bash
joshua diagnose [OPTIONS]
```

**Options:**
- `--verbose` - Show detailed diagnostic information
- `--fix` - Attempt to fix common issues automatically
- `--component <NAME>` - Check specific component only

**Components:**
- `config` - Configuration validation
- `database` - Database connectivity and schema
- `claude` - Claude API connectivity
- `sources` - Data source accessibility
- `notifications` - Notification channel testing
- `all` - All components (default)

**Examples:**

```bash
# Standard diagnostic
joshua diagnose

# Verbose output
joshua diagnose --verbose

# Check specific component
joshua diagnose --component claude

# Auto-fix common issues
joshua diagnose --fix
```

### 3.7 `joshua init-db` - Initialize Database

**Purpose:** Initialize or migrate database schema.

**Usage:**
```bash
joshua init-db [OPTIONS]
```

**Options:**
- `--connection <STRING>` - Database connection string
- `--force` - Drop existing database and reinitialize
- `--migrate` - Run pending migrations only
- `--seed` - Load sample data (for testing)

**Examples:**

```bash
# Initialize PostgreSQL database
joshua init-db --connection postgresql://localhost:5432/joshua

# Force reinitialize (DESTRUCTIVE)
joshua init-db --force

# Run migrations only
joshua init-db --migrate

# Initialize with sample data
joshua init-db --seed
```

### 3.8 Global Options

All commands support these global options:

```bash
joshua <COMMAND> [GLOBAL_OPTIONS]

Global Options:
  -h, --help              Show help information
  -V, --version           Show version information
  -v, --verbose           Enable verbose output
  -q, --quiet             Suppress non-error output
  --config <PATH>         Path to configuration file
  --log-level <LEVEL>     Log level: error, warn, info, debug, trace
  --log-file <PATH>       Write logs to file
  --no-color              Disable colored output
```

---

## 4. Understanding Risk Assessments

### 4.1 Risk Score Interpretation

The system expresses nuclear war risk as **"seconds to midnight"** on the Doomsday Clock:

**Risk Scale:**
```
       MIDNIGHT (0 seconds) = Nuclear War
            ↑
        ╔═══════════════╗
        ║ EXTREME RISK  ║  0-60 seconds
        ║   Imminent    ║  (Immediate crisis)
        ╚═══════════════╝
            ↑
        ╔═══════════════╗
        ║ VERY HIGH     ║  60-180 seconds
        ║   Critical    ║  (Severe tensions)
        ╚═══════════════╝
            ↑
        ╔═══════════════╗
        ║  HIGH RISK    ║  180-360 seconds
        ║   Elevated    ║  (Significant concerns)
        ╚═══════════════╝
            ↑
        ╔═══════════════╗
        ║ MODERATE      ║  360-720 seconds
        ║   Watchful    ║  (Baseline monitoring)
        ╚═══════════════╝
            ↑
        ╔═══════════════╗
        ║   LOW RISK    ║  720+ seconds
        ║   Stable      ║  (Relative calm)
        ╚═══════════════╝
            ↑
        NOON (1440 seconds) = Minimal Risk
```

**Historical Context:**
- **2 minutes (120s)**: Closest to midnight (1953, 2020, 2023-2024)
- **17 minutes (1020s)**: Furthest from midnight (1991, Cold War end)
- **Current (2025)**: 89 seconds to midnight

### 4.2 Risk Categories

The system evaluates 8 major risk categories:

**1. Nuclear Arsenal Changes (15% weight)**
- Warhead count increases/decreases
- New weapons systems deployment
- Arsenal modernization programs
- Delivery system upgrades (ICBMs, SLBMs, bombers)

**2. Doctrine and Posture (15% weight)**
- Nuclear doctrine modifications
- First-use policy changes
- Alert status changes
- Launch-on-warning posture shifts

**3. Regional Conflicts (20% weight)**
- Active military conflicts involving nuclear states
- Proxy wars and escalation risks
- Border disputes and territorial conflicts
- Regional alliances and military exercises

**4. Leadership & Rhetoric (10% weight)**
- Nuclear threats from leaders
- Hostile rhetoric escalation
- Political instability in nuclear states
- Nationalist movements and populism

**5. Technical Incidents (15% weight)**
- False alarms in early warning systems
- Accidents involving nuclear weapons
- Cyberattacks on nuclear infrastructure
- Command and control failures

**6. Communication Breakdown (10% weight)**
- Diplomatic channel disruptions
- Hotline failures or non-use
- Intelligence sharing breakdowns
- Treaty monitoring impediments

**7. Emerging Technology (10% weight)**
- Hypersonic weapons development
- AI in nuclear decision-making
- Space-based weapons systems
- Missile defense proliferation

**8. Economic Factors (5% weight)**
- Sanctions and economic warfare
- Resource competition
- Military spending changes
- Economic instability in nuclear states

### 4.3 Confidence Levels

Each assessment includes a confidence level indicating data quality and certainty:

**VeryHigh (95%+ confidence)**
- Abundant high-quality data from multiple reliable sources
- Strong consensus across analyses
- Clear causal relationships
- Minimal uncertainty in interpretation

**High (80-95% confidence)**
- Good data coverage from reliable sources
- General consensus in analysis
- Clear trends and patterns
- Some interpretive uncertainty

**Moderate (60-80% confidence)**
- Adequate data but some gaps
- Mixed signals or conflicting information
- Trends present but less clear
- Moderate interpretive challenges

**Low (40-60% confidence)**
- Limited data availability
- Significant conflicting information
- Weak or unclear patterns
- Substantial uncertainty

**VeryLow (<40% confidence)**
- Insufficient data for strong conclusions
- Highly conflicting information
- No clear trends
- High uncertainty (use with caution)

### 4.4 Trend Directions

**Increasing ↗**
- Risk is rising (moving closer to midnight)
- Indicates worsening conditions
- Triggers heightened monitoring
- May activate alert notifications

**Decreasing ↘**
- Risk is falling (moving away from midnight)
- Indicates improving conditions
- De-escalation in progress
- Positive development

**Stable →**
- Risk relatively unchanged
- Conditions maintaining current level
- Neither improving nor worsening
- Continued monitoring warranted

**Uncertain ?**
- Trend direction unclear
- Conflicting signals
- Volatility or rapid changes
- Requires additional data

### 4.5 Contributing Factors

Each assessment identifies specific risk factors ranked by contribution:

**Example Risk Factor:**
```
Factor: Russia-Ukraine Conflict Escalation
Category: Regional Conflicts
Contribution: 0.187 (18.7% of total risk)
Confidence: High
Trend: Increasing
Data Sources:
  - Reuters: 23 articles
  - State Department: 4 reports
  - SIPRI: 2 analyses
Evidence Summary:
  - Artillery strikes on border regions increased 40% this week
  - Both sides mobilizing additional forces
  - Diplomatic channels show limited progress
  - NATO expressing increased concern
Contrary Evidence:
  - Some local ceasefires reported
  - Humanitarian corridors maintained
Historical Context:
  - Current level exceeds 2024 peak tensions
  - Similar to 2022 escalation pattern
```

**Interpretation:**
- **Contribution**: Higher = more significant impact on overall risk
- **Confidence**: Higher = more reliable factor assessment
- **Trend**: Direction this specific factor is moving
- **Evidence**: Supporting data from multiple sources
- **Contrary Evidence**: Information suggesting different conclusion
- **Historical Context**: Comparison to past similar situations

---

## 5. Reading Reports

### 5.1 Report Formats

WarGames/JOSHUA generates reports in three formats:

**1. Markdown (.md)**
- Plain text with formatting
- Easy to read in terminal or text editor
- Version control friendly
- Lightweight and portable

**2. HTML (.html)**
- Rich formatting with CSS styling
- Interactive charts and visualizations
- Embedded images
- Best for browser viewing and sharing

**3. PDF (.pdf)**
- Professional presentation format
- Print-ready layout
- Archival quality
- Best for official distribution

### 5.2 Report Structure

All reports follow this structure:

#### Header Section
```markdown
# Nuclear War Risk Assessment Report
## WarGames/JOSHUA System

Assessment ID:    asmt_2025_10_27_120000
Assessment Date:  2025-10-27 12:00:00 UTC
Generated:        2025-10-27 12:02:34 UTC
System Version:   1.0.0
Claude Model:     claude-sonnet-4-20250514
```

#### Executive Summary
```markdown
## Executive Summary

Current nuclear war risk stands at 91 seconds to midnight on the
Doomsday Clock, representing a VERY HIGH RISK level. This marks an
increase of 2 seconds from the previous assessment, driven primarily
by escalating regional conflicts and nuclear arsenal developments.

Key Findings:
- Russia-Ukraine conflict showing signs of further escalation
- North Korean ICBM program demonstrating new capabilities
- Iranian nuclear program advancement continues
- Diplomatic channels under strain across multiple regions
- Technical incident near-miss in early warning system

Confidence in this assessment is HIGH (82%), based on analysis of
171 data points from 12 reliable sources including government reports,
think tank analyses, and verified news sources.
```

#### Risk Level Assessment
```markdown
## Risk Level Assessment

┌─────────────────────────────────────────────────────────────┐
│                    DOOMSDAY CLOCK                           │
│                  91 SECONDS TO MIDNIGHT                     │
│                      VERY HIGH RISK                         │
└─────────────────────────────────────────────────────────────┘

Risk Score:         91 seconds to midnight
Risk Level:         VERY HIGH
Previous Score:     89 seconds to midnight
Change:             +2 seconds (INCREASED)
Trend Direction:    ↗ INCREASING
Trend Magnitude:    +2.2%
Confidence:         HIGH (82%)

Historical Context:
- Current: 91 seconds (2025-10-27)
- 30-day avg: 88 seconds
- 90-day avg: 87 seconds
- 1-year avg: 85 seconds
- All-time closest: 89 seconds (January 2025)
```

#### Risk Factor Analysis
```markdown
## Risk Factor Analysis

### Top 10 Contributing Factors

1. Russia-Ukraine Conflict Escalation
   Category: Regional Conflicts
   Contribution: 18.7%
   Confidence: High
   Trend: Increasing

   Recent artillery strikes on border regions have intensified, with
   40% increase in incidents this week. Both sides mobilizing additional
   forces. Diplomatic progress limited.

2. North Korea ICBM Development
   Category: Nuclear Arsenal Changes
   Contribution: 14.5%
   Confidence: High
   Trend: Increasing

   Successful test of new solid-fuel ICBM demonstrating improved range
   and launch readiness. Intelligence suggests potential for further
   tests in coming weeks.

[... continues for all factors]
```

#### Category Breakdown
```markdown
## Risk Category Breakdown

| Category                  | Weight | Score | Contribution | Trend      |
|---------------------------|--------|-------|--------------|------------|
| Regional Conflicts        | 20%    | 0.89  | 23.5%        | Increasing |
| Nuclear Arsenal Changes   | 15%    | 0.82  | 18.2%        | Increasing |
| Doctrine and Posture      | 15%    | 0.76  | 16.8%        | Stable     |
| Technical Incidents       | 15%    | 0.64  | 14.1%        | Decreasing |
| Leadership & Rhetoric     | 10%    | 0.71  | 12.3%        | Increasing |
| Communication Breakdown   | 10%    | 0.53  | 7.9%         | Stable     |
| Emerging Technology       | 10%    | 0.31  | 4.6%         | Increasing |
| Economic Factors          | 5%     | 0.35  | 2.6%         | Decreasing |
```

#### Detailed Analysis
```markdown
## Detailed Analysis

### Regional Conflicts (23.5% contribution)

The Russia-Ukraine conflict continues to be the primary driver of
nuclear risk, accounting for 18.7% of total risk on its own...

[Detailed analysis of each category]
```

#### Recommendations
```markdown
## Recommendations

### Immediate Actions (High Priority)
1. Enhance diplomatic engagement in Russia-Ukraine conflict
2. Strengthen communication channels between nuclear powers
3. Monitor North Korean ICBM developments closely
4. Increase transparency around Iranian nuclear program

### Medium-Term Actions
1. Revitalize arms control negotiations
2. Improve early warning system reliability
3. Establish crisis management protocols for regional conflicts
4. Enhance verification mechanisms for treaty compliance

### Long-Term Actions
1. Comprehensive nuclear risk reduction framework
2. Multilateral diplomatic initiatives
3. Technology safeguards for emerging threats
4. Public education and awareness programs
```

#### Appendices
```markdown
## Appendix A: Data Sources

Total Data Points: 171
Collection Time: 47 seconds
Source Categories: News Media (127), Government Reports (23),
                   Think Tanks (15), Research Institutions (6)

Source Breakdown:
- Reuters: 41 articles (reliability: 0.85)
- SIPRI: 9 analyses (reliability: 0.95)
- Bulletin of Atomic Scientists: 15 updates (reliability: 0.90)
- State Department: 8 reports (reliability: 0.85)
[... continues]

## Appendix B: Methodology

Risk Calculation:
1. Base score from weighted risk factors
2. Bayesian adjustment using historical correlations
3. Monte Carlo simulation (10,000 iterations)
4. Confidence-weighted aggregation
5. Seconds-to-midnight conversion

## Appendix C: Historical Comparison

[Charts and historical data]

## Appendix D: Statistical Details

[Detailed statistical analysis]
```

### 5.3 Interpreting Visualizations

#### Doomsday Clock Diagram
```
Visual representation showing:
- Clock face with hands
- Current time to midnight
- Color coding (red = danger, yellow = caution, green = stable)
- Risk level label
```

#### Trend Charts
```
Time series showing:
- Risk level over time
- Moving averages
- Trend line
- Confidence intervals (shaded area)
- Critical events annotated
```

#### Category Breakdown Charts
```
Stacked area or bar chart showing:
- Relative contribution of each category
- How composition changes over time
- Dominant risk sources
```

#### Risk Factor Heatmap
```
Matrix visualization showing:
- Risk factors on one axis
- Time period on other axis
- Color intensity = risk level
- Easy pattern identification
```

---

## 6. Configuration

### 6.1 Configuration File Location

**Default paths** (in priority order):
1. `./config.toml` (current directory)
2. `~/.config/joshua/config.toml` (user config)
3. `/etc/joshua/config.toml` (system config)

**Custom path:**
```bash
joshua --config /path/to/config.toml assess
```

### 6.2 Configuration File Structure

**Complete example** (`~/.config/joshua/config.toml`):

```toml
# WarGames/JOSHUA Configuration File
# Version: 1.0.0

[general]
# Logging configuration
log_level = "info"  # error, warn, info, debug, trace
log_file = "~/.local/share/joshua/joshua.log"
log_format = "json"  # text or json

# Data directory
data_directory = "~/.local/share/joshua"
cache_directory = "~/.cache/joshua"
reports_directory = "~/.local/share/joshua/reports"

# Timezone for reports (default: UTC)
timezone = "UTC"

[database]
# Database connection
connection_string = "postgresql://joshua:password@localhost:5432/joshua"
# For SQLite: connection_string = "sqlite://~/.local/share/joshua/joshua.db"

# Connection pool settings
pool_size = 10
min_connections = 2
max_lifetime = "30m"
idle_timeout = "10m"
connection_timeout = 30

# Retry settings
max_retries = 3
retry_delay = 2

[claude]
# Anthropic API configuration
api_key = "sk-ant-api03-..."  # REQUIRED
model = "claude-sonnet-4-20250514"
max_tokens = 16000
temperature = 0.7
timeout = 120

# Cost controls
max_cost_per_assessment = 1.50  # USD
monthly_budget = 500.00  # USD
alert_threshold = 80  # Alert at 80% of budget

# Caching
cache_responses = true
cache_ttl = 21600  # 6 hours

# Retry settings
max_retries = 3
retry_delay = 2
exponential_backoff = true

[data_collection]
# Enabled data sources
enabled_sources = [
    "reuters",
    "sipri",
    "bulletin",
    "state_dept",
    "carnegie",
    "rand",
    "chatham_house",
]

# Collection settings
cache_duration = 3600  # 1 hour
max_parallel_collectors = 10
collection_timeout = 300  # 5 minutes
global_timeout = 600  # 10 minutes

# Deduplication
deduplication_enabled = true
similarity_threshold = 0.85

# Quality filtering
min_reliability_score = 0.6
min_data_quality = 0.5

# Source-specific settings
[data_collection.sources.reuters]
enabled = true
rss_url = "https://www.reuters.com/rssfeed/nuclearNews"
reliability = 0.85
rate_limit = 100  # requests per hour

[data_collection.sources.sipri]
enabled = true
api_url = "https://api.sipri.org/v1"
api_key = ""  # Optional
reliability = 0.95
rate_limit = 50

[risk_calculation]
# Risk model settings
model_type = "bayesian"  # bayesian, frequentist, hybrid

# Monte Carlo simulation
monte_carlo_iterations = 10000
confidence_interval = 95

# Category weights (must sum to 1.0)
[risk_calculation.category_weights]
nuclear_arsenal_changes = 0.15
doctrine_and_posture = 0.15
regional_conflicts = 0.20
leadership_rhetoric = 0.10
technical_incidents = 0.15
communication_breakdown = 0.10
emerging_technology = 0.10
economic_factors = 0.05

# Bayesian priors
[risk_calculation.bayesian]
use_historical_priors = true
prior_weight = 0.3
update_frequency = "weekly"

[visualization]
# Chart settings
default_format = "svg"  # svg, png, both
dpi = 300
width = 1200
height = 800

# Color scheme
theme = "dark"  # dark, light, classic (amber)
primary_color = "#ff6b35"
secondary_color = "#f7931e"

# Doomsday clock settings
clock_style = "classic"  # classic, modern, retro

[reporting]
# Default report format
default_format = "markdown"  # markdown, html, pdf, all

# Report templates
template_directory = "~/.config/joshua/templates"
custom_template = ""  # Path to custom template

# PDF settings (if PDF reports enabled)
pdf_engine = "wkhtmltopdf"  # wkhtmltopdf, weasyprint
pdf_page_size = "letter"  # letter, a4
pdf_orientation = "portrait"

# Report content
include_appendices = true
include_visualizations = true
include_raw_data = false

[notifications]
# Enable notifications
enabled = true

# Notification triggers
[notifications.triggers]
risk_increase = 5  # Notify if risk increases by 5+ seconds
risk_threshold = 60  # Notify if risk drops below 60 seconds
confidence_drop = 20  # Notify if confidence drops 20+ percentage points

# Email notifications
[notifications.email]
enabled = true
smtp_host = "smtp.example.com"
smtp_port = 587
smtp_username = "joshua@example.com"
smtp_password = "password"
from_address = "joshua@example.com"
to_addresses = ["alerts@example.com"]
use_tls = true

# Webhook notifications
[notifications.webhook]
enabled = true
url = "https://example.com/webhooks/joshua"
method = "POST"
headers = { "Authorization" = "Bearer token123" }
timeout = 10

# Slack notifications (optional)
[notifications.slack]
enabled = false
webhook_url = ""
channel = "#nuclear-alerts"
username = "JOSHUA"
icon_emoji = ":warning:"

[scheduling]
# Automated assessment scheduling
enabled = false

# Cron expression (minute hour day month weekday)
cron_expression = "0 0 * * *"  # Daily at midnight UTC
# cron_expression = "0 */6 * * *"  # Every 6 hours
# cron_expression = "0 0 * * 1"  # Weekly on Monday

# Scheduler settings
max_concurrent_assessments = 1
assessment_timeout = 600  # 10 minutes
retry_on_failure = true
max_retries = 3

[api]
# REST API server (if enabled)
enabled = false
host = "0.0.0.0"
port = 8080
workers = 4

# Authentication
require_auth = true
jwt_secret = "your-secret-key-here"
token_expiry = 3600  # 1 hour

# Rate limiting
rate_limit_enabled = true
requests_per_minute = 60

[security]
# Encryption at rest
encrypt_database = true
encryption_key_file = "~/.config/joshua/encryption.key"

# Audit logging
audit_log_enabled = true
audit_log_file = "~/.local/share/joshua/audit.log"

# API key rotation
rotate_api_keys = false
rotation_interval = "90d"

[performance]
# Caching
enable_redis = false
redis_url = "redis://localhost:6379"
redis_ttl = 3600

# Thread pool
worker_threads = 4
max_blocking_threads = 512

# Memory limits
max_memory_mb = 500
cache_size_mb = 100
```

### 6.3 Environment Variables

Environment variables override configuration file settings:

```bash
# General
export JOSHUA_LOG_LEVEL=debug
export JOSHUA_DATA_DIR=/var/lib/joshua

# Database
export DATABASE_URL=postgresql://localhost:5432/joshua
export JOSHUA_DB_POOL_SIZE=20

# Claude API
export ANTHROPIC_API_KEY=sk-ant-api03-...
export CLAUDE_MODEL=claude-opus-4-20250514
export CLAUDE_TIMEOUT=180

# Notifications
export JOSHUA_EMAIL_ENABLED=true
export JOSHUA_EMAIL_TO=alerts@example.com
export JOSHUA_WEBHOOK_URL=https://example.com/hooks/joshua

# Scheduling
export JOSHUA_SCHEDULING_ENABLED=true
export JOSHUA_CRON="0 0 * * *"

# API Server
export JOSHUA_API_ENABLED=true
export JOSHUA_API_PORT=8080
export JOSHUA_JWT_SECRET=secret-key-here
```

### 6.4 Configuration Best Practices

**1. Secure API Keys**
```bash
# Never commit API keys to version control
echo "config/local_config.toml" >> .gitignore

# Use environment variables for sensitive data
export ANTHROPIC_API_KEY=sk-ant-api03-...

# Or use secrets management
joshua --config <(vault kv get -field=config secret/joshua)
```

**2. Separate Environments**
```bash
# Development
joshua --config ~/.config/joshua/dev.toml assess

# Staging
joshua --config ~/.config/joshua/staging.toml assess

# Production
joshua --config ~/.config/joshua/prod.toml assess
```

**3. Validate Configuration**
```bash
# Check configuration validity
joshua diagnose --component config

# Test with dry-run
joshua assess --dry-run
```

---

## 7. FAQ and Troubleshooting

### 7.1 Frequently Asked Questions

**Q: How often should I run assessments?**

A: It depends on your use case:
- **Real-time monitoring**: Every 6-12 hours
- **Research/analysis**: Daily or weekly
- **Periodic review**: Weekly or monthly

The system caches data for 1 hour by default, so running more frequently than hourly provides diminishing returns.

**Q: How accurate are the risk assessments?**

A: The system provides data-driven analysis with stated confidence levels. Historical validation shows strong correlation with expert consensus (Bulletin of Atomic Scientists). However, nuclear risk assessment is inherently uncertain - treat assessments as informed guidance, not absolute predictions.

**Q: How much does it cost to run assessments?**

A: Costs depend primarily on Claude API usage:
- **Per assessment**: $0.50-$1.50 (depending on data volume and model)
- **Monthly (daily assessments)**: ~$30-$45
- **Monthly (6-hour assessments)**: ~$120-$180

PostgreSQL hosting and data collection bandwidth add minimal costs.

**Q: Can I run this without a Claude API key?**

A: No, Claude AI is integral to the risk analysis process. However, you can use the data collection and historical analysis features without API calls by examining past assessments.

**Q: How is this different from the Bulletin of Atomic Scientists' Doomsday Clock?**

A: Key differences:
- **Frequency**: JOSHUA provides continuous monitoring; Bulletin updates annually
- **Methodology**: JOSHUA uses AI + statistical models; Bulletin uses expert panel
- **Scope**: JOSHUA focuses on real-time data; Bulletin considers broader long-term trends
- **Purpose**: JOSHUA is a monitoring tool; Bulletin is an authoritative annual statement

JOSHUA complements, not replaces, the Bulletin's work.

**Q: Can I customize the risk categories or weights?**

A: Yes! Edit the `[risk_calculation.category_weights]` section in your config file. Ensure weights sum to 1.0:

```toml
[risk_calculation.category_weights]
regional_conflicts = 0.25  # Increased from 0.20
nuclear_arsenal_changes = 0.15
# ... adjust others to sum to 1.0
```

**Q: What if data collection fails for a source?**

A: The system is resilient:
- Failed collectors don't block assessment
- Assessment uses available data with adjusted confidence
- Diagnostic logs identify failed sources
- Retry logic handles transient failures

**Q: How do I interpret conflicting risk factors?**

A: Look at:
1. **Confidence levels**: Higher confidence factors are more reliable
2. **Data sources**: More sources = more reliable
3. **Trend consistency**: Consistent trends across factors are stronger signals
4. **Historical context**: Compare to similar past situations

**Q: Can I export data for my own analysis?**

A: Yes:
```bash
# Export assessment history as CSV
joshua history --count 365 --format csv > assessments_2025.csv

# Export via API (if enabled)
curl -H "Authorization: Bearer $TOKEN" \
  https://api.yourinstance.com/v1/assessments?limit=100 > data.json
```

### 7.2 Common Issues and Solutions

#### Issue: "Database connection failed"

**Symptoms:**
```
Error: Database operation failed: connection
Failed to connect to postgresql://localhost:5432/joshua
```

**Solutions:**

1. **Check PostgreSQL is running:**
   ```bash
   sudo systemctl status postgresql
   # or
   pg_isready
   ```

2. **Verify connection string:**
   ```bash
   joshua diagnose --component database
   ```

3. **Check permissions:**
   ```sql
   -- Connect as postgres user
   psql -U postgres

   -- Grant permissions
   GRANT ALL PRIVILEGES ON DATABASE joshua TO your_user;
   ```

4. **Initialize database:**
   ```bash
   joshua init-db --connection postgresql://localhost:5432/joshua
   ```

#### Issue: "Claude API authentication failed"

**Symptoms:**
```
Error: Claude API error: Invalid authentication
Status: 401 Unauthorized
```

**Solutions:**

1. **Verify API key:**
   ```bash
   echo $ANTHROPIC_API_KEY
   # Should start with sk-ant-api03-
   ```

2. **Check key validity:**
   Visit https://console.anthropic.com and verify key is active

3. **Update configuration:**
   ```bash
   nano ~/.config/joshua/config.toml
   # Update [claude] api_key
   ```

4. **Test connection:**
   ```bash
   joshua diagnose --component claude
   ```

#### Issue: "Assessment takes too long (timeout)"

**Symptoms:**
```
Error: Assessment timed out after 300 seconds
```

**Solutions:**

1. **Increase timeout:**
   ```toml
   [data_collection]
   collection_timeout = 600  # 10 minutes
   ```

2. **Reduce parallel collectors:**
   ```toml
   [data_collection]
   max_parallel_collectors = 5  # Reduced from 10
   ```

3. **Enable caching:**
   ```toml
   [data_collection]
   cache_duration = 7200  # 2 hours
   ```

4. **Check network:**
   ```bash
   joshua diagnose --verbose
   # Look for slow or failing sources
   ```

#### Issue: "High API costs"

**Symptoms:**
- Claude API bills higher than expected

**Solutions:**

1. **Enable response caching:**
   ```toml
   [claude]
   cache_responses = true
   cache_ttl = 21600  # 6 hours
   ```

2. **Use lower-cost model:**
   ```toml
   [claude]
   model = "claude-sonnet-4-20250514"  # Instead of claude-opus
   ```

3. **Set budget limits:**
   ```toml
   [claude]
   max_cost_per_assessment = 0.75
   monthly_budget = 50.00
   alert_threshold = 80
   ```

4. **Reduce assessment frequency:**
   ```toml
   [scheduling]
   cron_expression = "0 0 * * *"  # Once daily instead of every 6 hours
   ```

#### Issue: "Low confidence assessments"

**Symptoms:**
```
Confidence Level: Low (45%)
Warning: Assessment quality may be insufficient
```

**Solutions:**

1. **Check data source health:**
   ```bash
   joshua diagnose --component sources
   ```

2. **Enable more sources:**
   ```toml
   [data_collection]
   enabled_sources = [
       "reuters", "sipri", "bulletin", "state_dept",
       "carnegie", "rand", "chatham_house", "iaea"
   ]
   ```

3. **Reduce quality threshold:**
   ```toml
   [data_collection]
   min_reliability_score = 0.5  # Reduced from 0.6
   ```

4. **Run during peak data availability:**
   - Avoid weekends/holidays when sources publish less

#### Issue: "Report generation fails"

**Symptoms:**
```
Error: Visualization error: Failed to generate doomsday_clock.svg
```

**Solutions:**

1. **Check disk space:**
   ```bash
   df -h ~/.local/share/joshua
   ```

2. **Verify write permissions:**
   ```bash
   ls -la ~/.local/share/joshua/reports/
   ```

3. **Install dependencies (for PDF):**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install wkhtmltopdf

   # macOS
   brew install wkhtmltopdf
   ```

4. **Use alternative format:**
   ```bash
   joshua assess --output text  # Skip visualizations
   ```

### 7.3 Getting More Help

**Check logs:**
```bash
# View recent logs
tail -f ~/.local/share/joshua/joshua.log

# Search logs
grep ERROR ~/.local/share/joshua/joshua.log

# Enable debug logging
joshua --log-level debug assess
```

**Run diagnostics:**
```bash
# Full diagnostic
joshua diagnose --verbose

# Auto-fix common issues
joshua diagnose --fix
```

**Community resources:**
- GitHub Issues: https://github.com/yourusername/wargames-joshua/issues
- Discussions: https://github.com/yourusername/wargames-joshua/discussions
- Documentation: https://wargames-joshua.example.com/docs

**Report bugs:**
```bash
# Include diagnostic output
joshua diagnose --verbose > diagnostic.txt

# Attach to GitHub issue with:
# - joshua --version output
# - Relevant error messages
# - Steps to reproduce
```

---

## 8. Best Practices

### 8.1 Assessment Frequency

**Recommended schedules:**

**Real-time monitoring (organizations):**
```toml
[scheduling]
cron_expression = "0 */6 * * *"  # Every 6 hours
```
- Captures rapid developments
- Provides timely alerts
- Higher API costs (~$180/month)

**Daily monitoring (researchers):**
```toml
[scheduling]
cron_expression = "0 0 * * *"  # Daily at midnight UTC
```
- Balances timeliness and cost
- Good for trend analysis
- Moderate costs (~$45/month)

**Weekly reviews (general interest):**
```toml
[scheduling]
cron_expression = "0 0 * * 1"  # Monday mornings
```
- Sufficient for broad awareness
- Minimal costs (~$10/month)
- Focus on longer-term trends

**Ad-hoc assessments:**
```bash
# Run manually during major events
joshua assess --force
```
- After breaking news of nuclear significance
- Following geopolitical crises
- When expert analyses raise concerns

### 8.2 Data Interpretation Guidelines

**1. Consider confidence levels**
- High confidence assessments are more reliable
- Low confidence suggests data gaps or uncertainty
- Don't over-interpret low-confidence results

**2. Look for trends, not single points**
- Individual assessments can fluctuate
- Focus on 7-30 day trends
- Use `joshua trends` for proper analysis

**3. Understand limitations**
- AI analysis is sophisticated but not infallible
- Risk assessment is inherently uncertain
- Multiple perspectives (human experts, JOSHUA) are best

**4. Context matters**
- Historical comparisons provide perspective
- Current events influence risk heavily
- Long-term trends vs. short-term volatility

**5. Use multiple data points**
- Don't rely solely on seconds-to-midnight
- Examine individual risk factors
- Review confidence and data quality

### 8.3 Alert Threshold Configuration

**Conservative (high sensitivity):**
```toml
[notifications.triggers]
risk_increase = 3  # Alert on +3 seconds
risk_threshold = 100  # Alert if below 100 seconds
confidence_drop = 15  # Alert if confidence drops 15%
```
- More frequent alerts
- Catch small changes
- Risk of alert fatigue

**Balanced (recommended):**
```toml
[notifications.triggers]
risk_increase = 5  # Alert on +5 seconds
risk_threshold = 60  # Alert if below 60 seconds
confidence_drop = 20  # Alert if confidence drops 20%
```
- Significant changes only
- Manageable alert volume
- Good signal-to-noise ratio

**Selective (low sensitivity):**
```toml
[notifications.triggers]
risk_increase = 10  # Alert on +10 seconds
risk_threshold = 30  # Alert if below 30 seconds (extreme)
confidence_drop = 30  # Alert if confidence drops 30%
```
- Rare, critical alerts only
- May miss important developments
- For crisis-only monitoring

### 8.4 Historical Data Management

**Retention policies:**

```sql
-- Keep all assessments for 1 year
DELETE FROM assessments WHERE assessment_date < NOW() - INTERVAL '1 year';

-- Archive older data to separate table
INSERT INTO assessments_archive SELECT * FROM assessments
WHERE assessment_date < NOW() - INTERVAL '1 year';
```

**Backup strategy:**

```bash
# Daily backups
0 2 * * * pg_dump joshua | gzip > /backups/joshua_$(date +\%Y\%m\%d).sql.gz

# Keep 90 days of backups
0 3 * * * find /backups -name "joshua_*.sql.gz" -mtime +90 -delete
```

**Export for analysis:**

```bash
# Export yearly data for research
joshua history --from 2025-01-01 --to 2025-12-31 --format csv > 2025_assessments.csv
```

### 8.5 Security Best Practices

**1. Protect API keys**
```bash
# Use environment variables
export ANTHROPIC_API_KEY=$(security find-generic-password -a joshua -s anthropic -w)

# Or secrets management
export ANTHROPIC_API_KEY=$(vault kv get -field=api_key secret/joshua)
```

**2. Encrypt configuration**
```bash
# Encrypt config file
gpg --encrypt --recipient your@email.com config.toml

# Decrypt for use
joshua --config <(gpg --decrypt config.toml.gpg) assess
```

**3. Restrict database access**
```sql
-- Create read-only user for reports
CREATE USER joshua_readonly WITH PASSWORD 'password';
GRANT CONNECT ON DATABASE joshua TO joshua_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO joshua_readonly;
```

**4. Audit logging**
```toml
[security]
audit_log_enabled = true
audit_log_file = "~/.local/share/joshua/audit.log"
```

**5. Regular security updates**
```bash
# Update dependencies
cargo update

# Rebuild with latest dependencies
cargo build --release
```

---

## 9. API Usage

### 9.1 Enabling the API Server

Enable REST API access in configuration:

```toml
[api]
enabled = true
host = "0.0.0.0"
port = 8080
workers = 4
require_auth = true
```

Start server:

```bash
# Standalone server
joshua serve

# Or with systemd
sudo systemctl start joshua-api
```

### 9.2 Authentication

Obtain JWT token:

```bash
curl -X POST http://localhost:8080/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password",
    "mfa_code": "123456"
  }'
```

Response:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

Use token in requests:

```bash
curl -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  http://localhost:8080/v1/assessments/latest
```

### 9.3 Common API Endpoints

**Get latest assessment:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/v1/assessments/latest
```

**List assessments:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/v1/assessments?limit=10&offset=0"
```

**Get specific assessment:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8080/v1/assessments/asmt_2025_10_27_120000
```

**Trigger new assessment:**

```bash
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  http://localhost:8080/v1/assessments
```

**Get trends:**

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8080/v1/trends?period=30d"
```

**Health check:**

```bash
curl http://localhost:8080/health
```

For complete API documentation, see [docs/09_API_Reference.md](09_API_Reference.md).

---

## 10. Glossary

**Assessment** - Complete nuclear risk evaluation including data collection, analysis, calculation, and reporting.

**Bayesian Network** - Statistical model that represents probabilistic relationships between risk factors using conditional probabilities.

**Bulletin of Atomic Scientists** - Organization that publishes the Doomsday Clock, founded in 1945 by Manhattan Project scientists.

**Claude API** - Anthropic's AI API used for intelligent analysis of nuclear risk data.

**Confidence Level** - Measure of certainty in assessment accuracy: VeryLow, Low, Moderate, High, VeryHigh.

**Data Point** - Single piece of collected information from a source (article, report, etc.).

**Doomsday Clock** - Metaphorical clock showing time remaining until "midnight" (nuclear catastrophe), measured in seconds to midnight.

**Monte Carlo Simulation** - Statistical technique using repeated random sampling to model uncertainty and probability distributions.

**Risk Factor** - Individual component contributing to overall nuclear risk (e.g., "Russia-Ukraine Conflict").

**Risk Category** - Grouping of related risk factors (e.g., Regional Conflicts, Nuclear Arsenal Changes).

**Seconds to Midnight** - Risk score representation on Doomsday Clock scale (0 = nuclear war, 1440 = minimal risk).

**SIPRI** - Stockholm International Peace Research Institute, authoritative source for nuclear arsenal data.

**Trend Direction** - Direction of risk movement: Increasing (↗), Decreasing (↘), Stable (→), or Uncertain (?).

**Weighted Scoring** - Risk calculation method where each category receives a weight based on importance.

---

**Document Version:** 1.0.0
**Last Updated:** October 27, 2025
**Maintained By:** WarGames/JOSHUA Development Team
**Next Review:** November 2025

---

*"Greetings, Professor Falken. Shall we play a game?"*

*"A strange game. The only winning move is not to play. How about a nice game of chess?"*

**WarGames/JOSHUA monitors the game so we know when the stakes are highest.**

---

For technical implementation details, see:
- [Architecture Guide](06_Architecture_and_Implementation_Guide.md)
- [API Reference](09_API_Reference.md)
- [Security Specifications](08_Security_Implementation_Specifications.md)
- [Contributing Guide](14_Contributing_Guide.md)

For operational details, see:
- [Deployment Guide](07_Deployment_and_Operations_Guide.md)
- [Monitoring Guide](11_Monitoring_and_Observability.md)
- [Disaster Recovery](12_Disaster_Recovery_and_Business_Continuity.md)
