# WarGames/JOSHUA: Database Design & Schema Specification
## Complete Data Model for Nuclear Risk Assessment System
### Version 1.0.0 | October 2025

---

## Executive Summary

This document specifies the complete database architecture for the WarGames/JOSHUA system. The database serves as the system's persistent memory, storing historical assessments, risk factors, collected data, and analysis results. The design prioritizes data integrity, query performance, and historical analysis capabilities.

### Design Principles

1. **Normalization**: Third normal form (3NF) for data integrity
2. **Performance**: Strategic denormalization where justified
3. **History**: Complete audit trail of all assessments
4. **Flexibility**: Schema supports evolving risk factors
5. **Scalability**: Designed for years of historical data
6. **Integrity**: Foreign keys and constraints enforce relationships

---

## 1. Database Technology Selection

### 1.1 PostgreSQL as Primary Database

**Rationale:**
- JSONB support for flexible schema evolution
- Excellent time-series data handling
- Full-text search capabilities
- PostGIS for geospatial data
- Robust ACID guarantees
- Strong indexing options

```toml
[database]
type = "postgresql"
version = "14+"
connection_string = "postgresql://user:pass@localhost:5432/wargames"
pool_size = 10
max_lifetime = "30m"
idle_timeout = "10m"
```

### 1.2 Complementary Technologies

```rust
pub struct DatabaseStack {
    /// Primary persistent storage
    pub postgres: sqlx::PgPool,
    
    /// Fast caching layer
    pub redis: redis::Client,
    
    /// Time-series optimization (optional)
    pub timescaledb: Option<TimescaleDB>,
    
    /// Full-text search (optional)
    pub elasticsearch: Option<ElasticsearchClient>,
}
```

---

## 2. Core Schema Design

### 2.1 Assessments Table (Primary Entity)

```sql
-- Primary table storing nuclear risk assessments
CREATE TABLE assessments (
    -- Identity
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_date TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Risk Metrics
    seconds_to_midnight INTEGER NOT NULL CHECK (seconds_to_midnight >= 0 AND seconds_to_midnight <= 1440),
    raw_risk_score DECIMAL(5,4) NOT NULL CHECK (raw_risk_score >= 0.0 AND raw_risk_score <= 1.0),
    base_risk_score DECIMAL(5,4) NOT NULL,
    bayesian_adjusted_score DECIMAL(5,4) NOT NULL,
    
    -- Confidence Metrics
    overall_confidence VARCHAR(20) NOT NULL CHECK (overall_confidence IN ('VeryLow', 'Low', 'Moderate', 'High', 'VeryHigh')),
    data_quality_score DECIMAL(3,2) CHECK (data_quality_score >= 0.0 AND data_quality_score <= 1.0),
    
    -- Trend Analysis
    trend_direction VARCHAR(20) CHECK (trend_direction IN ('Increasing', 'Decreasing', 'Stable', 'Uncertain')),
    trend_magnitude DECIMAL(5,4),
    delta_from_previous INTEGER,
    
    -- Analysis Content
    executive_summary TEXT NOT NULL,
    detailed_analysis TEXT,
    recommendations TEXT[],
    
    -- Metadata
    claude_model_version VARCHAR(50) NOT NULL,
    data_sources_count INTEGER NOT NULL,
    collection_duration_seconds INTEGER,
    analysis_duration_seconds INTEGER,
    
    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100),
    
    -- Indexes
    CONSTRAINT assessments_date_idx UNIQUE (assessment_date)
);

-- Indexes for efficient querying
CREATE INDEX idx_assessments_seconds ON assessments(seconds_to_midnight);
CREATE INDEX idx_assessments_date_desc ON assessments(assessment_date DESC);
CREATE INDEX idx_assessments_trend ON assessments(trend_direction);
CREATE INDEX idx_assessments_created ON assessments(created_at);

-- Trigger for updated_at
CREATE TRIGGER update_assessments_updated_at
    BEFORE UPDATE ON assessments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### 2.2 Risk Factors Table

```sql
-- Individual risk factors contributing to each assessment
CREATE TABLE risk_factors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    
    -- Factor Identity
    factor_category VARCHAR(50) NOT NULL,
    factor_name VARCHAR(200) NOT NULL,
    factor_type VARCHAR(100),
    
    -- Factor Values
    raw_value DECIMAL(5,4) NOT NULL CHECK (raw_value >= 0.0 AND raw_value <= 1.0),
    weighted_value DECIMAL(5,4) NOT NULL,
    category_weight DECIMAL(4,3) NOT NULL,
    
    -- Contribution Analysis
    contribution_to_risk DECIMAL(5,4) NOT NULL,
    rank_in_category INTEGER,
    rank_overall INTEGER,
    
    -- Confidence
    confidence_level VARCHAR(20) NOT NULL,
    confidence_score DECIMAL(3,2),
    
    -- Supporting Data
    data_sources TEXT[],
    evidence_summary TEXT,
    contrary_evidence TEXT,
    
    -- Context
    historical_context TEXT,
    comparison_to_baseline DECIMAL(5,4),
    
    -- Timestamps
    observed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_risk_factors_assessment ON risk_factors(assessment_id);
