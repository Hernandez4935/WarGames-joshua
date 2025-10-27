# WarGames/JOSHUA: Monitoring and Alerting
## Complete Observability and Incident Response Strategy
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Overview](#1-overview)
2. [Monitoring Architecture](#2-monitoring-architecture)
3. [Metrics Collection](#3-metrics-collection)
4. [Logging Strategy](#4-logging-strategy)
5. [Alerting Framework](#5-alerting-framework)
6. [Dashboards](#6-dashboards)
7. [SLOs and SLAs](#7-slos-and-slas)
8. [Incident Response](#8-incident-response)
9. [Performance Monitoring](#9-performance-monitoring)
10. [Cost Monitoring](#10-cost-monitoring)

---

## 1. Overview

### 1.1 Purpose

The WarGames/JOSHUA system monitors existential nuclear risk - **system availability is critical**. This document specifies comprehensive monitoring and alerting to ensure:

- 99.9% uptime (< 43 minutes downtime/month)
- < 5 minute detection time for critical failures
- < 15 minute response time for critical incidents
- Complete observability into all system components

### 1.2 Monitoring Philosophy

**Layered Defense:**
```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Layer 1: Infrastructure (AWS CloudWatch)            â"‚
â"‚ - CPU, Memory, Disk, Network                        â"‚
â"‚ - EC2, RDS, ElastiCache health                      â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Layer 2: Application (Prometheus + Grafana)         â"‚
â"‚ - Request rates, latencies, error rates             â"‚
â"‚ - Database connection pools, query times            â"‚
â"‚ - Cache hit rates, API call latencies               â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Layer 3: Business Logic (Custom Metrics)            â"‚
â"‚ - Risk assessment completion rates                  â"‚
â"‚ - Data collection success rates                     â"‚
â"‚ - Risk score trends and anomalies                   â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ Layer 4: External Dependencies                      â"‚
â"‚ - Claude API availability and latency               â"‚
â"‚ - Data source availability                          â"‚
â"‚ - Third-party service health                        â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 1.3 Tool Stack

**Primary Tools:**
- **Metrics:** Prometheus + AWS CloudWatch
- **Logging:** Loki + CloudWatch Logs
- **Tracing:** OpenTelemetry + Jaeger
- **Dashboards:** Grafana
- **Alerting:** AlertManager + PagerDuty + Slack
- **Status Page:** Statuspage.io

---

## 2. Monitoring Architecture

### 2.1 System Architecture

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚                 WarGames/JOSHUA System              â"‚
â"‚                                                      â"‚
â"‚  â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"    â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"    â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"  â"‚
â"‚  â"‚   API      â"‚    â"‚  Workers   â"‚    â"‚ Database  â"‚  â"‚
â"‚  â"‚  Service   â"‚    â"‚            â"‚    â"‚           â"‚  â"‚
â"‚  â""â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜    â""â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜    â""â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜  â"‚
â"‚       â"‚              â"‚              â"‚         â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
        â"‚              â"‚              â"‚
        â"‚ Metrics    â"‚              â"‚
        â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
                       â"‚
        â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
        â"‚              â–¼              â"‚
        â"‚      â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"      â"‚
        â"‚      â"‚  Prometheus  â"‚      â"‚
        â"‚      â""â"€â"€â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜      â"‚
        â"‚             â"‚             â"‚
        â"‚      â"Œâ"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"      â"‚
        â"‚      â"‚   Grafana   â"‚      â"‚
        â"‚      â""â"€â"€â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜      â"‚
        â"‚             â"‚             â"‚
        â"‚      â"Œâ"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"      â"‚
        â"‚      â"‚AlertManager â"‚      â"‚
        â"‚      â""â"€â"€â"€â"€â"€â"€â"¬â"€â"€â"€â"€â"€â"€â"˜      â"‚
        â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
                     â"‚
        â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
        â"‚            â"‚            â"‚
        â–¼            â–¼            â–¼
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"  â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"  â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Slack  â"‚  â"‚PagerDutyâ"‚  â"‚  Email  â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜  â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜  â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 2.2 Prometheus Configuration

```yaml
# /etc/prometheus/prometheus.yml

global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'wargames-production'
    environment: 'prod'

# Alertmanager configuration
alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']

# Load rules
rule_files:
  - '/etc/prometheus/rules/*.yml'

# Scrape configurations
scrape_configs:
  # WarGames API Service
  - job_name: 'wargames-api'
    static_configs:
      - targets: 
          - 'api-1.wargames.internal:9090'
          - 'api-2.wargames.internal:9090'
          - 'api-3.wargames.internal:9090'
    relabel_configs:
      - source_labels: [__address__]
        target_label: instance
      - source_labels: [__address__]
        regex: '([^:]+).*'
        target_label: hostname
        
  # Worker Nodes
  - job_name: 'wargames-workers'
    static_configs:
      - targets:
          - 'worker-1.wargames.internal:9090'
          - 'worker-2.wargames.internal:9090'
          - 'worker-3.wargames.internal:9090'
          
  # PostgreSQL
  - job_name: 'postgresql'
    static_configs:
      - targets: ['postgres-exporter:9187']
        
  # Redis
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']
        
  # Node Exporter (System Metrics)
  - job_name: 'node'
    static_configs:
      - targets:
          - 'api-1.wargames.internal:9100'
          - 'api-2.wargames.internal:9100'
          - 'worker-1.wargames.internal:9100'
          
  # AWS CloudWatch Exporter
  - job_name: 'cloudwatch'
    static_configs:
      - targets: ['cloudwatch-exporter:9106']
```

### 2.3 Application Metrics Exporter

```rust
/// Prometheus metrics exporter for WarGames/JOSHUA
use prometheus::{
    Encoder, TextEncoder, Counter, Gauge, Histogram, HistogramOpts,
    register_counter, register_gauge, register_histogram,
};

lazy_static! {
    // Request metrics
    pub static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "wargames_http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    pub static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "wargames_http_request_duration_seconds",
        "HTTP request duration in seconds",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]
    ).unwrap();
    
    // Assessment metrics
    pub static ref ASSESSMENTS_TOTAL: Counter = register_counter!(
        "wargames_assessments_total",
        "Total number of assessments triggered"
    ).unwrap();
    
    pub static ref ASSESSMENTS_COMPLETED: Counter = register_counter!(
        "wargames_assessments_completed_total",
        "Total number of completed assessments"
    ).unwrap();
    
    pub static ref ASSESSMENTS_FAILED: Counter = register_counter!(
        "wargames_assessments_failed_total",
        "Total number of failed assessments"
    ).unwrap();
    
    pub static ref ASSESSMENT_DURATION: Histogram = register_histogram!(
        "wargames_assessment_duration_seconds",
        "Assessment completion time in seconds",
        vec![10.0, 30.0, 60.0, 120.0, 180.0, 300.0, 600.0]
    ).unwrap();
    
    pub static ref CURRENT_RISK_SCORE: Gauge = register_gauge!(
        "wargames_current_risk_score",
        "Current risk score (seconds to midnight)"
    ).unwrap();
    
    pub static ref RISK_CONFIDENCE: Gauge = register_gauge!(
        "wargames_risk_confidence",
        "Confidence level of current risk assessment"
    ).unwrap();
    
    // Data collection metrics
    pub static ref DATA_POINTS_COLLECTED: Counter = register_counter!(
        "wargames_data_points_collected_total",
        "Total data points collected"
    ).unwrap();
    
    pub static ref DATA_SOURCES_ACTIVE: Gauge = register_gauge!(
        "wargames_data_sources_active",
        "Number of active data sources"
    ).unwrap();
    
    pub static ref DATA_SOURCES_FAILED: Gauge = register_gauge!(
        "wargames_data_sources_failed",
        "Number of failed data sources"
    ).unwrap();
    
    pub static ref DATA_COLLECTION_DURATION: Histogram = register_histogram!(
        "wargames_data_collection_duration_seconds",
        "Data collection duration in seconds"
    ).unwrap();
    
    // Claude API metrics
    pub static ref CLAUDE_REQUESTS_TOTAL: Counter = register_counter!(
        "wargames_claude_requests_total",
        "Total number of Claude API requests"
    ).unwrap();
    
    pub static ref CLAUDE_REQUESTS_FAILED: Counter = register_counter!(
        "wargames_claude_requests_failed_total",
        "Total number of failed Claude API requests"
    ).unwrap();
    
    pub static ref CLAUDE_REQUEST_DURATION: Histogram = register_histogram!(
        "wargames_claude_request_duration_seconds",
        "Claude API request duration in seconds",
        vec![1.0, 2.0, 5.0, 10.0, 30.0, 60.0, 120.0, 180.0]
    ).unwrap();
    
    pub static ref CLAUDE_TOKENS_INPUT: Counter = register_counter!(
        "wargames_claude_tokens_input_total",
        "Total input tokens sent to Claude API"
    ).unwrap();
    
    pub static ref CLAUDE_TOKENS_OUTPUT: Counter = register_counter!(
        "wargames_claude_tokens_output_total",
        "Total output tokens received from Claude API"
    ).unwrap();
    
    pub static ref CLAUDE_COST_USD: Counter = register_counter!(
        "wargames_claude_cost_usd_total",
        "Total cost of Claude API usage in USD"
    ).unwrap();
    
    // Database metrics
    pub static ref DB_CONNECTIONS_ACTIVE: Gauge = register_gauge!(
        "wargames_db_connections_active",
        "Number of active database connections"
    ).unwrap();
    
    pub static ref DB_QUERY_DURATION: Histogram = register_histogram!(
        "wargames_db_query_duration_seconds",
        "Database query duration in seconds",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0]
    ).unwrap();
    
    // Cache metrics
    pub static ref CACHE_HITS: Counter = register_counter!(
        "wargames_cache_hits_total",
        "Total number of cache hits"
    ).unwrap();
    
    pub static ref CACHE_MISSES: Counter = register_counter!(
        "wargames_cache_misses_total",
        "Total number of cache misses"
    ).unwrap();
}

/// Metrics endpoint handler
pub async fn metrics_handler() -> impl axum::response::IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        buffer,
    )
}
```

---

## 3. Metrics Collection

### 3.1 Key Metrics Categories

**System Health Metrics:**
```
wargames_up{instance, job}                            # Service uptime
wargames_http_requests_total{method, status}          # Request count
wargames_http_request_duration_seconds{quantile}      # Request latency
wargames_errors_total{type, component}                # Error count
```

**Assessment Metrics:**
```
wargames_assessments_total                            # Total assessments
wargames_assessments_completed_total                  # Completed count
wargames_assessments_failed_total                     # Failed count
wargames_assessment_duration_seconds{quantile}        # Duration
wargames_current_risk_score                           # Current risk
wargames_risk_confidence                              # Confidence level
wargames_risk_score_change_rate                       # Risk velocity
```

**Data Collection Metrics:**
```
wargames_data_points_collected_total{source}          # Data points
wargames_data_sources_active                          # Active sources
wargames_data_sources_failed                          # Failed sources
wargames_data_quality_score{source}                   # Quality score
wargames_data_collection_duration_seconds{source}     # Collection time
```

**Claude API Metrics:**
```
wargames_claude_requests_total{model}                 # API requests
wargames_claude_requests_failed_total{error_type}     # Failed requests
wargames_claude_request_duration_seconds{quantile}    # Request latency
wargames_claude_tokens_input_total                    # Input tokens
wargames_claude_tokens_output_total                   # Output tokens
wargames_claude_cost_usd_total                        # Total cost
```

**Database Metrics:**
```
wargames_db_connections_active                        # Active connections
wargames_db_connections_max                           # Max connections
wargames_db_query_duration_seconds{query_type}        # Query time
wargames_db_size_bytes                                # Database size
```

**Cache Metrics:**
```
wargames_cache_hits_total                             # Cache hits
wargames_cache_misses_total                           # Cache misses
wargames_cache_hit_rate                               # Hit rate
wargames_cache_size_bytes                             # Cache size
```

### 3.2 Custom Business Metrics

```rust
/// Record custom business metrics
pub struct BusinessMetrics {
    metrics_client: PrometheusClient,
}

impl BusinessMetrics {
    /// Record completed assessment
    pub fn record_assessment_completed(
        &self,
        assessment: &Assessment,
    ) {
        ASSESSMENTS_COMPLETED.inc();
        CURRENT_RISK_SCORE.set(assessment.seconds_to_midnight as f64);
        RISK_CONFIDENCE.set(assessment.confidence);
        
        // Calculate risk change rate
        if let Some(prev) = &assessment.previous_score {
            let change = assessment.seconds_to_midnight - prev;
            let rate = change as f64 / assessment.time_since_previous.as_secs() as f64;
            
            prometheus::register_gauge!(
                "wargames_risk_score_change_rate",
                "Rate of risk score change (seconds per hour)"
            ).unwrap().set(rate * 3600.0);
        }
        
        // Record by risk level
        prometheus::register_counter!(
            "wargames_assessments_by_level",
            "Assessments by risk level",
            &["level"]
        ).unwrap()
            .with_label_values(&[&assessment.risk_level.to_string()])
            .inc();
    }
    
    /// Record data collection metrics
    pub fn record_data_collection(
        &self,
        result: &DataCollectionResult,
    ) {
        DATA_POINTS_COLLECTED.inc_by(result.total_points as u64);
        DATA_SOURCES_ACTIVE.set(result.sources_active as f64);
        DATA_SOURCES_FAILED.set(result.sources_failed as f64);
        
        // Record per-source metrics
        for source in &result.sources {
            prometheus::register_histogram!(
                "wargames_data_source_latency_seconds",
                "Data source collection latency",
                &["source"]
            ).unwrap()
                .with_label_values(&[&source.name])
                .observe(source.latency.as_secs_f64());
            
            prometheus::register_gauge!(
                "wargames_data_source_quality",
                "Data source quality score",
                &["source"]
            ).unwrap()
                .with_label_values(&[&source.name])
                .set(source.quality_score);
        }
    }
    
    /// Record Claude API usage
    pub fn record_claude_usage(
        &self,
        response: &ClaudeResponse,
    ) {
        CLAUDE_REQUESTS_TOTAL.inc();
        CLAUDE_TOKENS_INPUT.inc_by(response.usage.input_tokens as u64);
        CLAUDE_TOKENS_OUTPUT.inc_by(response.usage.output_tokens as u64);
        
        // Calculate cost
        let cost = self.calculate_claude_cost(&response.usage);
        CLAUDE_COST_USD.inc_by((cost * 100.0) as u64);  // Store as cents
        
        if let Some(latency) = response.latency {
            CLAUDE_REQUEST_DURATION.observe(latency.as_secs_f64());
        }
    }
}
```

---

## 4. Logging Strategy

### 4.1 Log Levels and Structure

```rust
/// Structured logging with tracing
use tracing::{error, warn, info, debug, trace};

// Log levels:
// ERROR: System failures, data loss, critical issues
// WARN:  Degraded performance, recoverable errors
// INFO:  Normal operations, assessment completions
// DEBUG: Detailed operational info
// TRACE: Very detailed debugging info

/// Initialize logging
pub fn init_logging() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .json()  // JSON format for structured logging
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("wargames=info".parse()?)
                .add_directive("tower_http=debug".parse()?)
                .add_directive("sqlx=warn".parse()?)
        )
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;
    
    Ok(())
}

/// Structured log examples
pub async fn example_logging() {
    // Assessment started
    info!(
        assessment_id = %assessment_id,
        trigger_source = %trigger,
        priority = %priority,
        "Assessment started"
    );
    
    // Data collection
    info!(
        sources_active = sources_active,
        sources_failed = sources_failed,
        data_points = data_points,
        duration_ms = duration.as_millis(),
        "Data collection completed"
    );
    
    // Claude API call
    info!(
        model = %model,
        input_tokens = input_tokens,
        output_tokens = output_tokens,
        latency_ms = latency.as_millis(),
        cost_usd = %cost,
        "Claude API request completed"
    );
    
    // Assessment completed
    info!(
        assessment_id = %assessment_id,
        seconds_to_midnight = seconds,
        risk_level = %level,
        confidence = %confidence,
        duration_sec = duration.as_secs(),
        "Assessment completed successfully"
    );
    
    // Error logging
    error!(
        error = %err,
        assessment_id = %assessment_id,
        component = "data_collector",
        source = %source_name,
        "Data collection failed"
    );
}
```

### 4.2 Log Aggregation

```yaml
# Loki configuration
auth_enabled: false

server:
  http_listen_port: 3100

ingester:
  lifecycler:
    ring:
      kvstore:
        store: inmemory
      replication_factor: 1
  chunk_idle_period: 5m
  chunk_retain_period: 30s

schema_config:
  configs:
    - from: 2024-01-01
      store: boltdb-shipper
      object_store: s3
      schema: v11
      index:
        prefix: loki_index_
        period: 24h

storage_config:
  boltdb_shipper:
    active_index_directory: /loki/index
    cache_location: /loki/cache
    shared_store: s3
  aws:
    s3: s3://wargames-logs/loki
    region: us-east-1

limits_config:
  retention_period: 2555h  # 7 years (per compliance)
  
compactor:
  working_directory: /loki/compactor
  shared_store: s3
  compaction_interval: 10m
```

### 4.3 CloudWatch Logs Integration

```rust
/// Send critical logs to CloudWatch
pub struct CloudWatchLogger {
    client: aws_sdk_cloudwatchlogs::Client,
    log_group: String,
    log_stream: String,
}

impl CloudWatchLogger {
    pub async fn log_critical_event(
        &self,
        event: CriticalEvent,
    ) -> Result<()> {
        let message = serde_json::to_string(&event)?;
        
        self.client
            .put_log_events()
            .log_group_name(&self.log_group)
            .log_stream_name(&self.log_stream)
            .log_events(
                aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                    .timestamp(Utc::now().timestamp_millis())
                    .message(message)
                    .build()
            )
            .send()
            .await?;
        
        Ok(())
    }
}
```

---

## 5. Alerting Framework

### 5.1 Alert Severity Levels

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Severity â"‚ Response Time â"‚ Escalation â"‚ Channels      â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â "€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ CRITICAL â"‚ Immediate    â"‚ Page on-call â"‚ PagerDuty +   â"‚
â"‚          â"‚ (< 5 min)    â"‚ engineer     â"‚ Slack + Email â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ SEVERE   â"‚ < 15 min     â"‚ Page if not  â"‚ Slack + Email â"‚
â"‚          â"‚              â"‚ ack'd in 15m â"‚               â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ WARNING  â"‚ < 1 hour     â"‚ During       â"‚ Slack         â"‚
â"‚          â"‚              â"‚ business hrs â"‚               â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ INFO     â"‚ Best effort  â"‚ None         â"‚ Slack         â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

### 5.2 Prometheus Alert Rules

```yaml
# /etc/prometheus/rules/wargames_alerts.yml

groups:
  - name: wargames_critical
    interval: 30s
    rules:
      # System down
      - alert: WarGamesServiceDown
        expr: up{job="wargames-api"} == 0
        for: 2m
        labels:
          severity: critical
          component: api
        annotations:
          summary: "WarGames service is down"
          description: "{{ $labels.instance }} has been down for more than 2 minutes"
          
      # High error rate
      - alert: HighErrorRate
        expr: |
          rate(wargames_http_requests_total{status=~"5.."}[5m]) /
          rate(wargames_http_requests_total[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value | humanizePercentage }} over the last 5 minutes"
          
      # Assessment failures
      - alert: AssessmentFailureRate
        expr: |
          rate(wargames_assessments_failed_total[30m]) /
          rate(wargames_assessments_total[30m]) > 0.1
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "High assessment failure rate"
          description: "{{ $value | humanizePercentage }} of assessments failing"
          
      # Critical risk score change
      - alert: CriticalRiskScoreChange
        expr: |
          abs(wargames_current_risk_score - wargames_current_risk_score offset 1h) > 120
        labels:
          severity: critical
        annotations:
          summary: "Critical nuclear risk score change detected"
          description: "Risk score changed by {{ $value }} seconds in last hour"
          
      # Database down
      - alert: DatabaseDown
        expr: up{job="postgresql"} == 0
        for: 1m
        labels:
          severity: critical
          component: database
        annotations:
          summary: "PostgreSQL database is down"
          description: "Database has been unreachable for more than 1 minute"
          
      # Claude API failures
      - alert: ClaudeAPIFailureRate
        expr: |
          rate(wargames_claude_requests_failed_total[15m]) /
          rate(wargames_claude_requests_total[15m]) > 0.2
        for: 5m
        labels:
          severity: critical
          component: claude_api
        annotations:
          summary: "High Claude API failure rate"
          description: "{{ $value | humanizePercentage }} of Claude API requests failing"

  - name: wargames_severe
    interval: 1m
    rules:
      # High latency
      - alert: HighLatency
        expr: |
          histogram_quantile(0.95, rate(wargames_http_request_duration_seconds_bucket[5m])) > 5.0
        for: 10m
        labels:
          severity: severe
        annotations:
          summary: "High API latency detected"
          description: "P95 latency is {{ $value }}s over the last 10 minutes"
          
      # Data collection failures
      - alert: DataCollectionFailures
        expr: wargames_data_sources_failed > 5
        for: 15m
        labels:
          severity: severe
          component: data_collection
        annotations:
          summary: "Multiple data sources failing"
          description: "{{ $value }} data sources have failed"
          
      # Low data quality
      - alert: LowDataQuality
        expr: |
          avg(wargames_data_quality_score) < 0.7
        for: 30m
        labels:
          severity: severe
        annotations:
          summary: "Low data quality detected"
          description: "Average data quality score is {{ $value }}"
          
      # Database connection pool exhaustion
      - alert: DatabaseConnectionPoolNearLimit
        expr: |
          wargames_db_connections_active / wargames_db_connections_max > 0.9
        for: 5m
        labels:
          severity: severe
          component: database
        annotations:
          summary: "Database connection pool near capacity"
          description: "Using {{ $value | humanizePercentage }} of available connections"

  - name: wargames_warning
    interval: 2m
    rules:
      # Slow assessments
      - alert: SlowAssessments
        expr: |
          histogram_quantile(0.95, rate(wargames_assessment_duration_seconds_bucket[30m])) > 600
        for: 15m
        labels:
          severity: warning
        annotations:
          summary: "Assessments taking longer than expected"
          description: "P95 assessment duration is {{ $value }}s"
          
      # Cache hit rate declining
      - alert: LowCacheHitRate
        expr: |
          rate(wargames_cache_hits_total[30m]) /
          (rate(wargames_cache_hits_total[30m]) + rate(wargames_cache_misses_total[30m])) < 0.7
        for: 30m
        labels:
          severity: warning
        annotations:
          summary: "Cache hit rate is low"
          description: "Cache hit rate is {{ $value | humanizePercentage }}"
          
      # High Claude API cost
      - alert: HighClaudeAPICost
        expr: |
          increase(wargames_claude_cost_usd_total[24h]) > 1000
        labels:
          severity: warning
        annotations:
          summary: "High Claude API costs"
          description: "Spent ${{ $value }} on Claude API in last 24 hours"
          
      # Disk space
      - alert: DiskSpaceWarning
        expr: |
          (node_filesystem_avail_bytes{mountpoint="/"} /
           node_filesystem_size_bytes{mountpoint="/"}) < 0.2
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Low disk space on {{ $labels.instance }}"
          description: "Only {{ $value | humanizePercentage }} disk space remaining"
```

### 5.3 AlertManager Configuration

```yaml
# /etc/alertmanager/alertmanager.yml

global:
  resolve_timeout: 5m
  slack_api_url: 'https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK'
  pagerduty_url: 'https://events.pagerduty.com/v2/enqueue'

# Alert routing
route:
  receiver: 'default'
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 30s
  group_interval: 5m
  repeat_interval: 12h
  
  routes:
    # Critical alerts -> PagerDuty + Slack + Email
    - match:
        severity: critical
      receiver: 'critical-pager'
      continue: true
    
    - match:
        severity: critical
      receiver: 'critical-slack'
      continue: true
      
    - match:
        severity: critical
      receiver: 'critical-email'
    
    # Severe alerts -> Slack + Email
    - match:
        severity: severe
      receiver: 'severe-slack'
      continue: true
      
    - match:
        severity: severe
      receiver: 'severe-email'
      group_wait: 2m
      
    # Warning alerts -> Slack only
    - match:
        severity: warning
      receiver: 'warning-slack'
      group_wait: 5m
      repeat_interval: 24h

# Receivers
receivers:
  - name: 'default'
    slack_configs:
      - channel: '#wargames-alerts'
        title: 'WarGames Alert'
        text: '{{ range .Alerts }}{{ .Annotations.description }}{{ end }}'
        
  - name: 'critical-pager'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_SERVICE_KEY'
        severity: 'critical'
        description: '{{ .CommonAnnotations.summary }}'
        
  - name: 'critical-slack'
    slack_configs:
      - channel: '#wargames-critical'
        color: 'danger'
        title: ':rotating_light: CRITICAL: {{ .CommonAnnotations.summary }}'
        text: |
          *Description:* {{ .CommonAnnotations.description }}
          *Severity:* {{ .CommonLabels.severity }}
          *Component:* {{ .CommonLabels.component }}
        send_resolved: true
        
  - name: 'critical-email'
    email_configs:
      - to: 'oncall@wargames.example.com'
        from: 'alerts@wargames.example.com'
        subject: 'CRITICAL: {{ .CommonAnnotations.summary }}'
        
  - name: 'severe-slack'
    slack_configs:
      - channel: '#wargames-alerts'
        color: 'warning'
        title: ':warning: SEVERE: {{ .CommonAnnotations.summary }}'
        text: '{{ .CommonAnnotations.description }}'
        send_resolved: true
        
  - name: 'severe-email'
    email_configs:
      - to: 'team@wargames.example.com'
        from: 'alerts@wargames.example.com'
        subject: 'SEVERE: {{ .CommonAnnotations.summary }}'
        
  - name: 'warning-slack'
    slack_configs:
      - channel: '#wargames-alerts'
        color: '#FFCC00'
        title: 'â„¹ï¸ WARNING: {{ .CommonAnnotations.summary }}'
        text: '{{ .CommonAnnotations.description }}'

# Inhibition rules
inhibit_rules:
  # Inhibit warning if critical for same alert
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'instance']
    
  # Inhibit everything if service is down
  - source_match:
      alertname: 'WarGamesServiceDown'
    target_match_re:
      alertname: '.*'
    equal: ['instance']
```

---

## 6. Dashboards

### 6.1 Main System Dashboard

```json
{
  "dashboard": {
    "title": "WarGames/JOSHUA - System Overview",
    "tags": ["wargames", "overview"],
    "timezone": "UTC",
    "panels": [
      {
        "title": "Current Risk Score",
        "type": "gauge",
        "targets": [
          {
            "expr": "wargames_current_risk_score"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "max": 1440,
            "min": 0,
            "thresholds": {
              "steps": [
                { "value": 0, "color": "red" },
                { "value": 120, "color": "orange" },
                { "value": 300, "color": "yellow" },
                { "value": 600, "color": "green" }
              ]
            }
          }
        }
      },
      {
        "title": "Risk Score Trend (7 days)",
        "type": "graph",
        "targets": [
          {
            "expr": "wargames_current_risk_score",
            "legendFormat": "Seconds to Midnight"
          }
        ]
      },
      {
        "title": "Assessment Success Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "rate(wargames_assessments_completed_total[1h]) / rate(wargames_assessments_total[1h])"
          }
        ]
      },
      {
        "title": "API Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(wargames_http_requests_total[5m])",
            "legendFormat": "{{ method }} {{ status }}"
          }
        ]
      },
      {
        "title": "Data Collection Health",
        "type": "stat",
        "targets": [
          {
            "expr": "wargames_data_sources_active / (wargames_data_sources_active + wargames_data_sources_failed)"
          }
        ]
      },
      {
        "title": "Claude API Latency (P95)",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(wargames_claude_request_duration_seconds_bucket[5m]))"
          }
        ]
      }
    ]
  }
}
```

### 6.2 Assessment Details Dashboard

Provides deep insight into assessment execution:
- Assessment duration breakdown
- Risk factor contributions
- Confidence levels over time
- Data quality scores
- Claude API usage per assessment

### 6.3 Infrastructure Dashboard

Monitors underlying infrastructure:
- CPU, memory, disk usage
- Network I/O
- Database performance
- Cache performance
- AWS service health

---

## 7. SLOs and SLAs

### 7.1 Service Level Objectives

```yaml
# SLOs for WarGames/JOSHUA

