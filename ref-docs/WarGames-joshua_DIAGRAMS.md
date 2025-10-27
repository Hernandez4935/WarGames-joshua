# WarGames/JOSHUA: Comprehensive System Diagrams
## Visual Architecture and Design Documentation
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [System Architecture Overview](#1-system-architecture-overview)
2. [Module Hierarchy](#2-module-hierarchy)
3. [Data Flow Diagrams](#3-data-flow-diagrams)
4. [Database Schema (ERD)](#4-database-schema-erd)
5. [Risk Calculation Workflow](#5-risk-calculation-workflow)
6. [Claude API Integration](#6-claude-api-integration)
7. [Data Collection Process](#7-data-collection-process)
8. [Component Interaction](#8-component-interaction)
9. [Deployment Architecture](#9-deployment-architecture)
10. [State Machines](#10-state-machines)
11. [Testing Architecture](#11-testing-architecture)
12. [Security Architecture](#12-security-architecture)
13. [Scheduling and Automation](#13-scheduling-and-automation)
14. [Error Handling Flow](#14-error-handling-flow)
15. [Notification System](#15-notification-system)

---

## 1. System Architecture Overview

### 1.1 High-Level Component Architecture

```mermaid
graph TB
    subgraph "User Interfaces"
        CLI[CLI Interface<br/>clap]
        TUI[Terminal UI<br/>ratatui]
        API[REST API<br/>axum]
    end

    subgraph "Core Orchestration Layer"
        WGSYS[WarGamesSystem<br/>Main Coordinator]
        SCHED[Task Scheduler<br/>Cron Engine]
    end

    subgraph "Data Collection Layer"
        DCE[Data Collection<br/>Engine]
        NC[News Collectors]
        RC[Research Collectors]
        GC[Gov't Collectors]
        SC[Social Collectors]
    end

    subgraph "Analysis Layer"
        CAE[Claude Analysis<br/>Engine]
        RCE[Risk Calculation<br/>Engine]
        ANAL[Specialized<br/>Analyzers]
    end

    subgraph "Output Layer"
        VIZ[Visualization<br/>Engine]
        RPT[Report<br/>Generator]
        NOTIF[Notification<br/>System]
    end

    subgraph "Data Persistence"
        DB[(PostgreSQL<br/>Database)]
        CACHE[(Redis<br/>Cache)]
        FS[File System<br/>Reports/Charts]
    end

    subgraph "External Systems"
        CLAUDE[Anthropic<br/>Claude API]
        NEWS[News APIs]
        SIPRI[SIPRI Database]
        GOV[Gov't Sources]
    end

    CLI --> WGSYS
    TUI --> WGSYS
    API --> WGSYS
    
    WGSYS --> DCE
    WGSYS --> CAE
    WGSYS --> RCE
    WGSYS --> VIZ
    WGSYS --> RPT
    WGSYS --> NOTIF
    
    SCHED --> WGSYS
    
    DCE --> NC
    DCE --> RC
    DCE --> GC
    DCE --> SC
    
    NC --> NEWS
    RC --> SIPRI
    GC --> GOV
    
    DCE --> CACHE
    DCE --> DB
    
    CAE --> CLAUDE
    CAE --> DB
    
    RCE --> ANAL
    RCE --> DB
    
    VIZ --> FS
    RPT --> FS
    RPT --> DB
    
    NOTIF --> DB

    style WGSYS fill:#ff6b6b
    style CAE fill:#4ecdc4
    style RCE fill:#ffe66d
    style DB fill:#95e1d3
    style CLAUDE fill:#f38181
```

### 1.2 Layered Architecture View

```mermaid
graph LR
    subgraph "Presentation Layer"
        CLI1[CLI]
        TUI1[TUI]
        API1[API]
    end

    subgraph "Application Layer"
        ORCH[Orchestrator]
        SCHED1[Scheduler]
        VALID[Validators]
    end

    subgraph "Business Logic Layer"
        COLLECT[Collection]
        ANALYZE[Analysis]
        CALC[Calculation]
        REPORT[Reporting]
    end

    subgraph "Data Access Layer"
        REPO[Repositories]
        CACHE1[Cache Manager]
        FILE[File Manager]
    end

    subgraph "Infrastructure Layer"
        DB1[(Database)]
        REDIS1[(Cache)]
        FS1[File System]
        EXT[External APIs]
    end

    CLI1 --> ORCH
    TUI1 --> ORCH
    API1 --> ORCH
    
    ORCH --> COLLECT
    ORCH --> ANALYZE
    ORCH --> CALC
    ORCH --> REPORT
    
    SCHED1 --> ORCH
    
    COLLECT --> REPO
    ANALYZE --> REPO
    CALC --> REPO
    REPORT --> REPO
    
    REPO --> DB1
    CACHE1 --> REDIS1
    FILE --> FS1
    COLLECT --> EXT
    ANALYZE --> EXT
    
    VALID -.validates.-> COLLECT
    VALID -.validates.-> ANALYZE

    style ORCH fill:#ff6b6b
    style COLLECT fill:#4ecdc4
    style ANALYZE fill:#ffe66d
    style CALC fill:#95e1d3
```

---

## 2. Module Hierarchy

### 2.1 Rust Crate Structure

```mermaid
graph TD
    ROOT[wargames]
    
    ROOT --> LIB[lib.rs]
    ROOT --> MAIN[main.rs]
    ROOT --> CLI[cli/]
    ROOT --> ENG[engines/]
    ROOT --> COLL[collectors/]
    ROOT --> ANAL[analyzers/]
    ROOT --> MOD[models/]
    ROOT --> VIZ[visualizers/]
    ROOT --> UTIL[utils/]
    ROOT --> ERR[error.rs]
    ROOT --> TYPES[types.rs]
    ROOT --> CONST[constants.rs]
    
    CLI --> CLICMD[commands.rs]
    CLI --> ASSESS[assess.rs]
    CLI --> REPORT[report.rs]
    CLI --> TREND[trend.rs]
    CLI --> SCHED[schedule.rs]
    CLI --> INTER[interactive.rs]
    
    ENG --> DATACOLL[data_collection.rs]
    ENG --> CLAUDE[claude_integration.rs]
    ENG --> RISK[risk_calculation.rs]
    ENG --> VIZENG[visualization.rs]
    ENG --> RPTGEN[report_generation.rs]
    ENG --> STORAGE[storage.rs]
    ENG --> NOTIF[notifications.rs]
    ENG --> SCHEDENG[scheduler.rs]
    ENG --> TERMUI[terminal_ui.rs]
    
    COLL --> COLLBASE[base.rs]
    COLL --> NEWS[news/]
    COLL --> RESEARCH[research/]
    COLL --> GOVT[government/]
    COLL --> SOCIAL[social/]
    
    NEWS --> RSS[rss.rs]
    NEWS --> REUTERS[reuters.rs]
    NEWS --> AP[associated_press.rs]
    
    RESEARCH --> SIPRI[sipri.rs]
    RESEARCH --> CARNEGIE[carnegie.rs]
    RESEARCH --> BULLETIN[bulletin.rs]
    
    GOVT --> STATE[state_dept.rs]
    GOVT --> IAEA[iaea.rs]
    
    SOCIAL --> TWITTER[twitter.rs]
    
    ANAL --> ANALBASE[base.rs]
    ANAL --> ARSENAL[arsenal.rs]
    ANAL --> CONFLICT[conflict.rs]
    ANAL --> RHETORIC[rhetoric.rs]
    ANAL --> INCIDENT[incident.rs]
    ANAL --> TREATY[treaty.rs]
    
    MOD --> ASSESSMENT[assessment.rs]
    MOD --> RISKFACTOR[risk_factor.rs]
    MOD --> NUCARSENAL[nuclear_arsenal.rs]
    MOD --> GEOPOLITICAL[geopolitical.rs]
    MOD --> TREATYSTATUS[treaty_status.rs]
    MOD --> HISTORICAL[historical_data.rs]
    MOD --> DATAPOINT[data_point.rs]
    
    VIZ --> DOOMSDAY[doomsday_clock.rs]
    VIZ --> TRENDCHARTS[trend_charts.rs]
    VIZ --> RISKMATRIX[risk_matrix.rs]
    VIZ --> HEATMAPS[heat_maps.rs]
    VIZ --> DASHBOARD[dashboard.rs]
    
    UTIL --> CONFIG[config.rs]
    UTIL --> CRYPTO[crypto.rs]
    UTIL --> CACHE[cache.rs]
    UTIL --> RATELIMITER[rate_limiter.rs]
    UTIL --> RETRY[retry.rs]
    UTIL --> TEXT[text.rs]
    UTIL --> TIME[time.rs]

    style ROOT fill:#ff6b6b
    style ENG fill:#4ecdc4
    style COLL fill:#ffe66d
    style ANAL fill:#95e1d3
    style MOD fill:#f9ca24
```

### 2.2 Engine Dependencies

```mermaid
graph TD
    WGSYS[WarGamesSystem]
    
    subgraph "Core Engines"
        DCE[DataCollectionEngine]
        CAE[ClaudeAnalysisEngine]
        RCE[RiskCalculationEngine]
        VE[VisualizationEngine]
        RGE[ReportGenerationEngine]
        PSE[PersistentStorageEngine]
        ANE[AlertNotificationEngine]
        TSE[TaskSchedulingEngine]
    end
    
    subgraph "Support Systems"
        CONFIG[Config]
        CACHE[Cache]
        CRYPTO[Crypto]
        LOGGER[Logger]
    end
    
    WGSYS --> DCE
    WGSYS --> CAE
    WGSYS --> RCE
    WGSYS --> VE
    WGSYS --> RGE
    WGSYS --> PSE
    WGSYS --> ANE
    WGSYS --> TSE
    
    DCE --> CACHE
    DCE --> CONFIG
    
    CAE --> CONFIG
    CAE --> CRYPTO
    CAE --> CACHE
    
    RCE --> PSE
    RCE --> CONFIG
    
    VE --> CONFIG
    
    RGE --> CONFIG
    RGE --> VE
    
    PSE --> CONFIG
    
    ANE --> CONFIG
    
    TSE --> CONFIG
    
    DCE -.data.-> CAE
    CAE -.analysis.-> RCE
    RCE -.scores.-> VE
    RCE -.scores.-> RGE
    RCE -.scores.-> ANE
    
    VE -.charts.-> RGE
    
    DCE -.stores.-> PSE
    CAE -.stores.-> PSE
    RCE -.stores.-> PSE
    ANE -.stores.-> PSE

    style WGSYS fill:#ff6b6b
    style CAE fill:#4ecdc4
    style RCE fill:#ffe66d
    style PSE fill:#95e1d3
```

---

## 3. Data Flow Diagrams

### 3.1 Complete Assessment Flow

```mermaid
flowchart TD
    START([Start Assessment])
    
    INIT[Initialize System<br/>Load Configuration]
    
    subgraph COLLECTION["Data Collection Phase"]
        SCHEDULE[Check Schedule/<br/>Manual Trigger]
        PARALLEL[Parallel Collection<br/>50+ Sources]
        NEWS_COLLECT[News APIs<br/>RSS Feeds]
        RESEARCH_COLLECT[SIPRI<br/>Carnegie<br/>Bulletin]
        GOVT_COLLECT[State Dept<br/>IAEA<br/>UN]
        SOCIAL_COLLECT[Social Media<br/>Intelligence]
        AGGREGATE[Aggregate & Deduplicate]
        VALIDATE[Validate Data Quality]
        CACHE_STORE[Cache Results]
    end
    
    subgraph ANALYSIS["AI Analysis Phase"]
        PREP_PROMPT[Prepare Analysis Prompt<br/>Include Historical Context]
        CLAUDE_CALL[Call Claude API<br/>Multiple Analyses]
        PARSE_RESPONSE[Parse Structured Response]
        EXTRACT_FACTORS[Extract Risk Factors]
        VALIDATE_ANALYSIS[Validate Analysis Coherence]
        CONSENSUS[Build Consensus from<br/>Multiple Analyses]
    end
    
    subgraph CALCULATION["Risk Calculation Phase"]
        WEIGHT_SCORE[Calculate Weighted<br/>Base Score]
        BAYESIAN[Apply Bayesian<br/>Adjustment]
        TREND_CALC[Calculate Trend<br/>Momentum]
        MONTE_CARLO[Run Monte Carlo<br/>Simulation 10K+]
        UNCERTAINTY[Quantify Uncertainty<br/>Confidence Intervals]
        FINAL_SCORE[Generate Final<br/>Risk Score]
    end
    
    subgraph OUTPUT["Output Generation Phase"]
        GEN_VIZ[Generate Visualizations<br/>Charts & Graphs]
        GEN_REPORT[Generate Markdown<br/>Report]
        STORE_DB[Store in Database]
        CHECK_ALERTS[Check Alert<br/>Thresholds]
        SEND_NOTIF[Send Notifications<br/>If Needed]
    end
    
    END([Complete])
    
    START --> INIT
    INIT --> SCHEDULE
    
    SCHEDULE --> PARALLEL
    PARALLEL --> NEWS_COLLECT
    PARALLEL --> RESEARCH_COLLECT
    PARALLEL --> GOVT_COLLECT
    PARALLEL --> SOCIAL_COLLECT
    
    NEWS_COLLECT --> AGGREGATE
    RESEARCH_COLLECT --> AGGREGATE
    GOVT_COLLECT --> AGGREGATE
    SOCIAL_COLLECT --> AGGREGATE
    
    AGGREGATE --> VALIDATE
    VALIDATE --> CACHE_STORE
    CACHE_STORE --> PREP_PROMPT
    
    PREP_PROMPT --> CLAUDE_CALL
    CLAUDE_CALL --> PARSE_RESPONSE
    PARSE_RESPONSE --> EXTRACT_FACTORS
    EXTRACT_FACTORS --> VALIDATE_ANALYSIS
    VALIDATE_ANALYSIS --> CONSENSUS
    
    CONSENSUS --> WEIGHT_SCORE
    WEIGHT_SCORE --> BAYESIAN
    BAYESIAN --> TREND_CALC
    TREND_CALC --> MONTE_CARLO
    MONTE_CARLO --> UNCERTAINTY
    UNCERTAINTY --> FINAL_SCORE
    
    FINAL_SCORE --> GEN_VIZ
    FINAL_SCORE --> GEN_REPORT
    GEN_VIZ --> STORE_DB
    GEN_REPORT --> STORE_DB
    
    STORE_DB --> CHECK_ALERTS
    CHECK_ALERTS --> SEND_NOTIF
    SEND_NOTIF --> END
    
    style START fill:#95e1d3
    style COLLECTION fill:#4ecdc4
    style ANALYSIS fill:#ffe66d
    style CALCULATION fill:#f9ca24
    style OUTPUT fill:#f38181
    style END fill:#95e1d3
```

### 3.2 Data Collection Detail Flow

```mermaid
sequenceDiagram
    participant ORCH as Orchestrator
    participant DCE as DataCollectionEngine
    participant NC as NewsCollector
    participant RC as ResearchCollector
    participant GC as GovtCollector
    participant CACHE as Cache
    participant DB as Database
    
    ORCH->>DCE: collect_all_data()
    activate DCE
    
    Note over DCE: Check cache for recent data
    DCE->>CACHE: get(cache_key)
    CACHE-->>DCE: cached_data | None
    
    alt Cache Hit (< 6 hours old)
        DCE-->>ORCH: Return cached data
    else Cache Miss or Stale
        par Parallel Collection
            DCE->>NC: collect()
            activate NC
            NC->>NC: Fetch RSS feeds
            NC->>NC: Call News APIs
            NC->>NC: Deduplicate articles
            NC-->>DCE: news_data
            deactivate NC
        and
            DCE->>RC: collect()
            activate RC
            RC->>RC: Query SIPRI database
            RC->>RC: Fetch Carnegie data
            RC->>RC: Parse Bulletin updates
            RC-->>DCE: research_data
            deactivate RC
        and
            DCE->>GC: collect()
            activate GC
            GC->>GC: Scrape State Dept
            GC->>GC: Query IAEA
            GC->>GC: Parse UN reports
            GC-->>DCE: govt_data
            deactivate GC
        end
        
        Note over DCE: Aggregate all sources
        DCE->>DCE: aggregate_data()
        DCE->>DCE: deduplicate_content()
        DCE->>DCE: validate_quality()
        
        Note over DCE: Store results
        DCE->>CACHE: set(cache_key, data, ttl=6h)
        DCE->>DB: store_collected_data(data)
        
        DCE-->>ORCH: aggregated_data
    end
    
    deactivate DCE
```

---

## 4. Database Schema (ERD)

### 4.1 Core Schema

```mermaid
erDiagram
    ASSESSMENTS ||--o{ RISK_FACTORS : contains
    ASSESSMENTS ||--o{ ALERTS : triggers
    ASSESSMENTS ||--o{ VISUALIZATIONS : generates
    ASSESSMENTS }o--|| USERS : created_by
    
    RISK_FACTORS }o--|| DATA_SOURCES : sourced_from
    
    COLLECTED_DATA }o--|| DATA_SOURCES : collected_from
    COLLECTED_DATA }o--o{ ASSESSMENTS : used_in
    
    NUCLEAR_ARSENALS ||--o{ ARSENAL_CHANGES : tracks
    
    GEOPOLITICAL_EVENTS }o--|| COUNTRIES : involves
    GEOPOLITICAL_EVENTS }o--o{ ASSESSMENTS : influences
    
    TREATIES ||--o{ TREATY_EVENTS : has
    
    ASSESSMENTS {
        uuid id PK
        timestamp assessment_date
        integer seconds_to_midnight
        decimal raw_risk_score
        decimal base_risk_score
        decimal bayesian_adjusted_score
        varchar overall_confidence
        decimal data_quality_score
        varchar trend_direction
        decimal trend_magnitude
        integer delta_from_previous
        text executive_summary
        text detailed_analysis
        text_array recommendations
        varchar claude_model_version
        integer data_sources_count
        integer collection_duration_seconds
        integer analysis_duration_seconds
        timestamp created_at
        timestamp updated_at
        varchar created_by
    }
    
    RISK_FACTORS {
        uuid id PK
        uuid assessment_id FK
        varchar factor_category
        varchar factor_name
        varchar factor_type
        decimal raw_value
        decimal weighted_value
        decimal category_weight
        decimal contribution_to_risk
        integer rank_in_category
        integer rank_overall
        varchar confidence_level
        decimal confidence_score
        text_array data_sources
        text evidence_summary
        text contrary_evidence
        text historical_context
        decimal comparison_to_baseline
        timestamp observed_at
        timestamp created_at
    }
    
    DATA_SOURCES {
        uuid id PK
        varchar source_name
        varchar source_type
        varchar source_category
        varchar base_url
        decimal reliability_score
        jsonb metadata
        boolean is_active
        timestamp last_accessed
        timestamp created_at
        timestamp updated_at
    }
    
    COLLECTED_DATA {
        uuid id PK
        uuid source_id FK
        varchar category
        text content
        text summary
        text_array keywords
        decimal relevance_score
        decimal quality_score
        jsonb metadata
        timestamp timestamp
        timestamp collected_at
        timestamp created_at
    }
    
    NUCLEAR_ARSENALS {
        uuid id PK
        varchar country_code
        varchar country_name
        integer total_warheads
        integer deployed_strategic
        integer deployed_tactical
        integer reserve_warheads
        integer retired_awaiting_dismantlement
        text delivery_systems
        text modernization_programs
        varchar data_source
        date as_of_date
        timestamp created_at
        timestamp updated_at
    }
    
    ARSENAL_CHANGES {
        uuid id PK
        uuid arsenal_id FK
        varchar change_type
        integer previous_count
        integer new_count
        integer delta
        text description
        text significance
        varchar data_source
        timestamp observed_at
        timestamp created_at
    }
    
    GEOPOLITICAL_EVENTS {
        uuid id PK
        varchar event_type
        varchar event_category
        varchar severity
        text description
        text_array countries_involved
        text_array regions_affected
        decimal impact_score
        text analysis
        text_array data_sources
        timestamp event_date
        timestamp created_at
        timestamp updated_at
    }
    
    COUNTRIES {
        varchar country_code PK
        varchar country_name
        varchar region
        boolean is_nuclear_power
        boolean is_npt_member
        varchar political_system
        jsonb metadata
        timestamp created_at
        timestamp updated_at
    }
    
    TREATIES {
        uuid id PK
        varchar treaty_name
        varchar treaty_type
        text description
        text_array signatories
        date effective_date
        date expiration_date
        varchar status
        text key_provisions
        timestamp created_at
        timestamp updated_at
    }
    
    TREATY_EVENTS {
        uuid id PK
        uuid treaty_id FK
        varchar event_type
        varchar country_code
        text description
        text significance
        text_array data_sources
        timestamp event_date
        timestamp created_at
    }
    
    ALERTS {
        uuid id PK
        uuid assessment_id FK
        varchar alert_type
        varchar severity
        text message
        text details
        jsonb trigger_conditions
        boolean notification_sent
        timestamp sent_at
        timestamp created_at
    }
    
    VISUALIZATIONS {
        uuid id PK
        uuid assessment_id FK
        varchar viz_type
        varchar file_format
        text file_path
        jsonb parameters
        integer width
        integer height
        timestamp created_at
    }
    
    USERS {
        uuid id PK
        varchar username
        varchar email
        varchar role
        timestamp created_at
        timestamp updated_at
        timestamp last_login
    }
```

### 4.2 Indexing Strategy

```mermaid
graph TD
    subgraph "Primary Indexes"
        PK1[assessments.id]
        PK2[risk_factors.id]
        PK3[collected_data.id]
        PK4[nuclear_arsenals.id]
    end
    
    subgraph "Foreign Key Indexes"
        FK1[risk_factors.assessment_id]
        FK2[alerts.assessment_id]
        FK3[collected_data.source_id]
        FK4[arsenal_changes.arsenal_id]
    end
    
    subgraph "Query Optimization Indexes"
        IDX1[assessments.assessment_date DESC]
        IDX2[assessments.seconds_to_midnight]
        IDX3[assessments.trend_direction]
        IDX4[risk_factors.factor_category]
        IDX5[risk_factors.contribution_to_risk DESC]
        IDX6[collected_data.category, timestamp]
        IDX7[nuclear_arsenals.country_code, as_of_date]
    end
    
    subgraph "Full-Text Search"
        FTS1[collected_data.content]
        FTS2[assessments.executive_summary]
        FTS3[geopolitical_events.description]
    end
    
    style PK1 fill:#ff6b6b
    style PK2 fill:#ff6b6b
    style FK1 fill:#4ecdc4
    style FK2 fill:#4ecdc4
    style IDX1 fill:#ffe66d
    style IDX2 fill:#ffe66d
    style FTS1 fill:#95e1d3
    style FTS2 fill:#95e1d3
```

---

## 5. Risk Calculation Workflow

### 5.1 Multi-Stage Risk Calculation

```mermaid
flowchart TD
    START([Risk Factors Input])
    
    subgraph STAGE1["Stage 1: Base Weighted Scoring"]
        GROUP[Group Factors by Category]
        CATWEIGHT[Apply Category Weights]
        FACWEIGHT[Apply Factor Weights]
        CONFADJ[Confidence Adjustment]
        BASESUM[Sum Weighted Scores]
    end
    
    subgraph STAGE2["Stage 2: Bayesian Adjustment"]
        LOADHIST[Load Historical Data]
        CORR[Calculate Factor Correlations]
        PRIOR[Establish Prior Beliefs]
        EVIDENCE[Update with New Evidence]
        POSTERIOR[Calculate Posterior Probability]
    end
    
    subgraph STAGE3["Stage 3: Trend Analysis"]
        TIMESERIES[Extract Time Series]
        MOMENTUM[Calculate Momentum]
        PATTERNS[Pattern Matching]
        FORECAST[Short-term Forecast]
        TRENDADJ[Trend Adjustment Factor]
    end
    
    subgraph STAGE4["Stage 4: Monte Carlo Simulation"]
        DEFINE[Define Input Distributions]
        SAMPLE[Generate 10K+ Samples]
        SIMULATE[Simulate Scenarios]
        AGGREGATE[Aggregate Results]
        PERCENTILES[Calculate Percentiles]
    end
    
    subgraph STAGE5["Stage 5: Uncertainty Quantification"]
        DATASRC[Data Source Quality]
        CONFLEVEL[Confidence Levels]
        INTERVALS[Confidence Intervals]
        SENSITIVITY[Sensitivity Analysis]
    end
    
    FINAL[Final Risk Score<br/>Seconds to Midnight]
    OUTPUT[Output Package:<br/>- Score<br/>- Confidence<br/>- Trend<br/>- Drivers]
    END([Complete])
    
    START --> GROUP
    GROUP --> CATWEIGHT
    CATWEIGHT --> FACWEIGHT
    FACWEIGHT --> CONFADJ
    CONFADJ --> BASESUM
    
    BASESUM --> LOADHIST
    LOADHIST --> CORR
    CORR --> PRIOR
    PRIOR --> EVIDENCE
    EVIDENCE --> POSTERIOR
    
    POSTERIOR --> TIMESERIES
    TIMESERIES --> MOMENTUM
    MOMENTUM --> PATTERNS
    PATTERNS --> FORECAST
    FORECAST --> TRENDADJ
    
    TRENDADJ --> DEFINE
    DEFINE --> SAMPLE
    SAMPLE --> SIMULATE
    SIMULATE --> AGGREGATE
    AGGREGATE --> PERCENTILES
    
    PERCENTILES --> DATASRC
    DATASRC --> CONFLEVEL
    CONFLEVEL --> INTERVALS
    INTERVALS --> SENSITIVITY
    
    SENSITIVITY --> FINAL
    FINAL --> OUTPUT
    OUTPUT --> END
    
    style START fill:#95e1d3
    style STAGE1 fill:#4ecdc4
    style STAGE2 fill:#ffe66d
    style STAGE3 fill:#f9ca24
    style STAGE4 fill:#f38181
    style STAGE5 fill:#aa96da
    style END fill:#95e1d3
```

### 5.2 Factor Weighting Hierarchy

```mermaid
graph TD
    ROOT[Total Risk Score: 1.0]
    
    subgraph CATEGORIES["Risk Categories"]
        CAT1[Arsenal Changes<br/>0.15]
        CAT2[Doctrine & Posture<br/>0.15]
        CAT3[Regional Conflicts<br/>0.20]
        CAT4[Leadership Rhetoric<br/>0.10]
        CAT5[Technical Incidents<br/>0.15]
        CAT6[Communication Breakdown<br/>0.10]
        CAT7[Emerging Technology<br/>0.10]
        CAT8[Economic Factors<br/>0.05]
    end
    
    ROOT --> CAT1
    ROOT --> CAT2
    ROOT --> CAT3
    ROOT --> CAT4
    ROOT --> CAT5
    ROOT --> CAT6
    ROOT --> CAT7
    ROOT --> CAT8
    
    CAT1 --> F1A[Modernization Programs<br/>0.40]
    CAT1 --> F1B[Warhead Count Changes<br/>0.35]
    CAT1 --> F1C[Delivery System Advances<br/>0.25]
    
    CAT2 --> F2A[Threshold Changes<br/>0.35]
    CAT2 --> F2B[Alert Status<br/>0.30]
    CAT2 --> F2C[No-First-Use Policy<br/>0.20]
    CAT2 --> F2D[Declaratory Policy<br/>0.15]
    
    CAT3 --> F3A[Active Armed Conflicts<br/>0.40]
    CAT3 --> F3B[Territorial Disputes<br/>0.30]
    CAT3 --> F3C[Proxy Warfare<br/>0.20]
    CAT3 --> F3D[Military Incidents<br/>0.10]
    
    CAT4 --> F4A[Nuclear Threats<br/>0.45]
    CAT4 --> F4B[Leadership Stability<br/>0.30]
    CAT4 --> F4C[Domestic Pressure<br/>0.25]
    
    CAT5 --> F5A[Airspace Violations<br/>0.30]
    CAT5 --> F5B[False Alarms<br/>0.35]
    CAT5 --> F5C[Cyber Attacks<br/>0.25]
    CAT5 --> F5D[Accidents<br/>0.10]
    
    style ROOT fill:#ff6b6b
    style CAT1 fill:#4ecdc4
    style CAT2 fill:#4ecdc4
    style CAT3 fill:#ffe66d
    style CAT4 fill:#ffe66d
    style CAT5 fill:#95e1d3
```

---

## 6. Claude API Integration

### 6.1 Analysis Request Sequence

```mermaid
sequenceDiagram
    participant ORCH as Orchestrator
    participant CAE as ClaudeAnalysisEngine
    participant CTX as ContextManager
    participant PROMPT as PromptBuilder
    participant CLIENT as AnthropicClient
    participant CLAUDE as Claude API
    participant PARSER as ResponseParser
    participant VALID as Validator
    participant DB as Database
    
    ORCH->>CAE: analyze_risk(data)
    activate CAE
    
    Note over CAE: Prepare analysis request
    CAE->>CTX: load_historical_context()
    activate CTX
    CTX->>DB: query_previous_assessments(limit=10)
    DB-->>CTX: historical_data
    CTX-->>CAE: context
    deactivate CTX
    
    CAE->>PROMPT: build_analysis_prompt(data, context)
    activate PROMPT
    PROMPT->>PROMPT: format_data()
    PROMPT->>PROMPT: include_instructions()
    PROMPT->>PROMPT: add_schema()
    PROMPT-->>CAE: structured_prompt
    deactivate PROMPT
    
    Note over CAE: Execute analysis (3-5 runs)
    loop Multiple Analyses (N=3)
        CAE->>CLIENT: send_message(prompt)
        activate CLIENT
        
        CLIENT->>CLIENT: apply_rate_limiting()
        CLIENT->>CLIENT: add_auth_headers()
        
        CLIENT->>CLAUDE: POST /v1/messages
        activate CLAUDE
        
        Note over CLAUDE: Model: claude-sonnet-4<br/>Max Tokens: 8000<br/>Temperature: 0.3
        
        CLAUDE-->>CLIENT: analysis_response
        deactivate CLAUDE
        
        CLIENT-->>CAE: raw_response
        deactivate CLIENT
        
        CAE->>PARSER: parse_response(raw_response)
        activate PARSER
        PARSER->>PARSER: extract_json()
        PARSER->>PARSER: parse_risk_factors()
        PARSER->>PARSER: parse_scores()
        PARSER-->>CAE: parsed_analysis
        deactivate PARSER
        
        CAE->>VALID: validate_analysis(parsed_analysis)
        activate VALID
        VALID->>VALID: check_schema()
        VALID->>VALID: validate_ranges()
        VALID->>VALID: check_consistency()
        VALID-->>CAE: validation_result
        deactivate VALID
        
        alt Validation Failed
            CAE->>CLIENT: retry_analysis()
        else Validation Passed
            CAE->>CAE: store_analysis(parsed_analysis)
        end
    end
    
    Note over CAE: Build consensus
    CAE->>CAE: aggregate_analyses()
    CAE->>CAE: detect_disagreements()
    CAE->>CAE: build_consensus()
    
    CAE->>DB: store_analysis_result(consensus)
    DB-->>CAE: stored
    
    CAE-->>ORCH: consensus_analysis
    deactivate CAE
```

### 6.2 Prompt Engineering Flow

```mermaid
flowchart LR
    START([Raw Data])
    
    subgraph PREP["Data Preparation"]
        CLEAN[Clean & Normalize]
        FILTER[Filter Relevant Items]
        SUMMARIZE[Summarize Key Points]
        STRUCTURE[Structure by Category]
    end
    
    subgraph CONTEXT["Context Building"]
        HIST[Load Historical<br/>Assessments]
        TRENDS[Extract Trends]
        BASELINE[Establish Baseline]
        CHANGES[Identify Changes]
    end
    
    subgraph PROMPT["Prompt Construction"]
        SYSTEM[System Prompt:<br/>- Role Definition<br/>- Analysis Framework<br/>- Output Format]
        USER[User Prompt:<br/>- Current Data<br/>- Historical Context<br/>- Specific Questions]
        SCHEMA[JSON Schema:<br/>- Risk Factors<br/>- Scores<br/>- Explanations]
        EXAMPLES[Few-Shot Examples:<br/>- Previous Analyses<br/>- Expected Format]
    end
    
    subgraph EXEC["Execution"]
        API[Claude API Call]
        TEMP[Temperature: 0.3<br/>For Consistency]
        TOKENS[Max Tokens: 8000<br/>Comprehensive Output]
    end
    
    RESPONSE([Analysis Response])
    
    START --> CLEAN
    CLEAN --> FILTER
    FILTER --> SUMMARIZE
    SUMMARIZE --> STRUCTURE
    
    STRUCTURE --> HIST
    HIST --> TRENDS
    TRENDS --> BASELINE
    BASELINE --> CHANGES
    
    CHANGES --> SYSTEM
    STRUCTURE --> USER
    SYSTEM --> API
    USER --> API
    SCHEMA --> API
    EXAMPLES --> API
    TEMP --> API
    TOKENS --> API
    
    API --> RESPONSE
    
    style START fill:#95e1d3
    style PREP fill:#4ecdc4
    style CONTEXT fill:#ffe66d
    style PROMPT fill:#f9ca24
    style EXEC fill:#f38181
    style RESPONSE fill:#95e1d3
```

---

## 7. Data Collection Process

### 7.1 Parallel Collection Architecture

```mermaid
graph TB
    START[Data Collection<br/>Trigger]
    
    subgraph ORCHESTRATOR["Collection Orchestrator"]
        SCHED[Check Schedule]
        INIT[Initialize Collectors]
        DISPATCH[Dispatch Tasks]
        MONITOR[Monitor Progress]
        TIMEOUT[Handle Timeouts]
    end
    
    subgraph NEWS["News Collection (Parallel)"]
        RSS[RSS Feeds<br/>50+ sources]
        REUTERS[Reuters API]
        AP[Associated Press]
        BBC[BBC World Service]
        AJ[Al Jazeera]
    end
    
    subgraph RESEARCH["Research Collection (Parallel)"]
        SIPRI[SIPRI Database]
        CARNEGIE[Carnegie Endowment]
        BULLETIN[Bulletin of<br/>Atomic Scientists]
        ARMS[Arms Control<br/>Association]
        RAND[RAND Corporation]
    end
    
    subgraph GOVT["Government Collection (Parallel)"]
        STATE[State Department]
        IAEA[IAEA]
        UN[UN Security Council]
        DEFENSE[Defense Intelligence]
    end
    
    subgraph SOCIAL["Social Media (Parallel)"]
        TWITTER[Twitter/X API]
        REDDIT[Reddit r/worldnews]
    end
    
    subgraph PROCESSING["Data Processing"]
        DEDUP[Deduplication]
        QUALITY[Quality Scoring]
        RELEVANCE[Relevance Filtering]
        EXTRACT[Entity Extraction]
    end
    
    subgraph STORAGE["Storage Layer"]
        CACHE[(Cache<br/>6h TTL)]
        DATABASE[(Database)]
    end
    
    RESULT[Aggregated<br/>Data]
    
    START --> SCHED
    SCHED --> INIT
    INIT --> DISPATCH
    DISPATCH --> MONITOR
    
    DISPATCH --> RSS
    DISPATCH --> REUTERS
    DISPATCH --> AP
    DISPATCH --> BBC
    DISPATCH --> AJ
    
    DISPATCH --> SIPRI
    DISPATCH --> CARNEGIE
    DISPATCH --> BULLETIN
    DISPATCH --> ARMS
    DISPATCH --> RAND
    
    DISPATCH --> STATE
    DISPATCH --> IAEA
    DISPATCH --> UN
    DISPATCH --> DEFENSE
    
    DISPATCH --> TWITTER
    DISPATCH --> REDDIT
    
    RSS --> DEDUP
    REUTERS --> DEDUP
    AP --> DEDUP
    BBC --> DEDUP
    AJ --> DEDUP
    SIPRI --> DEDUP
    CARNEGIE --> DEDUP
    BULLETIN --> DEDUP
    ARMS --> DEDUP
    RAND --> DEDUP
    STATE --> DEDUP
    IAEA --> DEDUP
    UN --> DEDUP
    DEFENSE --> DEDUP
    TWITTER --> DEDUP
    REDDIT --> DEDUP
    
    DEDUP --> QUALITY
    QUALITY --> RELEVANCE
    RELEVANCE --> EXTRACT
    
    EXTRACT --> CACHE
    EXTRACT --> DATABASE
    
    CACHE --> RESULT
    DATABASE --> RESULT
    
    MONITOR -.timeout.-> TIMEOUT
    TIMEOUT -.retry.-> DISPATCH
    
    style START fill:#95e1d3
    style ORCHESTRATOR fill:#ff6b6b
    style NEWS fill:#4ecdc4
    style RESEARCH fill:#ffe66d
    style GOVT fill:#f9ca24
    style SOCIAL fill:#f38181
    style PROCESSING fill:#aa96da
    style STORAGE fill:#95e1d3
    style RESULT fill:#95e1d3
```

### 7.2 Data Quality Scoring

```mermaid
flowchart TD
    INPUT[Raw Data Point]
    
    subgraph SCORING["Quality Scoring Components"]
        SOURCE[Source Reliability<br/>0.0 - 1.0]
        FRESH[Freshness<br/>0.0 - 1.0]
        COMPLETE[Completeness<br/>0.0 - 1.0]
        VERIF[Verification Status<br/>0.0 - 1.0]
        RELEVANT[Relevance Score<br/>0.0 - 1.0]
    end
    
    WEIGHT[Apply Weights:<br/>Source: 0.30<br/>Freshness: 0.20<br/>Completeness: 0.20<br/>Verification: 0.15<br/>Relevance: 0.15]
    
    CALC[Calculate:<br/>weighted_sum / total_weights]
    
    THRESHOLD{Quality >= 0.5?}
    
    ACCEPT[Accept Data Point]
    REJECT[Reject Data Point]
    LOG[Log Rejection Reason]
    
    STORE[(Store with<br/>Quality Score)]
    
    INPUT --> SOURCE
    INPUT --> FRESH
    INPUT --> COMPLETE
    INPUT --> VERIF
    INPUT --> RELEVANT
    
    SOURCE --> WEIGHT
    FRESH --> WEIGHT
    COMPLETE --> WEIGHT
    VERIF --> WEIGHT
    RELEVANT --> WEIGHT
    
    WEIGHT --> CALC
    CALC --> THRESHOLD
    
    THRESHOLD -->|Yes| ACCEPT
    THRESHOLD -->|No| REJECT
    
    REJECT --> LOG
    
    ACCEPT --> STORE
    
    style INPUT fill:#95e1d3
    style SCORING fill:#4ecdc4
    style WEIGHT fill:#ffe66d
    style ACCEPT fill:#5fb96b
    style REJECT fill:#ff6b6b
    style STORE fill:#95e1d3
```

---

## 8. Component Interaction

### 8.1 Engine Communication Diagram

```mermaid
graph TB
    subgraph USER["User Interaction"]
        CLI[CLI Command]
        TUI[Terminal UI]
        API[REST API]
    end
    
    subgraph CORE["Core Orchestration"]
        WGSYS[WarGamesSystem<br/>Coordinator]
        CONFIG[Configuration<br/>Manager]
    end
    
    subgraph ENGINES["Processing Engines"]
        DCE[Data Collection<br/>Engine]
        CAE[Claude Analysis<br/>Engine]
        RCE[Risk Calculation<br/>Engine]
        VE[Visualization<br/>Engine]
        RGE[Report Generation<br/>Engine]
        PSE[Persistent Storage<br/>Engine]
        ANE[Alert Notification<br/>Engine]
        TSE[Task Scheduling<br/>Engine]
    end
    
    subgraph UTILS["Utilities"]
        CACHE[Cache Manager]
        CRYPTO[Crypto Utils]
        RATELIM[Rate Limiter]
        LOGGER[Logger]
    end
    
    subgraph EXTERNAL["External Systems"]
        CLAUDE_API[Claude API]
        NEWS_API[News APIs]
        RESEARCH_DB[Research DBs]
        POSTGRES[(PostgreSQL)]
        REDIS[(Redis)]
    end
    
    CLI --> WGSYS
    TUI --> WGSYS
    API --> WGSYS
    
    WGSYS <--> CONFIG
    
    WGSYS --> DCE
    WGSYS --> CAE
    WGSYS --> RCE
    WGSYS --> VE
    WGSYS --> RGE
    WGSYS --> PSE
    WGSYS --> ANE
    WGSYS --> TSE
    
    DCE --> CACHE
    DCE --> NEWS_API
    DCE --> RESEARCH_DB
    
    CAE --> CACHE
    CAE --> CRYPTO
    CAE --> RATELIM
    CAE --> CLAUDE_API
    
    RCE --> PSE
    
    VE --> RGE
    
    PSE --> POSTGRES
    CACHE --> REDIS
    
    DCE --> PSE
    CAE --> PSE
    RCE --> PSE
    ANE --> PSE
    
    DCE -.data.-> CAE
    CAE -.analysis.-> RCE
    RCE -.scores.-> VE
    RCE -.scores.-> RGE
    RCE -.scores.-> ANE
    
    LOGGER -.logs.-> DCE
    LOGGER -.logs.-> CAE
    LOGGER -.logs.-> RCE
    
    TSE --> WGSYS
    
    style WGSYS fill:#ff6b6b
    style DCE fill:#4ecdc4
    style CAE fill:#ffe66d
    style RCE fill:#f9ca24
    style PSE fill:#95e1d3
    style CLAUDE_API fill:#f38181
```

### 8.2 Event Flow Diagram

```mermaid
sequenceDiagram
    participant USER as User/Scheduler
    participant WGSYS as WarGamesSystem
    participant DCE as DataCollectionEngine
    participant CAE as ClaudeAnalysisEngine
    participant RCE as RiskCalculationEngine
    participant VE as VisualizationEngine
    participant RGE as ReportGenerationEngine
    participant ANE as AlertNotificationEngine
    participant PSE as PersistentStorageEngine
    
    USER->>WGSYS: run_assessment()
    activate WGSYS
    
    Note over WGSYS: Phase 1: Data Collection
    WGSYS->>DCE: collect_all_data()
    activate DCE
    DCE->>DCE: Parallel collection from 50+ sources
    DCE->>PSE: store_collected_data()
    DCE-->>WGSYS: aggregated_data
    deactivate DCE
    
    Note over WGSYS: Phase 2: AI Analysis
    WGSYS->>CAE: analyze_risk(data)
    activate CAE
    CAE->>PSE: load_historical_context()
    PSE-->>CAE: context
    CAE->>CAE: Multiple Claude API calls (3-5x)
    CAE->>CAE: Build consensus
    CAE->>PSE: store_analysis()
    CAE-->>WGSYS: risk_analysis
    deactivate CAE
    
    Note over WGSYS: Phase 3: Risk Calculation
    WGSYS->>RCE: calculate_risk(analysis)
    activate RCE
    RCE->>PSE: load_historical_scores()
    PSE-->>RCE: historical_data
    RCE->>RCE: Weighted scoring
    RCE->>RCE: Bayesian adjustment
    RCE->>RCE: Trend analysis
    RCE->>RCE: Monte Carlo simulation
    RCE->>PSE: store_risk_score()
    RCE-->>WGSYS: risk_score
    deactivate RCE
    
    Note over WGSYS: Phase 4: Visualization
    WGSYS->>VE: generate_visualizations(risk_score)
    activate VE
    VE->>VE: Create doomsday clock
    VE->>VE: Generate trend charts
    VE->>VE: Create risk matrix
    VE->>VE: Generate heat maps
    VE->>PSE: store_visualizations()
    VE-->>WGSYS: visualizations
    deactivate VE
    
    Note over WGSYS: Phase 5: Report Generation
    WGSYS->>RGE: generate_report(risk_score, viz)
    activate RGE
    RGE->>RGE: Create markdown report
    RGE->>RGE: Generate executive summary
    RGE->>RGE: Add recommendations
    RGE->>PSE: store_report()
    RGE-->>WGSYS: report
    deactivate RGE
    
    Note over WGSYS: Phase 6: Alert Check
    WGSYS->>ANE: check_and_notify(risk_score)
    activate ANE
    ANE->>ANE: Check alert thresholds
    alt Threshold Exceeded
        ANE->>ANE: Prepare notification
        ANE->>ANE: Send via webhook/email/slack
    end
    ANE->>PSE: store_alert()
    ANE-->>WGSYS: notification_status
    deactivate ANE
    
    Note over WGSYS: Complete Assessment
    WGSYS->>PSE: finalize_assessment()
    PSE-->>WGSYS: completed
    
    WGSYS-->>USER: Assessment {<br/>  risk_score,<br/>  report,<br/>  visualizations<br/>}
    deactivate WGSYS
```

---

## 9. Deployment Architecture

### 9.1 Production Deployment

```mermaid
graph TB
    subgraph INTERNET["Internet"]
        USER[Users]
        MONITOR[Monitoring<br/>Services]
    end
    
    subgraph DMZ["DMZ / Edge"]
        LB[Load Balancer<br/>Nginx/HAProxy]
        FW[Firewall]
    end
    
    subgraph APP["Application Tier"]
        APP1[WarGames<br/>Instance 1]
        APP2[WarGames<br/>Instance 2]
        APP3[WarGames<br/>Instance N]
        SCHED[Scheduler<br/>Instance]
    end
    
    subgraph CACHE["Cache Tier"]
        REDIS1[(Redis Master)]
        REDIS2[(Redis Replica)]
    end
    
    subgraph DATA["Data Tier"]
        PG_PRIMARY[(PostgreSQL<br/>Primary)]
        PG_STANDBY[(PostgreSQL<br/>Standby)]
    end
    
    subgraph STORAGE["File Storage"]
        S3[Object Storage<br/>Reports/Charts]
    end
    
    subgraph EXTERNAL["External Services"]
        CLAUDE[Claude API]
        NEWS[News APIs]
        RESEARCH[Research DBs]
    end
    
    subgraph OBSERVABILITY["Observability"]
        METRICS[Prometheus]
        LOGS[Loki/ELK]
        TRACES[Jaeger]
        DASH[Grafana]
    end
    
    USER --> LB
    LB --> FW
    FW --> APP1
    FW --> APP2
    FW --> APP3
    
    SCHED --> APP1
    
    APP1 --> REDIS1
    APP2 --> REDIS1
    APP3 --> REDIS1
    
    REDIS1 -.replication.-> REDIS2
    
    APP1 --> PG_PRIMARY
    APP2 --> PG_PRIMARY
    APP3 --> PG_PRIMARY
    
    PG_PRIMARY -.replication.-> PG_STANDBY
    
    APP1 --> S3
    APP2 --> S3
    APP3 --> S3
    
    APP1 --> CLAUDE
    APP2 --> CLAUDE
    APP3 --> CLAUDE
    
    APP1 --> NEWS
    APP2 --> NEWS
    APP3 --> NEWS
    
    APP1 --> RESEARCH
    APP2 --> RESEARCH
    APP3 --> RESEARCH
    
    APP1 --> METRICS
    APP2 --> METRICS
    APP3 --> METRICS
    
    APP1 --> LOGS
    APP2 --> LOGS
    APP3 --> LOGS
    
    APP1 --> TRACES
    APP2 --> TRACES
    APP3 --> TRACES
    
    METRICS --> DASH
    LOGS --> DASH
    TRACES --> DASH
    
    MONITOR --> DASH
    
    style USER fill:#95e1d3
    style LB fill:#4ecdc4
    style APP1 fill:#ff6b6b
    style APP2 fill:#ff6b6b
    style APP3 fill:#ff6b6b
    style REDIS1 fill:#ffe66d
    style PG_PRIMARY fill:#f9ca24
    style CLAUDE fill:#f38181
```

### 9.2 Container Architecture (Docker/K8s)

```mermaid
graph TB
    subgraph CLUSTER["Kubernetes Cluster"]
        subgraph NAMESPACE["wargames-prod namespace"]
            subgraph DEPLOY["Deployment: wargames-app"]
                POD1[Pod 1<br/>wargames container]
                POD2[Pod 2<br/>wargames container]
                POD3[Pod 3<br/>wargames container]
            end
            
            subgraph CRONJOB["CronJob: scheduled-assessment"]
                CRON_POD[Scheduler Pod<br/>wargames container]
            end
            
            SVC[Service:<br/>wargames-svc]
            INGRESS[Ingress:<br/>TLS/HTTPS]
            
            subgraph CONFIG["Configuration"]
                CM[ConfigMap:<br/>app-config]
                SECRET[Secret:<br/>api-keys]
            end
            
            subgraph STORAGE_PVC["Persistent Storage"]
                PVC[PVC:<br/>reports-storage]
            end
        end
        
        subgraph SYSTEM["System Services"]
            REDIS_STS[StatefulSet:<br/>redis]
            PG_STS[StatefulSet:<br/>postgresql]
        end
        
        subgraph MONITORING["Monitoring Stack"]
            PROM[Prometheus]
            GRAF[Grafana]
        end
    end
    
    subgraph EXTERNAL_SERVICES["External Services"]
        CLAUDE_EXT[Claude API]
        NEWS_EXT[News APIs]
        S3_EXT[Object Storage]
    end
    
    INGRESS --> SVC
    SVC --> POD1
    SVC --> POD2
    SVC --> POD3
    
    CRON_POD -.triggers.-> POD1
    
    POD1 --> CM
    POD2 --> CM
    POD3 --> CM
    
    POD1 --> SECRET
    POD2 --> SECRET
    POD3 --> SECRET
    
    POD1 --> PVC
    POD2 --> PVC
    POD3 --> PVC
    
    POD1 --> REDIS_STS
    POD2 --> REDIS_STS
    POD3 --> REDIS_STS
    
    POD1 --> PG_STS
    POD2 --> PG_STS
    POD3 --> PG_STS
    
    POD1 --> CLAUDE_EXT
    POD2 --> CLAUDE_EXT
    POD3 --> CLAUDE_EXT
    
    POD1 --> NEWS_EXT
    POD2 --> NEWS_EXT
    POD3 --> NEWS_EXT
    
    POD1 --> S3_EXT
    POD2 --> S3_EXT
    POD3 --> S3_EXT
    
    POD1 --> PROM
    POD2 --> PROM
    POD3 --> PROM
    
    PROM --> GRAF
    
    style INGRESS fill:#4ecdc4
    style POD1 fill:#ff6b6b
    style POD2 fill:#ff6b6b
    style POD3 fill:#ff6b6b
    style CRON_POD fill:#ffe66d
    style REDIS_STS fill:#f9ca24
    style PG_STS fill:#95e1d3
    style CLAUDE_EXT fill:#f38181
```

---

## 10. State Machines

### 10.1 Assessment Lifecycle State Machine

```mermaid
stateDiagram-v2
    [*] --> Idle
    
    Idle --> Scheduled: Timer/Manual Trigger
    Scheduled --> Initializing: Start Assessment
    
    Initializing --> CollectingData: System Ready
    Initializing --> Failed: Initialization Error
    
    CollectingData --> DataValidation: Collection Complete
    CollectingData --> CollectingData: Retry on Failure
    CollectingData --> Failed: Max Retries Exceeded
    
    DataValidation --> AnalyzingRisk: Data Valid
    DataValidation --> CollectingData: Data Insufficient
    
    AnalyzingRisk --> CalculatingRisk: Analysis Complete
    AnalyzingRisk --> AnalyzingRisk: Consensus Building
    AnalyzingRisk --> Failed: Analysis Error
    
    CalculatingRisk --> GeneratingOutputs: Calculation Complete
    CalculatingRisk --> Failed: Calculation Error
    
    GeneratingOutputs --> StoringResults: Outputs Generated
    GeneratingOutputs --> Failed: Generation Error
    
    StoringResults --> CheckingAlerts: Stored Successfully
    StoringResults --> Failed: Storage Error
    
    CheckingAlerts --> SendingNotifications: Alerts Triggered
    CheckingAlerts --> Completed: No Alerts
    
    SendingNotifications --> Completed: Notifications Sent
    SendingNotifications --> Completed: Notification Failed (Non-Critical)
    
    Completed --> Idle: Cleanup Complete
    Failed --> Idle: Error Logged & Reported
    
    note right of CollectingData
        Parallel collection from
        50+ data sources
        with timeout handling
    end note
    
    note right of AnalyzingRisk
        Multiple Claude API calls
        with consensus building
    end note
    
    note right of CalculatingRisk
        Multi-stage risk calculation:
        1. Weighted scoring
        2. Bayesian adjustment
        3. Trend analysis
        4. Monte Carlo simulation
    end note
```

### 10.2 Data Collector State Machine

```mermaid
stateDiagram-v2
    [*] --> Idle
    
    Idle --> CheckingCache: Collection Request
    
    CheckingCache --> ReturningCached: Cache Hit & Fresh
    CheckingCache --> Collecting: Cache Miss/Stale
    
    Collecting --> Fetching: Initialize Collectors
    
    Fetching --> Parsing: Data Retrieved
    Fetching --> Retrying: Temporary Failure
    Fetching --> Failed: Permanent Failure
    
    Retrying --> Fetching: Wait & Retry
    Retrying --> Failed: Max Retries Exceeded
    
    Parsing --> Validating: Parsed Successfully
    Parsing --> Failed: Parse Error
    
    Validating --> Deduplicating: Valid Data
    Validating --> Failed: Invalid Data
    
    Deduplicating --> Scoring: Duplicates Removed
    
    Scoring --> Caching: Quality Check Passed
    Scoring --> Failed: Quality Too Low
    
    Caching --> Storing: Cached
    
    Storing --> Complete: Stored
    Storing --> Failed: Storage Error
    
    Complete --> [*]
    Failed --> [*]
    ReturningCached --> [*]
    
    note right of Retrying
        Exponential backoff:
        2s, 4s, 8s, 16s, 32s
    end note
    
    note right of Scoring
        Quality threshold: 0.5
        Based on source reliability,
        freshness, completeness
    end note
```

---

## 11. Testing Architecture

### 11.1 Test Pyramid

```mermaid
graph TB
    subgraph PYRAMID["Test Pyramid"]
        E2E[End-to-End Tests<br/>Complete Assessment Flow<br/>~10 tests]
        
        INT[Integration Tests<br/>Component Interactions<br/>~50 tests]
        
        UNIT[Unit Tests<br/>Individual Functions<br/>~500+ tests]
    end
    
    subgraph TYPES["Test Types"]
        FUNC[Functional Tests]
        PERF[Performance Tests]
        SEC[Security Tests]
        CHAOS[Chaos Tests]
        PROP[Property Tests]
    end
    
    subgraph COVERAGE["Coverage Goals"]
        LINE[Line Coverage:<br/>95%+]
        BRANCH[Branch Coverage:<br/>90%+]
        FUNC_COV[Function Coverage:<br/>100%]
    end
    
    subgraph CI["CI/CD Pipeline"]
        COMMIT[Commit]
        LINT[Linting<br/>clippy]
        FMT[Formatting<br/>rustfmt]
        BUILD[Build]
        TEST_RUN[Run Tests]
        COV_CHECK[Coverage Check]
        BENCH[Benchmarks]
        DEPLOY[Deploy]
    end
    
    E2E --> INT
    INT --> UNIT
    
    UNIT --> FUNC
    INT --> FUNC
    E2E --> FUNC
    
    UNIT --> PERF
    INT --> PERF
    
    UNIT --> SEC
    INT --> SEC
    
    INT --> CHAOS
    E2E --> CHAOS
    
    UNIT --> PROP
    
    FUNC --> LINE
    FUNC --> BRANCH
    FUNC --> FUNC_COV
    
    COMMIT --> LINT
    LINT --> FMT
    FMT --> BUILD
    BUILD --> TEST_RUN
    TEST_RUN --> COV_CHECK
    COV_CHECK --> BENCH
    BENCH --> DEPLOY
    
    style E2E fill:#ff6b6b
    style INT fill:#ffe66d
    style UNIT fill:#4ecdc4
    style LINE fill:#5fb96b
    style BRANCH fill:#5fb96b
    style FUNC_COV fill:#5fb96b
```

### 11.2 Test Data Flow

```mermaid
flowchart LR
    subgraph FIXTURES["Test Fixtures"]
        MOCK_NEWS[Mock News Data]
        MOCK_SIPRI[Mock SIPRI Data]
        MOCK_CLAUDE[Mock Claude Responses]
        MOCK_DB[Mock Database]
    end
    
    subgraph GENERATORS["Data Generators"]
        GEN_ARTICLES[Article Generator]
        GEN_ARSENALS[Arsenal Generator]
        GEN_EVENTS[Event Generator]
        GEN_SCORES[Score Generator]
    end
    
    subgraph TESTS["Test Suites"]
        UNIT_TESTS[Unit Tests]
        INT_TESTS[Integration Tests]
        E2E_TESTS[E2E Tests]
    end
    
    subgraph ASSERTIONS["Assertions"]
        ASSERT_DATA[Data Quality]
        ASSERT_CALC[Calculations Correct]
        ASSERT_OUTPUT[Output Format]
        ASSERT_PERF[Performance Metrics]
    end
    
    subgraph REPORTS["Test Reports"]
        COV_REPORT[Coverage Report]
        PERF_REPORT[Performance Report]
        FAIL_REPORT[Failure Report]
    end
    
    MOCK_NEWS --> UNIT_TESTS
    MOCK_SIPRI --> UNIT_TESTS
    MOCK_CLAUDE --> UNIT_TESTS
    MOCK_DB --> UNIT_TESTS
    
    GEN_ARTICLES --> INT_TESTS
    GEN_ARSENALS --> INT_TESTS
    GEN_EVENTS --> INT_TESTS
    GEN_SCORES --> INT_TESTS
    
    MOCK_NEWS --> E2E_TESTS
    MOCK_SIPRI --> E2E_TESTS
    MOCK_CLAUDE --> E2E_TESTS
    MOCK_DB --> E2E_TESTS
    
    UNIT_TESTS --> ASSERT_DATA
    UNIT_TESTS --> ASSERT_CALC
    
    INT_TESTS --> ASSERT_DATA
    INT_TESTS --> ASSERT_CALC
    INT_TESTS --> ASSERT_OUTPUT
    
    E2E_TESTS --> ASSERT_OUTPUT
    E2E_TESTS --> ASSERT_PERF
    
    ASSERT_DATA --> COV_REPORT
    ASSERT_CALC --> COV_REPORT
    ASSERT_OUTPUT --> COV_REPORT
    ASSERT_PERF --> PERF_REPORT
    
    ASSERT_DATA -.failures.-> FAIL_REPORT
    ASSERT_CALC -.failures.-> FAIL_REPORT
    ASSERT_OUTPUT -.failures.-> FAIL_REPORT
    ASSERT_PERF -.failures.-> FAIL_REPORT
    
    style FIXTURES fill:#4ecdc4
    style GENERATORS fill:#ffe66d
    style TESTS fill:#ff6b6b
    style ASSERTIONS fill:#f9ca24
    style REPORTS fill:#95e1d3
```

---

## 12. Security Architecture

### 12.1 Security Layers

```mermaid
graph TB
    subgraph EDGE["Edge Security"]
        TLS[TLS 1.3<br/>Encryption]
        WAF[Web Application<br/>Firewall]
        DDOS[DDoS<br/>Protection]
        RATELIMIT[Rate Limiting]
    end
    
    subgraph AUTH["Authentication & Authorization"]
        API_KEY[API Key<br/>Management]
        TOKEN[Token<br/>Validation]
        RBAC[Role-Based<br/>Access Control]
    end
    
    subgraph APP["Application Security"]
        INPUT_VAL[Input<br/>Validation]
        OUTPUT_ENC[Output<br/>Encoding]
        CSRF[CSRF<br/>Protection]
        XSS[XSS<br/>Prevention]
    end
    
    subgraph DATA["Data Security"]
        ENCRYPT_REST[Encryption<br/>at Rest]
        ENCRYPT_TRANSIT[Encryption<br/>in Transit]
        KEY_MGMT[Key Management<br/>AES-256-GCM]
        SECRETS[Secrets<br/>Management]
    end
    
    subgraph AUDIT["Audit & Monitoring"]
        ACCESS_LOG[Access<br/>Logging]
        AUDIT_TRAIL[Audit<br/>Trail]
        ANOMALY[Anomaly<br/>Detection]
        SIEM[SIEM<br/>Integration]
    end
    
    subgraph COMPLIANCE["Compliance"]
        GDPR[GDPR<br/>Compliance]
        SOC2[SOC 2<br/>Controls]
        BACKUP[Backup &<br/>Recovery]
        RETENTION[Data<br/>Retention]
    end
    
    TLS --> AUTH
    WAF --> AUTH
    DDOS --> AUTH
    RATELIMIT --> AUTH
    
    AUTH --> APP
    
    APP --> DATA
    
    DATA --> AUDIT
    
    AUDIT --> COMPLIANCE
    
    API_KEY -.encrypts.-> KEY_MGMT
    TOKEN -.validates.-> RBAC
    
    INPUT_VAL -.sanitizes.-> DATA
    
    ENCRYPT_REST --> KEY_MGMT
    ENCRYPT_TRANSIT --> KEY_MGMT
    
    ACCESS_LOG --> SIEM
    AUDIT_TRAIL --> SIEM
    ANOMALY --> SIEM
    
    style TLS fill:#4ecdc4
    style WAF fill:#4ecdc4
    style API_KEY fill:#ffe66d
    style RBAC fill:#ffe66d
    style INPUT_VAL fill:#f9ca24
    style ENCRYPT_REST fill:#f38181
    style ENCRYPT_TRANSIT fill:#f38181
    style KEY_MGMT fill:#f38181
    style AUDIT_TRAIL fill:#aa96da
```

### 12.2 API Key Encryption Flow

```mermaid
sequenceDiagram
    participant USER as User/Config
    participant APP as Application
    participant CRYPTO as Crypto Utils
    participant KEYSTORE as Key Store
    participant EXTERNAL as External API
    
    Note over USER,KEYSTORE: Initial Setup
    USER->>APP: Provide API Key
    APP->>CRYPTO: encrypt_api_key(key)
    activate CRYPTO
    CRYPTO->>CRYPTO: Generate random salt
    CRYPTO->>CRYPTO: Derive encryption key<br/>from master password
    CRYPTO->>CRYPTO: Encrypt with AES-256-GCM
    CRYPTO-->>APP: encrypted_key
    deactivate CRYPTO
    APP->>KEYSTORE: store(encrypted_key, salt)
    
    Note over APP,EXTERNAL: API Usage
    APP->>KEYSTORE: retrieve(key_id)
    KEYSTORE-->>APP: encrypted_key, salt
    APP->>CRYPTO: decrypt_api_key(encrypted_key, salt)
    activate CRYPTO
    CRYPTO->>CRYPTO: Derive decryption key
    CRYPTO->>CRYPTO: Decrypt
    CRYPTO-->>APP: plaintext_key
    deactivate CRYPTO
    
    APP->>EXTERNAL: API Request with key
    EXTERNAL-->>APP: API Response
    
    Note over APP: Key never stored in plaintext<br/>Key in memory only during use
    APP->>APP: Zero out plaintext_key
```

---

## 13. Scheduling and Automation

### 13.1 Scheduler Architecture

```mermaid
graph TB
    subgraph SCHEDULER["Task Scheduling Engine"]
        CRON[Cron Parser]
        QUEUE[Task Queue]
        EXECUTOR[Task Executor]
        MONITOR[Job Monitor]
    end
    
    subgraph JOBS["Scheduled Jobs"]
        ASSESS_MONTHLY[Monthly Assessment<br/>1st of month, 00:00 UTC]
        ASSESS_WEEKLY[Weekly Check<br/>Monday, 06:00 UTC]
        NEWS_HOURLY[News Collection<br/>Every hour]
        RESEARCH_DAILY[Research Update<br/>Daily, 06:00 UTC]
        CLEANUP[Database Cleanup<br/>Weekly, Sunday 02:00 UTC]
        BACKUP[Backup Task<br/>Daily, 03:00 UTC]
    end
    
    subgraph EXECUTION["Execution Flow"]
        CHECK_TIME[Check Schedule]
        CREATE_TASK[Create Task Instance]
        ENQUEUE[Enqueue Task]
        EXEC_TASK[Execute Task]
        CHECK_STATUS[Check Status]
        HANDLE_RESULT[Handle Result]
    end
    
    subgraph RECOVERY["Error Recovery"]
        RETRY[Retry Logic]
        FALLBACK[Fallback Strategy]
        ALERT_ADMIN[Alert Admin]
        LOG_ERROR[Log Error]
    end
    
    CRON --> CHECK_TIME
    CHECK_TIME --> CREATE_TASK
    CREATE_TASK --> ENQUEUE
    
    ENQUEUE --> QUEUE
    QUEUE --> EXECUTOR
    
    EXECUTOR --> EXEC_TASK
    EXEC_TASK --> CHECK_STATUS
    CHECK_STATUS --> HANDLE_RESULT
    
    ASSESS_MONTHLY --> CRON
    ASSESS_WEEKLY --> CRON
    NEWS_HOURLY --> CRON
    RESEARCH_DAILY --> CRON
    CLEANUP --> CRON
    BACKUP --> CRON
    
    HANDLE_RESULT -.failure.-> RETRY
    RETRY -.max retries.-> FALLBACK
    FALLBACK --> ALERT_ADMIN
    RETRY --> LOG_ERROR
    
    MONITOR --> CHECK_STATUS
    MONITOR --> LOG_ERROR
    
    style CRON fill:#4ecdc4
    style QUEUE fill:#ffe66d
    style EXECUTOR fill:#ff6b6b
    style ASSESS_MONTHLY fill:#f9ca24
    style RETRY fill:#f38181
```

### 13.2 Cron Expression Examples

```mermaid
graph LR
    subgraph CRON_PATTERNS["Cron Schedule Patterns"]
        MONTHLY["0 0 1 * *<br/>Monthly: 1st day, midnight"]
        WEEKLY["0 6 * * 1<br/>Weekly: Monday 6AM"]
        DAILY["0 6 * * *<br/>Daily: 6AM UTC"]
        HOURLY["0 * * * *<br/>Hourly: Top of hour"]
        CUSTOM1["0 */6 * * *<br/>Every 6 hours"]
        CUSTOM2["0 2 * * 0<br/>Weekly: Sunday 2AM"]
    end
    
    subgraph TASKS["Associated Tasks"]
        T1[Full Assessment]
        T2[Trend Analysis]
        T3[Data Collection]
        T4[News Monitoring]
        T5[Research Updates]
        T6[Database Cleanup]
    end
    
    MONTHLY --> T1
    WEEKLY --> T2
    DAILY --> T3
    HOURLY --> T4
    CUSTOM1 --> T5
    CUSTOM2 --> T6
    
    style MONTHLY fill:#ff6b6b
    style WEEKLY fill:#4ecdc4
    style DAILY fill:#ffe66d
    style HOURLY fill:#f9ca24
    style T1 fill:#f38181
```

---

## 14. Error Handling Flow

### 14.1 Error Handling Strategy

```mermaid
flowchart TD
    ERROR([Error Occurs])
    
    CLASSIFY{Classify Error Type}
    
    TRANSIENT[Transient Error<br/>Network, Timeout, Rate Limit]
    PERMANENT[Permanent Error<br/>Invalid Data, Auth Failure]
    CRITICAL[Critical Error<br/>System Failure, Data Loss]
    
    RETRY_CHECK{Retry Count < Max?}
    BACKOFF[Exponential Backoff<br/>Wait & Retry]
    
    FALLBACK{Fallback Available?}
    USE_FALLBACK[Use Fallback Strategy<br/>Cached Data, Default Values]
    
    LOG[Log Error Details<br/>Stack Trace, Context]
    METRIC[Update Error Metrics<br/>Prometheus Counter]
    
    ALERT{Alert Threshold?}
    SEND_ALERT[Send Alert to Admins<br/>Slack/Email/PagerDuty]
    
    RECOVER{Can Recover?}
    GRACEFUL[Graceful Degradation<br/>Partial Functionality]
    FAIL[Fail Operation<br/>Return Error to User]
    
    STORE[Store Error in Database<br/>For Analysis]
    
    END([Complete Error Handling])
    
    ERROR --> CLASSIFY
    
    CLASSIFY --> TRANSIENT
    CLASSIFY --> PERMANENT
    CLASSIFY --> CRITICAL
    
    TRANSIENT --> RETRY_CHECK
    RETRY_CHECK -->|Yes| BACKOFF
    RETRY_CHECK -->|No| FALLBACK
    BACKOFF --> RETRY_CHECK
    
    PERMANENT --> FALLBACK
    
    CRITICAL --> LOG
    CRITICAL --> METRIC
    CRITICAL --> SEND_ALERT
    
    FALLBACK -->|Yes| USE_FALLBACK
    FALLBACK -->|No| LOG
    
    USE_FALLBACK --> LOG
    
    LOG --> METRIC
    METRIC --> ALERT
    
    ALERT -->|Yes| SEND_ALERT
    ALERT -->|No| RECOVER
    
    SEND_ALERT --> RECOVER
    
    RECOVER -->|Yes| GRACEFUL
    RECOVER -->|No| FAIL
    
    GRACEFUL --> STORE
    FAIL --> STORE
    
    STORE --> END
    
    style ERROR fill:#ff6b6b
    style TRANSIENT fill:#ffe66d
    style PERMANENT fill:#f9ca24
    style CRITICAL fill:#f38181
    style USE_FALLBACK fill:#4ecdc4
    style GRACEFUL fill:#5fb96b
    style FAIL fill:#ff6b6b
    style END fill:#95e1d3
```

### 14.2 Error Types Taxonomy

```mermaid
graph TB
    ROOT[WarGamesError]
    
    subgraph DATA_ERRORS["Data Collection Errors"]
        NETWORK[NetworkError]
        TIMEOUT[TimeoutError]
        PARSE[ParseError]
        SOURCE[SourceUnavailableError]
        QUALITY[DataQualityError]
    end
    
    subgraph API_ERRORS["API Errors"]
        AUTH[AuthenticationError]
        RATE[RateLimitError]
        QUOTA[QuotaExceededError]
        API_FAIL[APIFailureError]
    end
    
    subgraph CALC_ERRORS["Calculation Errors"]
        INVALID[InvalidInputError]
        OVERFLOW[OverflowError]
        CONVERGENCE[ConvergenceError]
        INSUFFICIENT[InsufficientDataError]
    end
    
    subgraph STORAGE_ERRORS["Storage Errors"]
        DB[DatabaseError]
        CACHE[CacheError]
        FILE[FileSystemError]
    end
    
    subgraph SYSTEM_ERRORS["System Errors"]
        CONFIG[ConfigurationError]
        RESOURCE[ResourceExhaustedError]
        DEPENDENCY[DependencyError]
    end
    
    ROOT --> NETWORK
    ROOT --> TIMEOUT
    ROOT --> PARSE
    ROOT --> SOURCE
    ROOT --> QUALITY
    
    ROOT --> AUTH
    ROOT --> RATE
    ROOT --> QUOTA
    ROOT --> API_FAIL
    
    ROOT --> INVALID
    ROOT --> OVERFLOW
    ROOT --> CONVERGENCE
    ROOT --> INSUFFICIENT
    
    ROOT --> DB
    ROOT --> CACHE
    ROOT --> FILE
    
    ROOT --> CONFIG
    ROOT --> RESOURCE
    ROOT --> DEPENDENCY
    
    style ROOT fill:#ff6b6b
    style DATA_ERRORS fill:#4ecdc4
    style API_ERRORS fill:#ffe66d
    style CALC_ERRORS fill:#f9ca24
    style STORAGE_ERRORS fill:#f38181
    style SYSTEM_ERRORS fill:#aa96da
```

---

## 15. Notification System

### 15.1 Alert Flow

```mermaid
flowchart TD
    START([Risk Score Calculated])
    
    CHECK[Check Alert Conditions]
    
    THRESHOLD{Meets Threshold?}
    
    CRITICAL{Critical<br/>< 100 seconds?}
    SEVERE{Severe<br/>< 200 seconds?}
    HIGH{High<br/>< 400 seconds?}
    TREND{Significant<br/>Trend Change?}
    
    PREPARE[Prepare Alert Message]
    
    FORMAT[Format Message:<br/>- Risk Score<br/>- Trend Direction<br/>- Primary Drivers<br/>- Recommendations]
    
    CHANNELS{Active Channels?}
    
    EMAIL[Send Email<br/>SMTP]
    SLACK[Send Slack<br/>Webhook]
    WEBHOOK[Send Custom<br/>Webhook]
    SMS[Send SMS<br/>Twilio]
    
    LOG[Log Alert Sent]
    STORE[Store in Database]
    
    THROTTLE{Rate Limited?}
    
    WAIT[Wait Cooldown Period]
    
    END([Complete])
    
    START --> CHECK
    CHECK --> THRESHOLD
    
    THRESHOLD -->|Yes| CRITICAL
    THRESHOLD -->|No| END
    
    CRITICAL -->|Yes| PREPARE
    CRITICAL -->|No| SEVERE
    
    SEVERE -->|Yes| PREPARE
    SEVERE -->|No| HIGH
    
    HIGH -->|Yes| PREPARE
    HIGH -->|No| TREND
    
    TREND -->|Yes| PREPARE
    TREND -->|No| END
    
    PREPARE --> FORMAT
    FORMAT --> THROTTLE
    
    THROTTLE -->|Yes| WAIT
    THROTTLE -->|No| CHANNELS
    
    WAIT --> END
    
    CHANNELS --> EMAIL
    CHANNELS --> SLACK
    CHANNELS --> WEBHOOK
    CHANNELS --> SMS
    
    EMAIL --> LOG
    SLACK --> LOG
    WEBHOOK --> LOG
    SMS --> LOG
    
    LOG --> STORE
    STORE --> END
    
    style START fill:#95e1d3
    style CRITICAL fill:#ff6b6b
    style SEVERE fill:#ff6b6b
    style HIGH fill:#ffe66d
    style TREND fill:#ffe66d
    style EMAIL fill:#4ecdc4
    style SLACK fill:#4ecdc4
    style WEBHOOK fill:#4ecdc4
    style END fill:#95e1d3
```

### 15.2 Alert Escalation

```mermaid
stateDiagram-v2
    [*] --> Monitoring
    
    Monitoring --> LowRisk: Score > 600s
    Monitoring --> ModerateRisk: 400s < Score ≤ 600s
    Monitoring --> HighRisk: 200s < Score ≤ 400s
    Monitoring --> SevereRisk: 100s < Score ≤ 200s
    Monitoring --> CriticalRisk: Score ≤ 100s
    
    LowRisk --> Monitoring: Score Improves
    LowRisk --> ModerateRisk: Score Deteriorates
    
    ModerateRisk --> LowRisk: Score Improves
    ModerateRisk --> HighRisk: Score Deteriorates
    ModerateRisk --> NotifyTeam: Alert Threshold
    
    HighRisk --> ModerateRisk: Score Improves
    HighRisk --> SevereRisk: Score Deteriorates
    HighRisk --> NotifyLeadership: Escalation Level 1
    
    SevereRisk --> HighRisk: Score Improves
    SevereRisk --> CriticalRisk: Score Deteriorates
    SevereRisk --> NotifyExecutives: Escalation Level 2
    
    CriticalRisk --> SevereRisk: Score Improves
    CriticalRisk --> EmergencyProtocol: Immediate Action Required
    
    NotifyTeam --> ModerateRisk: Acknowledged
    NotifyLeadership --> HighRisk: Acknowledged
    NotifyExecutives --> SevereRisk: Acknowledged
    EmergencyProtocol --> CriticalRisk: Initiated
    
    note right of ModerateRisk
        Send to: Development Team
        Method: Slack, Email
        Frequency: Once per change
    end note
    
    note right of HighRisk
        Send to: Team Leads, Security
        Method: Slack, Email, Dashboard
        Frequency: Every assessment
    end note
    
    note right of SevereRisk
        Send to: Executives, Security Team
        Method: All channels + SMS
        Frequency: Continuous monitoring
    end note
    
    note right of CriticalRisk
        Send to: All stakeholders
        Method: ALL CHANNELS
        Frequency: Real-time alerts
        Action: Emergency protocols
    end note
```

---

## Conclusion

This comprehensive diagram collection provides visual documentation of every major aspect of the WarGames/JOSHUA nuclear risk assessment system. These diagrams serve as:

1. **Architecture Reference**: Complete system design and component relationships
2. **Implementation Guide**: Detailed flows for developers implementing features
3. **Operations Manual**: Deployment, monitoring, and error handling procedures
4. **Communication Tool**: Explaining system design to stakeholders
5. **Onboarding Resource**: Helping new team members understand the system

### Usage Guidelines

- **Development**: Reference module hierarchy and data flow diagrams during implementation
- **Deployment**: Use deployment architecture diagrams for infrastructure setup
- **Troubleshooting**: Consult error handling and state machine diagrams
- **Optimization**: Review component interaction and performance-critical paths
- **Security**: Reference security architecture for compliance and audits

### Diagram Maintenance

These diagrams should be updated whenever:
- Major architectural changes occur
- New components are added
- Integration patterns change
- Deployment strategy evolves
- Security controls are modified

*"The only winning move is not to play — but if we must monitor the game, we visualize every aspect with precision and clarity."*

---

**Document Version**: 1.0.0  
**Last Updated**: October 2025  
**Maintained By**: WarGames/JOSHUA Development Team  
**Review Frequency**: Quarterly or after major changes