CREATE INDEX idx_risk_factors_category ON risk_factors(factor_category);
CREATE INDEX idx_risk_factors_contribution ON risk_factors(contribution_to_risk DESC);
CREATE INDEX idx_risk_factors_name ON risk_factors(factor_name);

-- Composite index for efficient category queries
CREATE INDEX idx_risk_factors_assessment_category ON risk_factors(assessment_id, factor_category);
```

### 2.3 Nuclear Arsenals Tracking

```sql
-- Track nuclear arsenal changes over time
CREATE TABLE nuclear_arsenals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID REFERENCES assessments(id) ON DELETE SET NULL,
    
    -- Country Identification
    country VARCHAR(50) NOT NULL,
    country_code CHAR(2) NOT NULL,
    
    -- Warhead Counts
    total_warheads INTEGER NOT NULL CHECK (total_warheads >= 0),
    deployed_strategic INTEGER NOT NULL CHECK (deployed_strategic >= 0),
    deployed_tactical INTEGER CHECK (deployed_tactical >= 0),
    reserve_warheads INTEGER CHECK (reserve_warheads >= 0),
    retired_awaiting_dismantlement INTEGER CHECK (retired_awaiting_dismantlement >= 0),
    
    -- Changes from Previous
    change_total INTEGER,
    change_deployed_strategic INTEGER,
    change_deployed_tactical INTEGER,
    
    -- Delivery Systems
    icbm_count INTEGER,
    slbm_count INTEGER,
    bomber_count INTEGER,
    tactical_delivery_systems INTEGER,
    
    -- Modernization Programs
    modernization_programs JSONB,
    new_systems_deployed TEXT[],
    systems_retired TEXT[],
    
    -- Alert Status
    alert_status VARCHAR(50),
    deployed_on_alert INTEGER,
    
    -- Data Quality
    data_source VARCHAR(200) NOT NULL,
    source_reliability DECIMAL(3,2),
    uncertainty_range JSONB,  -- {"lower": X, "upper": Y}
    
    -- Timestamps
    observation_date DATE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT nuclear_arsenals_country_date UNIQUE (country, observation_date)
);