system_availability:
  target: 99.9%
  measurement_window: 30 days
  error_budget: 43 minutes/month
  
api_latency:
  target_p95: 500ms
  target_p99: 2000ms
  measurement_window: 7 days
  
assessment_completion_rate:
  target: 99%
  measurement_window: 7 days
  
data_collection_success_rate:
  target: 95%
  measurement_window: 24 hours
  
claude_api_success_rate:
  target: 99.5%
  measurement_window: 24 hours
```

### 7.2 SLO Monitoring

```rust
/// Track SLO compliance
pub struct SLOMonitor {
    prometheus_client: PrometheusClient,
    targets: HashMap<String, SLOTarget>,
}

impl SLOMonitor {
    pub async fn check_slo_compliance(&self) -> Result<SLOReport> {
        let mut report = SLOReport::default();
        
        // Check availability SLO
        let uptime = self.calculate_uptime_percentage(Duration::days(30)).await?;
        report.availability = SLOMetric {
            target: 99.9,
            actual: uptime,
            compliant: uptime >= 99.9,
            error_budget_remaining: self.calculate_error_budget(99.9, uptime),
        };
        
        // Check latency SLO
        let p95_latency = self.query_p95_latency(Duration::days(7)).await?;
        report.api_latency_p95 = SLOMetric {
            target: 500.0,
            actual: p95_latency,
            compliant: p95_latency <= 500.0,
            error_budget_remaining: 0.0,  // Binary pass/fail
        };
        
        // Check assessment completion rate
        let completion_rate = self.calculate_assessment_completion_rate(
            Duration::days(7)
        ).await?;
        report.assessment_completion = SLOMetric {
            target: 99.0,
            actual: completion_rate,
            compliant: completion_rate >= 99.0,
            error_budget_remaining: self.calculate_error_budget(99.0, completion_rate),
        };
        
        Ok(report)
    }
}
```

---

## 8. Incident Response

### 8.1 Incident Response Playbook

**Critical Alert Response:**

1. **Acknowledge** (< 5 minutes)
   - Acknowledge alert in PagerDuty
   - Post in #incident-response Slack channel
   - Notify backup on-call if needed

2. **Assess** (< 10 minutes)
   - Check dashboards for scope
   - Review recent deployments
   - Check external dependencies (Claude API, AWS)
   - Determine severity and impact

3. **Mitigate** (< 30 minutes)
   - Apply immediate fix if known
   - Rollback recent deployment if needed
   - Scale resources if capacity issue
   - Failover to backup systems

4. **Communicate** (ongoing)
   - Update status page
   - Post updates every 30 minutes
   - Notify stakeholders

5. **Resolve**
   - Confirm metrics returned to normal
   - Run validation tests
   - Update status page
   - Schedule post-mortem

### 8.2 Runbooks

**WarGames Service Down:**
```bash
# 1. Check if service is actually down
kubectl get pods -n wargames-production