CREATE INDEX idx_arsenals_country ON nuclear_arsenals(country);
CREATE INDEX idx_arsenals_date ON nuclear_arsenals(observation_date DESC);
CREATE INDEX idx_arsenals_assessment ON nuclear_arsenals(assessment_id);
CREATE INDEX idx_arsenals_country_date ON nuclear_arsenals(country, observation_date DESC);
```

### 2.4 Regional Conflicts & Hotspots

```sql
-- Track active conflicts and regional tensions
CREATE TABLE regional_conflicts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID REFERENCES assessments(id) ON DELETE SET NULL,
    
    -- Conflict Identification
    conflict_name VARCHAR(200) NOT NULL,
    region VARCHAR(100) NOT NULL,
    sub_region VARCHAR(100),
    
    -- Parties Involved
    parties_involved TEXT[] NOT NULL,
    nuclear_powers_involved TEXT[],
    nato_involvement BOOLEAN DEFAULT FALSE,
    
    -- Risk Metrics
    escalation_level INTEGER CHECK (escalation_level BETWEEN 0 AND 10),
    nuclear_threat_level DECIMAL(3,2) CHECK (nuclear_threat_level >= 0.0 AND nuclear_threat_level <= 1.0),
    probability_of_escalation DECIMAL(3,2),
    
    -- Conflict Details
    conflict_type VARCHAR(50),  -- 'Active Military', 'Diplomatic Crisis', 'Frozen Conflict', etc.
    start_date DATE,
    intensity_level VARCHAR(20),
    casualty_count_estimate INTEGER,
    
    -- Nuclear Dimensions
    nuclear_facilities_at_risk TEXT[],
    tactical_nuclear_deployment BOOLEAN DEFAULT FALSE,
    nuclear_threats_issued BOOLEAN DEFAULT FALSE,
    
    -- Geographic Data
    latitude DECIMAL(10, 7),
    longitude DECIMAL(10, 7),
    affected_area_km2 DECIMAL(12, 2),
    
    -- Status
    status VARCHAR(50) NOT NULL,  -- 'Active', 'Frozen', 'Resolved', 'Escalating'
    last_incident_date DATE,
    
    -- Analysis
    key_developments TEXT[],
    escalation_pathway TEXT,
    de_escalation_factors TEXT[],
    
    -- Timestamps
    observation_date DATE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_conflicts_region ON regional_conflicts(region);
CREATE INDEX idx_conflicts_status ON regional_conflicts(status);
CREATE INDEX idx_conflicts_escalation ON regional_conflicts(escalation_level DESC);
CREATE INDEX idx_conflicts_nuclear_powers ON regional_conflicts USING GIN (nuclear_powers_involved);
CREATE INDEX idx_conflicts_assessment ON regional_conflicts(assessment_id);
```

### 2.5 Historical Events Database

```sql
-- Catalog of significant historical nuclear-related events
CREATE TABLE nuclear_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Event Identification
    event_name VARCHAR(300) NOT NULL,
    event_date DATE NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    
    -- Classification
    severity_score DECIMAL(3,2) NOT NULL CHECK (severity_score >= 0.0 AND severity_score <= 1.0),
    event_category VARCHAR(50),  -- 'Crisis', 'Test', 'Incident', 'Policy Change', etc.
    
    -- Parties Involved
    countries_involved TEXT[] NOT NULL,
    nuclear_powers_involved TEXT[],
    
    -- Event Details
    description TEXT NOT NULL,
    context TEXT,
    outcome TEXT,
    lessons_learned TEXT,
    
    -- Impact Assessment
    immediate_risk_impact DECIMAL(3,2),
    long_term_risk_impact DECIMAL(3,2),
    casualties INTEGER,
    economic_impact_usd BIGINT,
    
    -- Resolution
    resolution_method VARCHAR(100),
    time_to_resolution_days INTEGER,
    escalation_avoided BOOLEAN,
    
    -- Sources
    primary_sources TEXT[],
    scholarly_references TEXT[],
    declassified_documents TEXT[],
    
    -- Geographic
    location_latitude DECIMAL(10, 7),
    location_longitude DECIMAL(10, 7),
    affected_regions TEXT[],
    
    -- Metadata
    tags TEXT[],
    related_events UUID[],
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_date ON nuclear_events(event_date DESC);
CREATE INDEX idx_events_type ON nuclear_events(event_type);
CREATE INDEX idx_events_severity ON nuclear_events(severity_score DESC);
CREATE INDEX idx_events_countries ON nuclear_events USING GIN (countries_involved);
CREATE INDEX idx_events_tags ON nuclear_events USING GIN (tags);
```

### 2.6 Collected Data Points

```sql
-- Raw data collected from various sources
CREATE TABLE collected_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Source Information
    source_id UUID NOT NULL REFERENCES data_sources(id),
    source_type VARCHAR(50) NOT NULL,
    
    -- Data Classification
    data_category VARCHAR(50) NOT NULL,
    data_type VARCHAR(100),
    keywords TEXT[],
    
    -- Content
    title VARCHAR(500),
    content TEXT NOT NULL,
    summary TEXT,
    
    -- Extracted Entities
    entities JSONB,  -- {"countries": [], "people": [], "organizations": []}
    named_entities TEXT[],
    
    -- Quality Metrics
    quality_score DECIMAL(3,2) CHECK (quality_score >= 0.0 AND quality_score <= 1.0),
    reliability_score DECIMAL(3,2),
    relevance_score DECIMAL(3,2),
    
    -- Verification
    verified BOOLEAN DEFAULT FALSE,
    verification_method VARCHAR(100),
    corroborating_sources INTEGER DEFAULT 0,
    
    -- Sentiment Analysis
    sentiment_score DECIMAL(4,3),  -- -1.0 to 1.0
    sentiment_label VARCHAR(20),
    
    -- URLs and References
    source_url TEXT,
    archive_url TEXT,
    related_urls TEXT[],
    
    -- Metadata
    metadata JSONB,
    
    -- Timestamps
    published_at TIMESTAMP WITH TIME ZONE,
    collected_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Processing Status
    processed BOOLEAN DEFAULT FALSE,
    used_in_assessment UUID REFERENCES assessments(id)
);

CREATE INDEX idx_collected_data_source ON collected_data(source_id);
CREATE INDEX idx_collected_data_category ON collected_data(data_category);
CREATE INDEX idx_collected_data_collected ON collected_data(collected_at DESC);
CREATE INDEX idx_collected_data_published ON collected_data(published_at DESC);
CREATE INDEX idx_collected_data_quality ON collected_data(quality_score DESC);
CREATE INDEX idx_collected_data_keywords ON collected_data USING GIN (keywords);

-- Full-text search index
CREATE INDEX idx_collected_data_content_fts ON collected_data USING GIN (to_tsvector('english', content));
```

### 2.7 Data Sources Registry

```sql
-- Registry of all data sources
CREATE TABLE data_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Source Identity
    source_name VARCHAR(200) NOT NULL UNIQUE,
    source_type VARCHAR(50) NOT NULL,  -- 'RSS', 'API', 'Scraper', 'Database', etc.
    source_url TEXT,
    
    -- Classification
    category VARCHAR(100) NOT NULL,
    subcategory VARCHAR(100),
    
    -- Reliability Metrics
    base_reliability_score DECIMAL(3,2) NOT NULL DEFAULT 0.5,
    current_reliability_score DECIMAL(3,2) NOT NULL DEFAULT 0.5,
    editorial_independence DECIMAL(3,2),
    
    -- Track Record
    total_data_points_collected INTEGER DEFAULT 0,
    verified_accurate INTEGER DEFAULT 0,
    verified_inaccurate INTEGER DEFAULT 0,
    accuracy_rate DECIMAL(3,2) GENERATED ALWAYS AS (
        CASE 
            WHEN (verified_accurate + verified_inaccurate) > 0 
            THEN verified_accurate::DECIMAL / (verified_accurate + verified_inaccurate)
            ELSE 0.5
        END
    ) STORED,
    
    -- Collection Configuration
    collection_frequency VARCHAR(50),  -- 'Real-time', 'Hourly', 'Daily', etc.
    rate_limit_per_hour INTEGER,
    timeout_seconds INTEGER DEFAULT 30,
    
    -- Authentication
    requires_auth BOOLEAN DEFAULT FALSE,
    auth_type VARCHAR(50),
    
    -- Status
    active BOOLEAN DEFAULT TRUE,
    last_successful_collection TIMESTAMP WITH TIME ZONE,
    last_failed_collection TIMESTAMP WITH TIME ZONE,
    consecutive_failures INTEGER DEFAULT 0,
    
    -- Geographic Coverage
    geographic_coverage TEXT[],
    languages TEXT[],
    
    -- Metadata
    notes TEXT,
    tags TEXT[],
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sources_type ON data_sources(source_type);
CREATE INDEX idx_sources_category ON data_sources(category);
CREATE INDEX idx_sources_reliability ON data_sources(current_reliability_score DESC);
CREATE INDEX idx_sources_active ON data_sources(active);
```

### 2.8 Alerts and Notifications

```sql
-- System alerts and notifications
CREATE TABLE alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID REFERENCES assessments(id) ON DELETE CASCADE,
    
    -- Alert Classification
    alert_level VARCHAR(20) NOT NULL CHECK (alert_level IN ('Info', 'Warning', 'Severe', 'Critical', 'Apocalyptic')),
    alert_type VARCHAR(50) NOT NULL,
    alert_category VARCHAR(100),
    
    -- Alert Content
    title VARCHAR(300) NOT NULL,
    message TEXT NOT NULL,
    detailed_explanation TEXT,
    
    -- Risk Context
    associated_risk_factors TEXT[],
    seconds_to_midnight_at_alert INTEGER,
    trigger_threshold DECIMAL(5,4),
    actual_value DECIMAL(5,4),
    
    -- Actionable Information
    recommended_actions TEXT[],
    urgency_level VARCHAR(20),
    time_sensitive BOOLEAN DEFAULT FALSE,
    
    -- Notification Status
    sent_at TIMESTAMP WITH TIME ZONE,
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_at TIMESTAMP WITH TIME ZONE,
    acknowledged_by VARCHAR(100),
    
    -- Response Tracking
    action_taken TEXT,
    action_taken_at TIMESTAMP WITH TIME ZONE,
    action_taken_by VARCHAR(100),
    
    -- Delivery Tracking
    email_sent BOOLEAN DEFAULT FALSE,
    slack_sent BOOLEAN DEFAULT FALSE,
    webhook_sent BOOLEAN DEFAULT FALSE,
    
    -- Timestamps
    triggered_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_alerts_level ON alerts(alert_level);