# 2. Check recent logs
kubectl logs -n wargames-production deployment/wargames-api --tail=100

# 3. Check for recent deployments
kubectl rollout history deployment/wargames-api -n wargames-production

# 4. Rollback if needed
kubectl rollout undo deployment/wargames-api -n wargames-production

# 5. Scale up if capacity issue
kubectl scale deployment/wargames-api --replicas=5 -n wargames-production
```

**High Claude API Failure Rate:**
```bash
# 1. Check Claude API status
curl https://status.anthropic.com/api/v2/status.json

# 2. Review failed requests
kubectl logs -n wargames-production deployment/wargames-workers | grep "Claude API error"

# 3. Check rate limiting
# Review metrics: wargames_claude_requests_total

# 4. Implement backoff if rate limited
# Reduce assessment frequency temporarily

# 5. Switch to fallback model if needed
# Update config: claude.model = "claude-3-opus-20240229"
```

### 8.3 Escalation Matrix

```
â"Œâ"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"
â"‚ Time   â"‚ Action                                   â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ 0 min  â"‚ Page primary on-call engineer            â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ 15 min â"‚ Page backup on-call if no acknowledge    â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ 30 min â"‚ Escalate to engineering manager          â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ 60 min â"‚ Escalate to VP Engineering               â"‚
â"œâ"€â"€â"€â"€â"€â"€â"€â"€â"¼â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"¤
â"‚ 2 hour â"‚ Notify executive leadership              â"‚
â""â"€â"€â"€â"€â"€â"€â"€â"€â"´â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"€â"˜
```

---

## 9. Performance Monitoring

### 9.1 Performance Benchmarks

```rust
/// Performance benchmark suite
#[cfg(test)]
mod benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn bench_risk_assessment(c: &mut Criterion) {
        c.bench_function("complete_assessment", |b| {
            b.iter(|| {
                // Run complete assessment
                let result = run_assessment(black_box(&test_data));
                result
            });
        });
    }
    
    fn bench_data_collection(c: &mut Criterion) {
        c.bench_function("data_collection", |b| {
            b.iter(|| {
                let result = collect_data_parallel(black_box(&sources));
                result
            });
        });
    }
    
    fn bench_claude_parsing(c: &mut Criterion) {
        c.bench_function("claude_response_parsing", |b| {
            b.iter(|| {
                let result = parse_claude_response(black_box(&response_json));
                result
            });
        });
    }
    
    criterion_group!(benches, 
        bench_risk_assessment,
        bench_data_collection,
        bench_claude_parsing
    );
    criterion_main!(benches);
}
```

### 9.2 Load Testing

```python
# Load test with Locust