CREATE INDEX idx_alerts_type ON alerts(alert_type);
CREATE INDEX idx_alerts_triggered ON alerts(triggered_at DESC);
CREATE INDEX idx_alerts_assessment ON alerts(assessment_id);
CREATE INDEX idx_alerts_acknowledged ON alerts(acknowledged, triggered_at DESC);
```

### 2.9 Visualization Metadata

```sql
-- Metadata about generated visualizations
CREATE TABLE visualizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    
    -- Visualization Identity
    visualization_type VARCHAR(100) NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    
    -- File Information
    file_path TEXT NOT NULL,
    file_format VARCHAR(20) NOT NULL,  -- 'SVG', 'PNG', 'HTML', 'ASCII', etc.
    file_size_bytes INTEGER,
    
    -- Rendering Details
    width INTEGER,
    height INTEGER,
    resolution_dpi INTEGER,
    color_scheme VARCHAR(50),
    
    -- Content Summary
    data_points_visualized INTEGER,
    time_range_start DATE,
    time_range_end DATE,
    
    -- Generation Metadata
    generation_duration_ms INTEGER,
    rendering_engine VARCHAR(50),
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_visualizations_assessment ON visualizations(assessment_id);
CREATE INDEX idx_visualizations_type ON visualizations(visualization_type);
```

---

## 3. Advanced Features

### 3.1 Time-Series Optimization

```sql
-- Convert assessments table to hypertable for time-series optimization
-- (Requires TimescaleDB extension)
SELECT create_hypertable('assessments', 'assessment_date');

-- Create continuous aggregates for efficient historical queries
CREATE MATERIALIZED VIEW assessment_daily_stats
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 day', assessment_date) AS day,
    AVG(seconds_to_midnight) AS avg_seconds,
    MIN(seconds_to_midnight) AS min_seconds,
    MAX(seconds_to_midnight) AS max_seconds,
    AVG(raw_risk_score) AS avg_risk_score,
    COUNT(*) AS assessment_count
FROM assessments
GROUP BY day;

-- Retention policy: keep raw data for 5 years
SELECT add_retention_policy('assessments', INTERVAL '5 years');
```

### 3.2 Full-Text Search Configuration

```sql
-- Enhanced full-text search with custom dictionary
CREATE TEXT SEARCH CONFIGURATION nuclear_search (COPY = english);

-- Add custom stop words and synonyms
ALTER TEXT SEARCH CONFIGURATION nuclear_search
    ALTER MAPPING FOR asciiword, asciihword, hword_asciipart, word, hword, hword_part
    WITH unaccent, english_stem;

-- Create full-text index on key content fields
CREATE INDEX idx_assessments_fts ON assessments 
    USING GIN (to_tsvector('nuclear_search', executive_summary || ' ' || detailed_analysis));

CREATE INDEX idx_collected_data_fts ON collected_data
    USING GIN (to_tsvector('nuclear_search', title || ' ' || content));
```

### 3.3 Materialized Views for Performance

```sql
-- Pre-computed risk factor rankings
CREATE MATERIALIZED VIEW risk_factor_rankings AS
SELECT 
    rf.factor_name,
    rf.factor_category,
    AVG(rf.contribution_to_risk) AS avg_contribution,
    COUNT(*) AS occurrence_count,
    MAX(rf.contribution_to_risk) AS max_contribution,
    AVG(rf.confidence_score) AS avg_confidence
FROM risk_factors rf
JOIN assessments a ON rf.assessment_id = a.id
WHERE a.assessment_date >= NOW() - INTERVAL '1 year'
GROUP BY rf.factor_name, rf.factor_category
ORDER BY avg_contribution DESC;

-- Refresh policy
CREATE INDEX ON risk_factor_rankings (avg_contribution DESC);
REFRESH MATERIALIZED VIEW risk_factor_rankings;

-- Arsenal trends summary
CREATE MATERIALIZED VIEW arsenal_trends AS
SELECT 
    country,
    MAX(observation_date) AS latest_date,
    AVG(total_warheads) OVER (
        PARTITION BY country 
        ORDER BY observation_date 
        ROWS BETWEEN 11 PRECEDING AND CURRENT ROW
    ) AS moving_avg_12_months,
    total_warheads - LAG(total_warheads, 12) OVER (
        PARTITION BY country 
        ORDER BY observation_date
    ) AS change_12_months