from locust import HttpUser, task, between

class WarGamesUser(HttpUser):
    wait_time = between(1, 5)
    
    @task(1)
    def get_current_risk(self):
        self.client.get("/api/v1/risk/current")
    
    @task(2)
    def get_assessments(self):
        self.client.get("/api/v1/assessments?limit=10")
    
    @task(3)
    def trigger_assessment(self):
        self.client.post("/api/v1/assessments", json={
            "trigger_source": "load_test",
            "priority": "normal"
        })
    
    def on_start(self):
        # Login
        response = self.client.post("/api/v1/auth/login", json={
            "email": "test@example.com",
            "password": "test_password"
        })
        self.token = response.json()["data"]["access_token"]
        self.client.headers["Authorization"] = f"Bearer {self.token}"
```

---

## 10. Cost Monitoring

### 10.1 Cost Tracking

```rust
/// Track operational costs
pub struct CostMonitor {
    db: DatabasePool,
}

impl CostMonitor {
    pub async fn calculate_daily_costs(&self) -> Result<CostBreakdown> {
        // Claude API costs
        let claude_cost = self.get_claude_api_costs().await?;
        
        // AWS infrastructure costs
        let aws_cost = self.get_aws_costs().await?;
        
        // Data source API costs
        let data_source_cost = self.get_data_source_costs().await?;
        
        Ok(CostBreakdown {
            claude_api: claude_cost,
            aws_infrastructure: aws_cost,
            data_sources: data_source_cost,
            total: claude_cost + aws_cost + data_source_cost,
        })
    }
    
    pub async fn project_monthly_costs(&self) -> Result<f64> {
        let daily_avg = self.calculate_average_daily_cost(Duration::days(7)).await?;
        Ok(daily_avg * 30.0)
    }
    
    pub async fn check_budget_alerts(&self) -> Result<Vec<BudgetAlert>> {
        let monthly_projection = self.project_monthly_costs().await?;
        let mut alerts = Vec::new();
        
        // Check Claude API budget
        if monthly_projection > 10000.0 {
            alerts.push(BudgetAlert {
                component: "Claude API",
                projected_cost: monthly_projection,
                budget: 10000.0,
                severity: "warning",
            });
        }
        
        Ok(alerts)
    }
}
```

---

## Summary

This comprehensive monitoring and alerting system ensures:

✓ **99.9% uptime** through multi-layered monitoring  
✓ **< 5 minute detection** of critical failures  
✓ **Complete observability** across all components  
✓ **Proactive alerting** before user impact  
✓ **Rapid incident response** with defined runbooks  
✓ **Cost tracking** and budget management  

**Document Version:** 1.0.0  
**Last Updated:** October 2025  
**Maintained By:** WarGames/JOSHUA SRE Team  
**Next Review:** November 2025

*"In nuclear risk assessment, we monitor the monitors."*