FROM nuclear_arsenals
ORDER BY country, observation_date DESC;
```

---

## 4. Query Patterns & Optimizations

### 4.1 Common Query Patterns

```rust
/// Rust query implementations
impl Database {
    /// Get latest assessment
    pub async fn get_latest_assessment(&self) -> Result<Assessment> {
        sqlx::query_as!(
            Assessment,
            r#"
            SELECT * FROM assessments
            ORDER BY assessment_date DESC
            LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
    
    /// Get assessment trend over period
    pub async fn get_assessment_trend(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>
    ) -> Result<Vec<AssessmentSummary>> {
        sqlx::query_as!(
            AssessmentSummary,
            r#"
            SELECT 
                assessment_date,
                seconds_to_midnight,
                raw_risk_score,
                trend_direction
            FROM assessments
            WHERE assessment_date BETWEEN $1 AND $2
            ORDER BY assessment_date ASC
            "#,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
    
    /// Get top risk factors for assessment
    pub async fn get_top_risk_factors(
        &self,
        assessment_id: Uuid,
        limit: i64
    ) -> Result<Vec<RiskFactor>> {
        sqlx::query_as!(
            RiskFactor,
            r#"
            SELECT * FROM risk_factors
            WHERE assessment_id = $1
            ORDER BY contribution_to_risk DESC
            LIMIT $2
            "#,
            assessment_id,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
    
    /// Search historical events
    pub async fn search_historical_events(
        &self,
        search_term: &str
    ) -> Result<Vec<NuclearEvent>> {
        sqlx::query_as!(
            NuclearEvent,
            r#"
            SELECT * FROM nuclear_events
            WHERE to_tsvector('english', event_name || ' ' || description) 
                  @@ plainto_tsquery('english', $1)
            ORDER BY severity_score DESC, event_date DESC
            LIMIT 50
            "#,
            search_term
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
    
    /// Get arsenal changes for country
    pub async fn get_arsenal_history(
        &self,
        country: &str,
        start_date: NaiveDate
    ) -> Result<Vec<NuclearArsenal>> {
        sqlx::query_as!(
            NuclearArsenal,
            r#"
            SELECT * FROM nuclear_arsenals
            WHERE country = $1 
              AND observation_date >= $2
            ORDER BY observation_date ASC
            "#,
            country,
            start_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### 4.2 Complex Analytical Queries

```sql
-- Calculate rolling risk average
SELECT 
    assessment_date,
    seconds_to_midnight,
    AVG(seconds_to_midnight) OVER (
        ORDER BY assessment_date
        ROWS BETWEEN 29 PRECEDING AND CURRENT ROW
    ) AS rolling_avg_30_days
FROM assessments
ORDER BY assessment_date DESC;

-- Identify risk factor trends
SELECT 
    rf.factor_name,
    DATE_TRUNC('month', a.assessment_date) AS month,
    AVG(rf.contribution_to_risk) AS avg_contribution,
    STDDEV(rf.contribution_to_risk) AS stddev_contribution
FROM risk_factors rf
JOIN assessments a ON rf.assessment_id = a.id
WHERE a.assessment_date >= NOW() - INTERVAL '2 years'
GROUP BY rf.factor_name, DATE_TRUNC('month', a.assessment_date)
ORDER BY factor_name, month;

-- Compare current to historical averages
WITH historical_avg AS (
    SELECT AVG(seconds_to_midnight) AS avg_seconds
    FROM assessments
    WHERE assessment_date BETWEEN 
        NOW() - INTERVAL '5 years' AND 
        NOW() - INTERVAL '1 year'
)
SELECT 
    a.assessment_date,
    a.seconds_to_midnight,
    h.avg_seconds AS historical_avg,
    a.seconds_to_midnight - h.avg_seconds AS deviation_from_historical
FROM assessments a
CROSS JOIN historical_avg h
WHERE a.assessment_date >= NOW() - INTERVAL '1 year'
ORDER BY a.assessment_date DESC;

-- Correlation between risk factors
SELECT 
    rf1.factor_name AS factor_1,
    rf2.factor_name AS factor_2,
    CORR(rf1.raw_value, rf2.raw_value) AS correlation
FROM risk_factors rf1
JOIN risk_factors rf2 ON rf1.assessment_id = rf2.assessment_id
WHERE rf1.factor_name < rf2.factor_name
GROUP BY rf1.factor_name, rf2.factor_name
HAVING COUNT(*) >= 30  -- Require sufficient data points
ORDER BY ABS(CORR(rf1.raw_value, rf2.raw_value)) DESC;
```

---

## 5. Data Integrity & Constraints

### 5.1 Check Constraints

```sql
-- Ensure logical consistency
ALTER TABLE assessments ADD CONSTRAINT check_risk_score_range
    CHECK (raw_risk_score >= 0.0 AND raw_risk_score <= 1.0);

ALTER TABLE assessments ADD CONSTRAINT check_seconds_range
    CHECK (seconds_to_midnight >= 0 AND seconds_to_midnight <= 1440);

ALTER TABLE risk_factors ADD CONSTRAINT check_contribution_positive
    CHECK (contribution_to_risk >= 0.0);

ALTER TABLE nuclear_arsenals ADD CONSTRAINT check_warhead_counts
    CHECK (
        total_warheads >= deployed_strategic + COALESCE(deployed_tactical, 0) + 
        COALESCE(reserve_warheads, 0)
    );
```

### 5.2 Foreign Key Cascades

```sql
-- Define cascade behavior
ALTER TABLE risk_factors
    ADD CONSTRAINT fk_assessment
    FOREIGN KEY (assessment_id)
    REFERENCES assessments(id)
    ON DELETE CASCADE;

ALTER TABLE alerts
    ADD CONSTRAINT fk_assessment
    FOREIGN KEY (assessment_id)
    REFERENCES assessments(id)
    ON DELETE CASCADE;

-- Prevent deletion of referenced sources
ALTER TABLE collected_data
    ADD CONSTRAINT fk_source
    FOREIGN KEY (source_id)
    REFERENCES data_sources(id)
    ON DELETE RESTRICT;
```

### 5.3 Audit Triggers

```sql
-- Audit trail for sensitive tables
CREATE TABLE audit_log (
    id BIGSERIAL PRIMARY KEY,
    table_name VARCHAR(100) NOT NULL,
    record_id UUID NOT NULL,
    operation VARCHAR(10) NOT NULL,
    old_values JSONB,
    new_values JSONB,
    changed_by VARCHAR(100),
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION audit_trigger_func()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        INSERT INTO audit_log (table_name, record_id, operation, old_values)
        VALUES (TG_TABLE_NAME, OLD.id, 'DELETE', row_to_json(OLD));
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
        VALUES (TG_TABLE_NAME, NEW.id, 'UPDATE', row_to_json(OLD), row_to_json(NEW));
        RETURN NEW;
    ELSIF TG_OP = 'INSERT' THEN
        INSERT INTO audit_log (table_name, record_id, operation, new_values)
        VALUES (TG_TABLE_NAME, NEW.id, 'INSERT', row_to_json(NEW));
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Apply to critical tables
CREATE TRIGGER audit_assessments
    AFTER INSERT OR UPDATE OR DELETE ON assessments
    FOR EACH ROW EXECUTE FUNCTION audit_trigger_func();
```

---

## 6. Backup & Recovery

### 6.1 Backup Strategy

```bash
#!/bin/bash
# Daily automated backup script

DB_NAME="wargames"
BACKUP_DIR="/var/backups/wargames"
DATE=$(date +%Y%m%d_%H%M%S)

# Full database backup
pg_dump -Fc $DB_NAME > "$BACKUP_DIR/full_${DATE}.dump"

# Schema-only backup
pg_dump -s $DB_NAME > "$BACKUP_DIR/schema_${DATE}.sql"

# Data-only backup for critical tables
pg_dump -a -t assessments -t risk_factors \
    $DB_NAME > "$BACKUP_DIR/critical_data_${DATE}.sql"

# Compress old backups
find "$BACKUP_DIR" -name "*.dump" -mtime +7 -exec gzip {} \;

# Remove backups older than 90 days
find "$BACKUP_DIR" -name "*.gz" -mtime +90 -delete
```

### 6.2 Point-in-Time Recovery

```sql
-- Enable WAL archiving for PITR
ALTER SYSTEM SET wal_level = replica;
ALTER SYSTEM SET archive_mode = on;
ALTER SYSTEM SET archive_command = 'cp %p /var/lib/postgresql/wal_archive/%f';

-- Create restore point before major operations
SELECT pg_create_restore_point('before_major_update');
```

---

## 7. Monitoring & Maintenance

### 7.1 Performance Monitoring Queries

```sql
-- Table sizes and bloat
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) AS size,
    n_live_tup AS live_tuples,
    n_dead_tup AS dead_tuples
FROM pg_stat_user_tables
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

-- Index usage statistics
SELECT 
    schemaname,
    tablename,
    indexname,
    idx_scan AS index_scans,
    idx_tup_read AS tuples_read,
    idx_tup_fetch AS tuples_fetched
FROM pg_stat_user_indexes
ORDER BY idx_scan DESC;

-- Slow queries from pg_stat_statements
SELECT 
    query,
    calls,
    total_exec_time / 1000 AS total_time_seconds,
    mean_exec_time / 1000 AS mean_time_seconds,
    max_exec_time / 1000 AS max_time_seconds
FROM pg_stat_statements
ORDER BY total_exec_time DESC
LIMIT 20;
```

### 7.2 Maintenance Tasks

```sql
-- Vacuum and analyze schedule
-- (Run via cron or pg_cron extension)
VACUUM ANALYZE assessments;
VACUUM ANALYZE risk_factors;
VACUUM ANALYZE collected_data;

-- Reindex periodically
REINDEX TABLE CONCURRENTLY assessments;

-- Update statistics
ANALYZE;
```

---

## 8. Migration Management

### 8.1 Migration Framework

```rust
// migrations/001_initial_schema.sql
// migrations/002_add_arsenal_tracking.sql
// migrations/003_add_fulltext_search.sql

impl Database {
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}
```

---

## Conclusion

This comprehensive database design provides a robust foundation for the WarGames/JOSHUA system. The schema supports efficient storage and retrieval of historical assessments, enables complex analytical queries, and scales to accommodate years of data accumulation.

### Key Features

1. **Normalized Design**: Eliminates redundancy while maintaining performance
2. **Rich Indexing**: Strategic indexes for all common query patterns
3. **Data Integrity**: Constraints and foreign keys enforce consistency
4. **Audit Trail**: Complete history of all changes
5. **Performance Optimization**: Materialized views and time-series support
6. **Full-Text Search**: Efficient content search capabilities

**The database is the system's memoryâ€”without it, we cannot learn from history.**

*"Those who cannot remember the past are condemned to repeat it." - George Santayana*
