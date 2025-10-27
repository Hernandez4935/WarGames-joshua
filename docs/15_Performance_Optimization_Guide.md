# WarGames/JOSHUA: Performance Optimization Guide
## Comprehensive Performance Engineering Reference
### Version 1.0.0 | October 2025

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Performance Monitoring](#2-performance-monitoring)
3. [Profiling and Benchmarking](#3-profiling-and-benchmarking)
4. [Database Performance](#4-database-performance)
5. [Caching Strategies](#5-caching-strategies)
6. [Parallel Processing](#6-parallel-processing)
7. [Memory Optimization](#7-memory-optimization)
8. [Network Optimization](#8-network-optimization)
9. [Claude API Optimization](#9-claude-api-optimization)
10. [Async/Await Best Practices](#10-asyncawait-best-practices)
11. [Compilation Optimization](#11-compilation-optimization)
12. [Production Performance Patterns](#12-production-performance-patterns)
13. [Performance Testing](#13-performance-testing)
14. [Case Studies](#14-case-studies)
15. [Performance Checklist](#15-performance-checklist)

---

## 1. Introduction

### 1.1 Performance Targets

WarGames/JOSHUA has specific performance requirements to ensure responsive, cost-effective operation:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        PERFORMANCE TARGETS                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Assessment Completion:        < 5 minutes (target: 2-3 minutes)        │
│  Memory Usage:                 < 500MB (target: 300-400MB)              │
│  Database Query (P95):         < 100ms                                  │
│  API Response Caching:         > 80% hit rate                           │
│  Concurrent Assessments:       10+ simultaneous                         │
│  System Uptime:                99.9% (< 43 minutes downtime/month)      │
│  Claude API Cost/Assessment:   < $1.00 (target: $0.50-$0.75)            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 1.2 When to Optimize

**Optimization priorities:**

```
1. Correctness FIRST     ←  Accurate risk assessment is paramount
2. Maintainability       ←  Code clarity over micro-optimizations
3. Performance           ←  Optimize after profiling identifies bottlenecks
```

**Donald Knuth's wisdom applies:**
> "Premature optimization is the root of all evil. Yet we should not pass up our opportunities in that critical 3%."

**When to optimize:**
- ✅ After profiling shows a real bottleneck
- ✅ When performance targets are not met
- ✅ For critical paths identified by measurements
- ✅ When cost reduction has clear ROI

**When NOT to optimize:**
- ❌ Before measuring (no data = no optimization)
- ❌ For code executed infrequently
- ❌ At the expense of code clarity
- ❌ For marginal gains with high complexity cost

### 1.3 Profiling-First Approach

**Always follow this workflow:**

```
1. Measure    →  Identify actual bottlenecks with profiling
2. Analyze    →  Understand why the bottleneck exists
3. Optimize   →  Apply targeted optimizations
4. Verify     →  Measure again to confirm improvement
5. Document   →  Explain optimization for future maintainers
```

---

## 2. Performance Monitoring

### 2.1 Metrics to Track

**Application Metrics:**

```rust
// src/utils/metrics.rs
use prometheus::{Counter, Histogram, Gauge, Registry};
use std::time::Instant;

lazy_static! {
    // Assessment metrics
    pub static ref ASSESSMENT_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("assessment_duration_seconds", "Assessment completion time")
            .buckets(vec![30.0, 60.0, 120.0, 180.0, 300.0, 600.0])
    ).unwrap();

    pub static ref ASSESSMENT_TOTAL: Counter = Counter::new(
        "assessments_total",
        "Total number of assessments completed"
    ).unwrap();

    pub static ref ASSESSMENT_ERRORS: Counter = Counter::new(
        "assessment_errors_total",
        "Total number of failed assessments"
    ).unwrap();

    // Data collection metrics
    pub static ref DATA_POINTS_COLLECTED: Counter = Counter::new(
        "data_points_collected_total",
        "Total data points collected"
    ).unwrap();

    pub static ref COLLECTION_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("collection_duration_seconds", "Data collection time")
            .buckets(vec![5.0, 10.0, 30.0, 60.0, 120.0])
    ).unwrap();

    // Claude API metrics
    pub static ref CLAUDE_API_CALLS: Counter = Counter::new(
        "claude_api_calls_total",
        "Total Claude API calls"
    ).unwrap();

    pub static ref CLAUDE_API_COST: Counter = Counter::new(
        "claude_api_cost_usd",
        "Total Claude API cost in USD"
    ).unwrap();

    pub static ref CLAUDE_TOKENS_USED: Counter = Counter::new(
        "claude_tokens_used_total",
        "Total tokens consumed"
    ).unwrap();

    // Database metrics
    pub static ref DB_QUERY_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("db_query_duration_seconds", "Database query time")
            .buckets(vec![0.001, 0.005, 0.010, 0.050, 0.100, 0.500, 1.0])
    ).unwrap();

    pub static ref DB_POOL_CONNECTIONS: Gauge = Gauge::new(
        "db_pool_connections",
        "Active database connections"
    ).unwrap();

    // Memory metrics
    pub static ref MEMORY_USAGE_BYTES: Gauge = Gauge::new(
        "memory_usage_bytes",
        "Current memory usage in bytes"
    ).unwrap();
}

pub fn record_assessment_duration(duration: f64) {
    ASSESSMENT_DURATION.observe(duration);
}

pub fn record_claude_api_call(tokens: u32, cost: f64) {
    CLAUDE_API_CALLS.inc();
    CLAUDE_TOKENS_USED.inc_by(tokens as f64);
    CLAUDE_API_COST.inc_by(cost);
}
```

**Usage in code:**

```rust
use crate::utils::metrics;

pub async fn run_assessment(&self) -> Result<RiskAssessment> {
    let start = Instant::now();

    let result = self.execute_assessment().await;

    let duration = start.elapsed().as_secs_f64();
    metrics::record_assessment_duration(duration);

    match result {
        Ok(assessment) => {
            metrics::ASSESSMENT_TOTAL.inc();
            Ok(assessment)
        }
        Err(e) => {
            metrics::ASSESSMENT_ERRORS.inc();
            Err(e)
        }
    }
}
```

### 2.2 Baseline Establishment

**Create performance baselines:**

```bash
# Run benchmark suite and save baseline
cargo bench --bench assessment_flow -- --save-baseline main

# After optimization, compare
cargo bench --bench assessment_flow -- --baseline main
```

**Track baselines in version control:**

```bash
# Store benchmark results
mkdir -p benchmarks/baselines
cargo bench -- --save-baseline v1.0.0 2>&1 | tee benchmarks/baselines/v1.0.0.txt
git add benchmarks/baselines/v1.0.0.txt
git commit -m "chore: add v1.0.0 performance baseline"
```

### 2.3 Performance Regression Detection

**CI integration:**

```yaml
# .github/workflows/performance.yml
name: Performance Regression Tests

on:
  pull_request:
    branches: [main]

jobs:
  performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Run benchmarks
        run: |
          cargo bench --bench assessment_flow -- --save-baseline pr

      - name: Compare with main
        run: |
          git fetch origin main:main
          git checkout main
          cargo bench --bench assessment_flow -- --save-baseline main
          git checkout -

      - name: Check for regression
        run: |
          cargo bench --bench assessment_flow -- --baseline main
          # Fail if regression > 10%
```

### 2.4 Production Monitoring

See [Monitoring and Observability](11_Monitoring_and_Observability.md) for complete production monitoring setup including Prometheus, Grafana dashboards, and alerting.

---

## 3. Profiling and Benchmarking

### 3.1 CPU Profiling

**Using `cargo-flamegraph`:**

```bash
# Install flamegraph
cargo install flamegraph

# Profile specific command
cargo flamegraph --bin joshua -- assess --dry-run

# Open flamegraph.svg
open flamegraph.svg
```

**Interpreting flamegraphs:**

```
┌────────────────────────────────────────────────────────────────────────┐
│                         FLAMEGRAPH ANATOMY                             │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  ████████████████████████ main (100%)                                  │
│  │                                                                     │
│  ├─ ██████████ run_assessment (40%)  ← Wide = CPU intensive           │
│  │  │                                                                  │
│  │  ├─ ████ collect_data (15%)                                         │
│  │  │                                                                  │
│  │  └─ ██████ calculate_risk (25%)  ← Optimization target             │
│  │                                                                     │
│  ├─ ████ database_save (10%)                                           │
│  │                                                                     │
│  └─ ██████ claude_api_call (50%)    ← Largest, but I/O (expected)     │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘

Optimization priorities:
1. calculate_risk (25%) - Pure CPU, good optimization target
2. run_assessment (40%) - Large but includes I/O
3. claude_api_call (50%) - I/O bound, optimize with caching not CPU
```

**Using `perf` (Linux):**

```bash
# Record performance data
perf record --call-graph dwarf target/release/joshua assess

# Analyze
perf report

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > perf-flamegraph.svg
```

### 3.2 Memory Profiling

**Using `heaptrack` (Linux):**

```bash
# Install heaptrack
sudo apt-get install heaptrack heaptrack-gui

# Profile application
heaptrack target/release/joshua assess

# Analyze
heaptrack_gui heaptrack.joshua.*.gz
```

**Using `valgrind --tool=massif`:**

```bash
# Run with massif
valgrind --tool=massif \
         --massif-out-file=massif.out \
         target/release/joshua assess

# Analyze
ms_print massif.out

# Visualize
massif-visualizer massif.out
```

**Identifying memory issues:**

```
Common patterns:
- Memory leaks: Continuously growing heap without plateau
- Excessive allocations: Frequent allocation/deallocation spikes
- Large objects: Single allocations consuming significant memory
```

### 3.3 Benchmarking with Criterion

**Comprehensive benchmark example:**

```rust
// benches/risk_calculation.rs
use criterion::{
    black_box, criterion_group, criterion_main,
    BenchmarkId, Criterion, Throughput,
};
use wargames_joshua::{
    engines::risk_calculation::*,
    models::{RiskFactor, CategoryWeights},
};

fn create_test_factors(count: usize) -> Vec<RiskFactor> {
    (0..count)
        .map(|i| RiskFactor {
            category: RiskCategory::RegionalConflicts,
            name: format!("Factor {}", i),
            value: 0.5,
            confidence: ConfidenceLevel::High,
            // ... other fields
        })
        .collect()
}

fn benchmark_risk_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("risk_calculation");

    // Benchmark with different input sizes
    for count in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*count as u64));

        group.bench_with_input(
            BenchmarkId::new("weighted_score", count),
            count,
            |b, &count| {
                let factors = create_test_factors(count);
                let weights = CategoryWeights::default();

                b.iter(|| {
                    calculate_weighted_score(
                        black_box(&factors),
                        black_box(&weights)
                    )
                });
            },
        );
    }

    group.finish();
}

fn benchmark_bayesian_adjustment(c: &mut Criterion) {
    let factors = create_test_factors(50);
    let historical = create_historical_data();

    c.bench_function("bayesian_adjustment", |b| {
        b.iter(|| {
            apply_bayesian_adjustment(
                black_box(&factors),
                black_box(&historical)
            )
        });
    });
}

fn benchmark_monte_carlo_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("monte_carlo");
    group.sample_size(10); // Reduce sample size for long-running benchmarks

    for iterations in [1_000, 10_000, 50_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("simulation", iterations),
            iterations,
            |b, &iterations| {
                let factors = create_test_factors(50);

                b.iter(|| {
                    run_monte_carlo_simulation(
                        black_box(&factors),
                        black_box(iterations)
                    )
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_risk_calculation,
    benchmark_bayesian_adjustment,
    benchmark_monte_carlo_simulation
);
criterion_main!(benches);
```

**Running benchmarks:**

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench risk_calculation

# Save baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main

# Generate detailed report
cargo bench -- --verbose
```

**Example output:**

```
risk_calculation/weighted_score/10
                        time:   [12.345 us 12.456 us 12.567 us]
                        change: [-5.2341% -3.1234% -1.0123%] (p = 0.00 < 0.05)
                        Performance has improved.

monte_carlo/simulation/10000
                        time:   [45.123 ms 45.678 ms 46.234 ms]
                        thrpt:  [216.45 elem/s 218.92 elem/s 221.41 elem/s]
```

---

## 4. Database Performance

### 4.1 Query Optimization

**Use `EXPLAIN ANALYZE` to understand query plans:**

```sql
-- Identify slow queries
EXPLAIN ANALYZE
SELECT
    a.*,
    COUNT(rf.id) as factor_count
FROM assessments a
LEFT JOIN risk_factors rf ON rf.assessment_id = a.id
WHERE a.assessment_date >= NOW() - INTERVAL '30 days'
GROUP BY a.id
ORDER BY a.assessment_date DESC
LIMIT 10;
```

**Output analysis:**

```
Limit  (cost=125.43..125.45 rows=10 width=256) (actual time=2.345..2.356 rows=10 loops=1)
  ->  Sort  (cost=125.43..126.93 rows=600 width=256) (actual time=2.344..2.350 rows=10 loops=1)
        Sort Key: a.assessment_date DESC
        Sort Method: quicksort  Memory: 27kB
        ->  GroupAggregate  (cost=105.23..120.45 rows=600 width=256) (actual time=1.234..2.123 rows=30 loops=1)
              Group Key: a.id
              ->  Hash Join  (cost=12.34..98.76 rows=2400 width=48) (actual time=0.123..1.456 rows=450 loops=1)
                    Hash Cond: (rf.assessment_id = a.id)
                    ->  Seq Scan on risk_factors rf  (cost=0.00..78.00 rows=2400 width=16) (actual time=0.012..0.789 rows=450 loops=1)
                    ->  Hash  (cost=10.00..10.00 rows=187 width=48) (actual time=0.098..0.099 rows=30 loops=1)
                          Buckets: 1024  Batches: 1  Memory Usage: 12kB
                          ->  Index Scan using idx_assessments_date_desc on assessments a  (cost=0.28..10.00 rows=187 width=48) (actual time=0.015..0.067 rows=30 loops=1)
                                Index Cond: (assessment_date >= (now() - '30 days'::interval))
Planning Time: 0.234 ms
Execution Time: 2.389 ms
```

**Optimization strategies:**

```sql
-- 1. Add appropriate indexes
CREATE INDEX idx_risk_factors_assessment_id ON risk_factors(assessment_id);
CREATE INDEX idx_assessments_date_desc ON assessments(assessment_date DESC);

-- 2. Use composite indexes for common queries
CREATE INDEX idx_assessments_date_confidence
    ON assessments(assessment_date DESC, overall_confidence);

-- 3. Partial indexes for frequently filtered data
CREATE INDEX idx_recent_assessments
    ON assessments(assessment_date DESC)
    WHERE assessment_date >= NOW() - INTERVAL '90 days';

-- 4. Covering indexes (include frequently accessed columns)
CREATE INDEX idx_assessments_covering
    ON assessments(assessment_date DESC)
    INCLUDE (seconds_to_midnight, overall_confidence, trend_direction);
```

### 4.2 Connection Pooling

**Optimize SQLx pool configuration:**

```rust
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use std::time::Duration;

pub async fn create_optimized_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let connect_options = config.connection_string
        .parse::<PgConnectOptions>()?
        .application_name("wargames-joshua")
        .statement_cache_capacity(100);  // Cache prepared statements

    let pool = PgPoolOptions::new()
        .max_connections(20)           // Maximum connections in pool
        .min_connections(5)            // Minimum idle connections
        .max_lifetime(Duration::from_secs(30 * 60))  // 30 minutes
        .idle_timeout(Duration::from_secs(10 * 60))  // 10 minutes
        .acquire_timeout(Duration::from_secs(30))    // 30 seconds
        .test_before_acquire(true)     // Health check before use
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}
```

**Monitor connection pool:**

```rust
use sqlx::Pool;

pub async fn monitor_pool_health(pool: &PgPool) {
    loop {
        let size = pool.size();
        let idle = pool.num_idle();
        let active = size - idle;

        tracing::info!(
            pool_size = size,
            active_connections = active,
            idle_connections = idle,
            "Database connection pool status"
        );

        // Alert if pool is exhausted
        if idle == 0 && size >= pool.options().get_max_connections() {
            tracing::warn!("Database connection pool exhausted!");
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
```

### 4.3 Batch Operations

**Efficient batch inserts:**

```rust
// Inefficient: Individual inserts
for factor in risk_factors {
    sqlx::query!(
        "INSERT INTO risk_factors (...) VALUES (...)",
        // ... parameters
    )
    .execute(&pool)
    .await?;
}

// Efficient: Batch insert with transaction
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

// Most efficient: Use UNNEST for bulk insert
sqlx::query!(
    r#"
    INSERT INTO risk_factors (
        assessment_id, factor_category, factor_name, raw_value
    )
    SELECT * FROM UNNEST(
        $1::uuid[], $2::text[], $3::text[], $4::decimal[]
    )
    "#,
    &assessment_ids[..],
    &categories[..],
    &names[..],
    &values[..]
)
.execute(&pool)
.await?;
```

### 4.4 Query Result Caching

**Cache frequently accessed queries:**

```rust
use moka::future::Cache;
use std::sync::Arc;

pub struct CachedDatabaseEngine {
    pool: PgPool,
    assessment_cache: Cache<Uuid, Arc<RiskAssessment>>,
}

impl CachedDatabaseEngine {
    pub fn new(pool: PgPool) -> Self {
        let assessment_cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(3600))  // 1 hour TTL
            .build();

        Self {
            pool,
            assessment_cache,
        }
    }

    pub async fn get_assessment(&self, id: Uuid) -> Result<Arc<RiskAssessment>> {
        // Try cache first
        if let Some(cached) = self.assessment_cache.get(&id) {
            tracing::debug!(assessment_id = %id, "Assessment cache hit");
            return Ok(cached);
        }

        // Cache miss - fetch from database
        tracing::debug!(assessment_id = %id, "Assessment cache miss");
        let assessment = self.fetch_assessment_from_db(id).await?;
        let assessment_arc = Arc::new(assessment);

        // Update cache
        self.assessment_cache.insert(id, assessment_arc.clone()).await;

        Ok(assessment_arc)
    }

    async fn fetch_assessment_from_db(&self, id: Uuid) -> Result<RiskAssessment> {
        // Database fetch implementation
        todo!()
    }
}
```

### 4.5 Indexing Strategies

**Complete indexing schema:**

```sql
-- src/migrations/002_performance_indexes.sql

-- Primary indexes (already created in schema)
-- These are created as part of table definitions

-- Secondary indexes for common query patterns

-- 1. Temporal queries (most common)
CREATE INDEX idx_assessments_date_desc
    ON assessments(assessment_date DESC);

CREATE INDEX idx_assessments_created
    ON assessments(created_at DESC);

-- 2. Risk level filtering
CREATE INDEX idx_assessments_seconds
    ON assessments(seconds_to_midnight);

CREATE INDEX idx_assessments_trend
    ON assessments(trend_direction);

-- 3. Risk factors queries
CREATE INDEX idx_risk_factors_assessment
    ON risk_factors(assessment_id);

CREATE INDEX idx_risk_factors_category
    ON risk_factors(factor_category);

CREATE INDEX idx_risk_factors_contribution
    ON risk_factors(contribution_to_risk DESC);

CREATE INDEX idx_risk_factors_name
    ON risk_factors(factor_name);

-- 4. Composite indexes for multi-column queries
CREATE INDEX idx_risk_factors_assessment_category
    ON risk_factors(assessment_id, factor_category);

CREATE INDEX idx_assessments_date_confidence
    ON assessments(assessment_date DESC, overall_confidence);

-- 5. Partial indexes for hot data
CREATE INDEX idx_recent_assessments
    ON assessments(assessment_date DESC)
    WHERE assessment_date >= NOW() - INTERVAL '90 days';

-- 6. Covering indexes (include frequently accessed columns)
CREATE INDEX idx_assessments_covering
    ON assessments(assessment_date DESC)
    INCLUDE (seconds_to_midnight, overall_confidence, trend_direction, raw_risk_score);

-- 7. Full-text search indexes
CREATE INDEX idx_assessments_summary_fts
    ON assessments USING gin(to_tsvector('english', executive_summary));

CREATE INDEX idx_risk_factors_evidence_fts
    ON risk_factors USING gin(to_tsvector('english', evidence_summary));

-- Index maintenance
ANALYZE assessments;
ANALYZE risk_factors;
```

**Monitor index usage:**

```sql
-- Check index usage statistics
SELECT
    schemaname,
    tablename,
    indexname,
    idx_scan,
    idx_tup_read,
    idx_tup_fetch
FROM pg_stat_user_indexes
WHERE schemaname = 'public'
ORDER BY idx_scan ASC;

-- Identify unused indexes
SELECT
    schemaname,
    tablename,
    indexname,
    idx_scan
FROM pg_stat_user_indexes
WHERE schemaname = 'public'
AND idx_scan = 0
AND indexname NOT LIKE '%_pkey';

-- Index size
SELECT
    schemaname,
    tablename,
    indexname,
    pg_size_pretty(pg_relation_size(indexrelid)) AS index_size
FROM pg_stat_user_indexes
WHERE schemaname = 'public'
ORDER BY pg_relation_size(indexrelid) DESC;
```

---

## 5. Caching Strategies

### 5.1 Multi-Level Caching

**Architecture:**

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      MULTI-LEVEL CACHE HIERARCHY                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  L1: In-Memory Cache (moka)                                            │
│      ├─ Latency: < 1ms                                                 │
│      ├─ Capacity: 100MB                                                │
│      ├─ TTL: 1 hour                                                    │
│      └─ Use: Hot data, frequently accessed                             │
│                          │                                              │
│                          ▼ (miss)                                       │
│                                                                         │
│  L2: Redis Cache                                                       │
│      ├─ Latency: 1-5ms                                                 │
│      ├─ Capacity: 1GB                                                  │
│      ├─ TTL: 6 hours                                                   │
│      └─ Use: Shared across instances                                   │
│                          │                                              │
│                          ▼ (miss)                                       │
│                                                                         │
│  L3: Database                                                          │
│      ├─ Latency: 10-100ms                                              │
│      ├─ Capacity: Unlimited                                            │
│      └─ Use: Persistent storage                                        │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Implementation:**

```rust
use moka::future::Cache as MokaCache;
use redis::AsyncCommands;
use std::sync::Arc;

pub struct MultiLevelCache {
    // L1: In-memory cache
    l1_cache: MokaCache<String, Arc<Vec<u8>>>,

    // L2: Redis cache (optional)
    l2_cache: Option<redis::aio::ConnectionManager>,

    // L3: Database
    database: Arc<dyn DatabaseEngine>,
}

impl MultiLevelCache {
    pub fn new(
        database: Arc<dyn DatabaseEngine>,
        redis_url: Option<String>,
    ) -> Result<Self> {
        // L1 cache configuration
        let l1_cache = MokaCache::builder()
            .max_capacity(100_000)  // 100k entries
            .time_to_live(Duration::from_secs(3600))  // 1 hour
            .time_to_idle(Duration::from_secs(600))   // 10 minutes
            .build();

        // L2 cache configuration (if Redis available)
        let l2_cache = if let Some(url) = redis_url {
            let client = redis::Client::open(url)?;
            let manager = client.get_tokio_connection_manager().await?;
            Some(manager)
        } else {
            None
        };

        Ok(Self {
            l1_cache,
            l2_cache,
            database,
        })
    }

    pub async fn get_assessment(&self, id: Uuid) -> Result<Arc<RiskAssessment>> {
        let key = format!("assessment:{}", id);

        // Try L1 cache
        if let Some(cached) = self.l1_cache.get(&key) {
            tracing::debug!(cache_level = "L1", key = %key, "Cache hit");
            return deserialize_assessment(&cached);
        }

        // Try L2 cache (Redis)
        if let Some(redis) = &self.l2_cache {
            if let Ok(Some(data)) = redis.clone().get::<_, Option<Vec<u8>>>(&key).await {
                tracing::debug!(cache_level = "L2", key = %key, "Cache hit");

                let assessment = deserialize_assessment(&data)?;

                // Populate L1 cache
                self.l1_cache.insert(key.clone(), Arc::new(data)).await;

                return Ok(assessment);
            }
        }

        // L3: Fetch from database
        tracing::debug!(cache_level = "L3", key = %key, "Fetching from database");
        let assessment = self.database.get_assessment(id).await?;
        let assessment_arc = Arc::new(assessment);

        // Serialize for caching
        let serialized = Arc::new(serialize_assessment(&assessment_arc)?);

        // Populate L2 cache
        if let Some(redis) = &self.l2_cache {
            let _: () = redis.clone()
                .set_ex(&key, &*serialized, 21600)  // 6 hours
                .await
                .unwrap_or(());
        }

        // Populate L1 cache
        self.l1_cache.insert(key, serialized).await;

        Ok(assessment_arc)
    }

    pub async fn invalidate(&self, id: Uuid) {
        let key = format!("assessment:{}", id);

        // Invalidate L1
        self.l1_cache.invalidate(&key).await;

        // Invalidate L2
        if let Some(redis) = &self.l2_cache {
            let _: () = redis.clone().del(&key).await.unwrap_or(());
        }
    }
}
```

### 5.2 Cache Invalidation

**Strategies:**

```rust
// 1. Time-based expiration (TTL)
cache.insert_with_ttl(key, value, Duration::from_secs(3600));

// 2. Event-based invalidation
pub async fn store_assessment(&self, assessment: &RiskAssessment) -> Result<Uuid> {
    let id = self.database.store_assessment(assessment).await?;

    // Invalidate caches
    self.cache.invalidate_assessment(id).await;
    self.cache.invalidate_latest_assessment().await;

    Ok(id)
}

// 3. Cache stampede prevention (only one task fetches)
pub async fn get_with_lock(&self, key: &str) -> Result<Value> {
    if let Some(cached) = self.cache.get(key) {
        return Ok(cached);
    }

    // Acquire lock to prevent multiple fetches
    let _lock = self.fetch_locks.lock(key).await;

    // Check again after acquiring lock (another task may have populated)
    if let Some(cached) = self.cache.get(key) {
        return Ok(cached);
    }

    // Fetch and cache
    let value = self.fetch_from_source(key).await?;
    self.cache.insert(key, value.clone()).await;

    Ok(value)
}
```

### 5.3 Cache Warming

**Pre-populate caches on startup:**

```rust
pub async fn warm_caches(&self) -> Result<()> {
    tracing::info!("Starting cache warming");

    // Warm assessment cache with recent assessments
    let recent_assessments = self.database
        .get_assessment_history(
            Utc::now() - Duration::days(7),
            Utc::now(),
        )
        .await?;

    for assessment in recent_assessments {
        let serialized = serialize_assessment(&assessment)?;
        let key = format!("assessment:{}", assessment.id);

        self.l1_cache.insert(key.clone(), Arc::new(serialized)).await;

        if let Some(redis) = &self.l2_cache {
            let _: () = redis.clone()
                .set_ex(&key, &serialized, 21600)
                .await
                .unwrap_or(());
        }
    }

    tracing::info!(
        cached_assessments = recent_assessments.len(),
        "Cache warming complete"
    );

    Ok(())
}
```

---

## 6. Parallel Processing

### 6.1 Tokio Runtime Optimization

**Configure runtime for workload:**

```rust
// main.rs
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    // Application code
}

// Or custom runtime
use tokio::runtime::Runtime;

fn create_optimized_runtime() -> Runtime {
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .builder()
        .worker_threads(4)          // 4 worker threads
        .max_blocking_threads(512)  // For blocking operations
        .thread_name("joshua-worker")
        .thread_stack_size(3 * 1024 * 1024)  // 3MB stack
        .build()
        .expect("Failed to build runtime")
}
```

### 6.2 Concurrent Data Collection

**Parallel collector execution:**

```rust
use tokio::task::JoinSet;
use std::sync::Arc;

pub async fn collect_all_parallel(&self) -> Result<AggregatedData> {
    let mut join_set = JoinSet::new();

    // Spawn tasks for each collector
    for collector in &self.collectors {
        let collector = collector.clone();
        let rate_limiter = self.rate_limiters.get(collector.source_name()).cloned();

        join_set.spawn(async move {
            // Apply rate limiting
            if let Some(limiter) = rate_limiter {
                limiter.check_and_wait().await?;
            }

            // Collect with retries
            let mut attempts = 0;
            loop {
                attempts += 1;
                match collector.collect().await {
                    Ok(data) => return Ok((collector.source_name().to_string(), data)),
                    Err(e) if attempts < 3 => {
                        tokio::time::sleep(Duration::from_secs(2u64.pow(attempts))).await;
                    }
                    Err(e) => return Err(e),
                }
            }
        });
    }

    // Collect results
    let mut all_data = Vec::new();
    let mut failures = Vec::new();

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok((source, data))) => {
                tracing::info!(source = %source, count = data.len(), "Collection succeeded");
                all_data.extend(data);
            }
            Ok(Err(e)) => {
                tracing::warn!(error = %e, "Collection failed");
                failures.push(e.to_string());
            }
            Err(join_err) => {
                tracing::error!(error = %join_err, "Task panicked");
            }
        }
    }

    Ok(AggregatedData {
        data_points: all_data,
        collection_timestamp: Utc::now(),
        sources_count: self.collectors.len() - failures.len(),
        failed_sources: failures,
        collection_duration: start.elapsed(),
    })
}
```

### 6.3 Rayon for CPU-Bound Work

**Parallel risk factor calculation:**

```rust
use rayon::prelude::*;

pub fn calculate_contributions_parallel(
    &self,
    factors: &[RiskFactor],
) -> Vec<f64> {
    factors
        .par_iter()  // Parallel iterator
        .map(|factor| self.calculate_single_contribution(factor))
        .collect()
}

// Configure Rayon thread pool
use rayon::ThreadPoolBuilder;

fn create_rayon_pool() -> Result<rayon::ThreadPool> {
    ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("joshua-rayon-{}", i))
        .build()
        .map_err(|e| Error::Configuration(format!("Rayon pool: {}", e)))
}
```

### 6.4 Semaphore-Based Concurrency Control

**Limit concurrent operations:**

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimitedCollector {
    collector: Box<dyn DataCollector>,
    semaphore: Arc<Semaphore>,
}

impl RateLimitedCollector {
    pub fn new(
        collector: Box<dyn DataCollector>,
        max_concurrent: usize,
    ) -> Self {
        Self {
            collector,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn collect(&self) -> Result<Vec<DataPoint>> {
        // Acquire permit (blocks if at limit)
        let _permit = self.semaphore.acquire().await
            .map_err(|_| Error::Collection {
                collector: self.collector.source_name().to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Semaphore closed"
                )),
            })?;

        // Perform collection while holding permit
        self.collector.collect().await

        // Permit released automatically when dropped
    }
}
```

---

## 7. Memory Optimization

### 7.1 Allocation Reduction

**Use `Cow` to avoid unnecessary allocations:**

```rust
use std::borrow::Cow;

// Instead of always cloning
fn process_data_clone(data: String) -> String {
    if needs_transformation(&data) {
        transform(data)
    } else {
        data  // Unnecessary clone if passed as &str
    }
}

// Use Cow for zero-copy when possible
fn process_data_cow(data: Cow<str>) -> Cow<str> {
    if needs_transformation(&data) {
        Cow::Owned(transform(data.into_owned()))
    } else {
        data  // No allocation if already borrowed
    }
}
```

**Pre-allocate collections:**

```rust
// Inefficient: Repeated reallocations
let mut factors = Vec::new();
for item in items {
    factors.push(process(item));
}

// Efficient: Pre-allocate with known capacity
let mut factors = Vec::with_capacity(items.len());
for item in items {
    factors.push(process(item));
}

// Even better: Use iterator
let factors: Vec<_> = items.iter().map(process).collect();
```

### 7.2 Arc for Shared Ownership

**Share data across threads without copying:**

```rust
use std::sync::Arc;

// Inefficient: Clone large data for each task
for collector in collectors {
    let data = large_dataset.clone();  // Expensive!
    tokio::spawn(async move {
        collector.process(data).await
    });
}

// Efficient: Share with Arc
let data = Arc::new(large_dataset);
for collector in collectors {
    let data = data.clone();  // Cheap reference count increment
    tokio::spawn(async move {
        collector.process(&data).await
    });
}
```

### 7.3 Memory Pooling

**Reuse buffers:**

```rust
use bytes::{BytesMut, BufMut};

pub struct BufferPool {
    pool: deadpool::managed::Pool<BytesMut>,
}

impl BufferPool {
    pub fn new(size: usize, capacity: usize) -> Self {
        // Create pool of pre-allocated buffers
        let pool = deadpool::managed::Pool::builder(
            BufferManager { capacity }
        )
        .max_size(size)
        .build()
        .unwrap();

        Self { pool }
    }

    pub async fn get(&self) -> Result<PooledBuffer> {
        let buffer = self.pool.get().await?;
        Ok(PooledBuffer { buffer })
    }
}

pub struct PooledBuffer {
    buffer: deadpool::managed::Object<BytesMut>,
}

impl PooledBuffer {
    pub fn write(&mut self, data: &[u8]) {
        self.buffer.put_slice(data);
    }

    // Buffer automatically returned to pool when dropped
}
```

### 7.4 String Interning

**Deduplicate repeated strings:**

```rust
use string_interner::{StringInterner, Symbol};

pub struct StringCache {
    interner: StringInterner,
}

impl StringCache {
    pub fn new() -> Self {
        Self {
            interner: StringInterner::default(),
        }
    }

    pub fn intern(&mut self, s: &str) -> Symbol {
        self.interner.get_or_intern(s)
    }

    pub fn resolve(&self, symbol: Symbol) -> &str {
        self.interner.resolve(symbol).unwrap()
    }
}

// Usage
let mut cache = StringCache::new();

// These all point to the same underlying string
let sym1 = cache.intern("Reuters");
let sym2 = cache.intern("Reuters");
let sym3 = cache.intern("Reuters");

assert_eq!(sym1, sym2);
assert_eq!(sym2, sym3);
// Only one allocation for "Reuters"
```

---

## 8. Network Optimization

### 8.1 HTTP Client Configuration

**Optimize reqwest client:**

```rust
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub fn create_optimized_http_client() -> Result<Client> {
    ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(10)     // Connection pooling
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_keepalive(Duration::from_secs(60))
        .http2_adaptive_window(true)    // HTTP/2 flow control
        .http2_keep_alive_interval(Duration::from_secs(30))
        .user_agent("WarGames-JOSHUA/1.0")
        .gzip(true)                     // Automatic compression
        .build()
        .map_err(|e| Error::Configuration(format!("HTTP client: {}", e)))
}
```

### 8.2 Request Batching

**Batch API requests:**

```rust
pub async fn fetch_articles_batch(
    &self,
    article_ids: &[String],
) -> Result<Vec<Article>> {
    // Batch into groups of 50
    let batches: Vec<_> = article_ids
        .chunks(50)
        .collect();

    let mut all_articles = Vec::with_capacity(article_ids.len());

    for batch in batches {
        let response = self.http_client
            .post(&self.config.batch_endpoint)
            .json(&json!({
                "ids": batch
            }))
            .send()
            .await?;

        let articles: Vec<Article> = response.json().await?;
        all_articles.extend(articles);

        // Rate limiting between batches
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(all_articles)
}
```

### 8.3 Connection Reuse

**HTTP connection pooling is automatic with reqwest, but ensure:**

```rust
// Good: Reuse single client instance
pub struct DataCollector {
    http_client: Client,  // Reused across requests
}

// Bad: Create new client per request
async fn fetch_data() -> Result<Data> {
    let client = Client::new();  // New connection pool each time!
    client.get(url).send().await
}
```

---

## 9. Claude API Optimization

### 9.1 Cost Optimization

**Token usage reduction:**

```rust
pub fn build_optimized_prompt(
    &self,
    data: &AggregatedData,
) -> Result<String> {
    // Summarize data instead of including full text
    let summary = self.summarize_data_points(data, max_chars = 5000)?;

    // Use structured format
    let prompt = format!(
        r#"Analyze this nuclear risk data (summarized for efficiency):

Date Range: {} to {}
Data Points: {} from {} sources

Summary by Category:
{}

Provide risk assessment in JSON format."#,
        data.start_date,
        data.end_date,
        data.data_points.len(),
        data.sources_count,
        summary,
    );

    Ok(prompt)
}

fn summarize_data_points(&self, data: &AggregatedData, max_chars: usize) -> Result<String> {
    let mut summary = String::with_capacity(max_chars);

    for category in DataCategory::all() {
        let category_data = data.filter_by_category(category);

        if !category_data.is_empty() {
            summary.push_str(&format!(
                "\n{:?} ({} items):\n",
                category,
                category_data.len()
            ));

            // Include only most significant items
            for item in category_data.iter().take(5) {
                let snippet = item.content.chars().take(200).collect::<String>();
                summary.push_str(&format!("  - {}\n", snippet));
            }
        }

        if summary.len() > max_chars {
            summary.truncate(max_chars);
            summary.push_str("...[truncated]");
            break;
        }
    }

    Ok(summary)
}
```

**Model selection:**

```rust
pub enum ClaudeModel {
    Haiku,    // Fastest, cheapest - $0.25/$1.25 per million tokens
    Sonnet,   // Balanced - $3/$15 per million tokens
    Opus,     // Best quality - $15/$75 per million tokens
}

pub fn select_model_for_task(task: &AnalysisTask) -> ClaudeModel {
    match task.complexity {
        Complexity::Low => ClaudeModel::Haiku,    // Simple categorization
        Complexity::Medium => ClaudeModel::Sonnet, // Standard assessment
        Complexity::High => ClaudeModel::Opus,     // Complex analysis
    }
}
```

**Cost tracking:**

```rust
pub struct CostTracker {
    total_tokens: AtomicU64,
    total_cost_cents: AtomicU64,
}

impl CostTracker {
    pub fn record_api_call(&self, tokens_used: u32, model: ClaudeModel) {
        let cost_per_token = match model {
            ClaudeModel::Haiku => 0.00000025,   // $0.25 per million input tokens
            ClaudeModel::Sonnet => 0.000003,    // $3 per million
            ClaudeModel::Opus => 0.000015,      // $15 per million
        };

        let cost_cents = (tokens_used as f64 * cost_per_token * 100.0) as u64;

        self.total_tokens.fetch_add(tokens_used as u64, Ordering::Relaxed);
        self.total_cost_cents.fetch_add(cost_cents, Ordering::Relaxed);

        // Alert if approaching budget
        let total_cost = self.total_cost_cents.load(Ordering::Relaxed) as f64 / 100.0;
        if total_cost > self.config.monthly_budget * 0.8 {
            tracing::warn!(
                cost_usd = total_cost,
                budget = self.config.monthly_budget,
                "Approaching Claude API budget limit"
            );
        }
    }
}
```

### 9.2 Response Caching

**Cache Claude API responses:**

```rust
use sha2::{Sha256, Digest};

pub async fn call_claude_with_cache(
    &self,
    prompt: &str,
) -> Result<String> {
    // Generate cache key from prompt hash
    let mut hasher = Sha256::new();
    hasher.update(prompt.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let cache_key = format!("claude:response:{}", hash);

    // Check cache
    if let Some(redis) = &self.redis_cache {
        if let Ok(Some(cached_response)) = redis.clone().get(&cache_key).await {
            tracing::info!("Claude API response cache hit");
            return Ok(cached_response);
        }
    }

    // Cache miss - call API
    tracing::info!("Claude API response cache miss");
    let response = self.call_claude_api(prompt).await?;

    // Cache response (6 hour TTL)
    if let Some(redis) = &self.redis_cache {
        let _: () = redis.clone()
            .set_ex(&cache_key, &response, 21600)
            .await
            .unwrap_or(());
    }

    Ok(response)
}
```

### 9.3 Streaming Responses

**Use streaming for large responses:**

```rust
use futures::StreamExt;

pub async fn call_claude_streaming(&self, prompt: &str) -> Result<String> {
    let request = json!({
        "model": self.config.model,
        "max_tokens": self.config.max_tokens,
        "stream": true,  // Enable streaming
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
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut full_response = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = std::str::from_utf8(&chunk)?;

        // Parse SSE format
        for line in text.lines() {
            if line.starts_with("data: ") {
                let data = &line[6..];
                if let Ok(event) = serde_json::from_str::<StreamEvent>(data) {
                    if let Some(content) = event.delta.text {
                        full_response.push_str(&content);

                        // Optional: Stream to user for progress indication
                        print!("{}", content);
                        std::io::stdout().flush()?;
                    }
                }
            }
        }
    }

    Ok(full_response)
}
```

For complete Claude API optimization details, see [Claude Integration Specifications](10_Claude_Integration_Specifications.md).

---

## 10. Async/Await Best Practices

### 10.1 Avoiding Blocking

**NEVER block async runtime:**

```rust
// BAD: Blocking call in async context
async fn bad_example() {
    let data = std::fs::read_to_string("file.txt").unwrap();  // BLOCKS!
}

// GOOD: Use async I/O
async fn good_example() -> Result<String> {
    let data = tokio::fs::read_to_string("file.txt").await?;
    Ok(data)
}

// If you MUST use blocking code, use spawn_blocking
async fn acceptable_example() -> Result<String> {
    tokio::task::spawn_blocking(|| {
        std::fs::read_to_string("file.txt")
    })
    .await
    .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
    .map_err(Into::into)
}
```

### 10.2 Select and Join

**Concurrent operations:**

```rust
use tokio::try_join;

// Run operations concurrently, fail if any fails
async fn concurrent_operations() -> Result<(Data1, Data2, Data3)> {
    try_join!(
        fetch_data_source_1(),
        fetch_data_source_2(),
        fetch_data_source_3(),
    )
}

// Select first completed
use tokio::select;

async fn first_successful() -> Result<Data> {
    select! {
        result = fetch_from_primary() => result,
        result = fetch_from_backup() => result,
        _ = tokio::time::sleep(Duration::from_secs(30)) => {
            Err(Error::Timeout)
        }
    }
}
```

### 10.3 Cancellation Safety

**Ensure proper cleanup on cancellation:**

```rust
async fn cancellation_safe_operation() -> Result<()> {
    // Acquire resources
    let lock = self.mutex.lock().await;
    let connection = self.pool.acquire().await?;

    // Operation that might be cancelled
    let result = risky_operation(&connection).await;

    // Cleanup happens automatically (Drop trait)
    // But for explicit cleanup:
    drop(connection);
    drop(lock);

    result
}

// For complex cleanup, use scopeguard
use scopeguard::defer;

async fn complex_cleanup_operation() -> Result<()> {
    let resource = acquire_resource().await?;

    defer! {
        // This runs even if function is cancelled
        cleanup_resource(resource);
    }

    // Do work
    process(resource).await?;

    Ok(())
}
```

---

## 11. Compilation Optimization

### 11.1 Release Profile

**Optimal release configuration:**

```toml
# Cargo.toml

[profile.release]
opt-level = 3              # Maximum optimization
lto = "fat"                # Link-time optimization
codegen-units = 1          # Better optimization, slower compile
panic = "abort"            # Smaller binary, no unwinding
strip = true               # Remove debug symbols
overflow-checks = false    # Disable integer overflow checks (use with caution)

# Alternative: Optimized for size
[profile.release-small]
inherits = "release"
opt-level = "z"            # Optimize for size
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

### 11.2 Link-Time Optimization (LTO)

**LTO benefits and tradeoffs:**

```
Benefits:
  + 10-30% runtime performance improvement
  + Smaller binary size (10-20% reduction)
  + Better dead code elimination

Tradeoffs:
  - Much longer compilation time (2-5x slower)
  - Higher memory usage during compilation
```

**When to use:**

- ✅ Production releases
- ✅ Performance-critical builds
- ❌ Development/debug builds
- ❌ CI builds (unless time permits)

### 11.3 Profile-Guided Optimization (PGO)

**Advanced: PGO for maximum performance:**

```bash
# Step 1: Build with instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    cargo build --release

# Step 2: Run representative workload
./target/release/joshua assess
./target/release/joshua trends --period 30d
# ... more representative operations

# Step 3: Merge profile data
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# Step 4: Build with PGO
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata -Cllvm-args=-pgo-warn-missing-function" \
    cargo build --release
```

**Expected improvements:**
- 10-20% performance gain
- Better branch prediction
- More optimal inlining

---

## 12. Production Performance Patterns

### 12.1 Load Balancing

**Multiple instance deployment:**

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         LOAD BALANCED SETUP                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                          Load Balancer                                  │
│                          (nginx/ALB)                                    │
│                                 │                                       │
│                 ┌───────────────┼───────────────┐                       │
│                 ▼               ▼               ▼                       │
│           ┌──────────┐    ┌──────────┐    ┌──────────┐                 │
│           │ Instance │    │ Instance │    │ Instance │                 │
│           │    1     │    │    2     │    │    3     │                 │
│           └──────────┘    └──────────┘    └──────────┘                 │
│                 │               │               │                       │
│                 └───────────────┼───────────────┘                       │
│                                 ▼                                       │
│                          ┌──────────┐                                   │
│                          │ Database │                                   │
│                          │(Primary) │                                   │
│                          └──────────┘                                   │
│                                 │                                       │
│                          ┌──────────┐                                   │
│                          │  Redis   │                                   │
│                          │  Cache   │                                   │
│                          └──────────┘                                   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 12.2 Horizontal Scaling

**Stateless design for scaling:**

```rust
// Good: Stateless design
pub struct AssessmentEngine {
    database: Arc<dyn DatabaseEngine>,
    claude_client: Arc<ClaudeClient>,
    // No mutable state
}

// Bad: Stateful design (hard to scale)
pub struct StatefulEngine {
    database: Arc<dyn DatabaseEngine>,
    cache: HashMap<String, CachedData>,  // Instance-local cache
    current_assessment: Option<RiskAssessment>,  // Instance state
}
```

**Shared state via Redis:**

```rust
pub struct DistributedLock {
    redis: redis::aio::ConnectionManager,
}

impl DistributedLock {
    pub async fn acquire(&self, key: &str, ttl: Duration) -> Result<Lock> {
        let lock_key = format!("lock:{}", key);
        let lock_value = Uuid::new_v4().to_string();

        // SET NX EX - only set if not exists, with expiration
        let result: bool = self.redis.clone()
            .set_nx(&lock_key, &lock_value, ttl.as_secs() as usize)
            .await?;

        if result {
            Ok(Lock {
                key: lock_key,
                value: lock_value,
                redis: self.redis.clone(),
            })
        } else {
            Err(Error::LockAcquisition(format!("Failed to acquire lock: {}", key)))
        }
    }
}
```

### 12.3 Resource Limits

**Configure resource constraints:**

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct ResourceLimiter {
    memory_semaphore: Arc<Semaphore>,
    concurrent_assessments: Arc<Semaphore>,
}

impl ResourceLimiter {
    pub fn new(config: &LimiterConfig) -> Self {
        Self {
            memory_semaphore: Arc::new(Semaphore::new(config.max_memory_mb)),
            concurrent_assessments: Arc::new(Semaphore::new(config.max_concurrent)),
        }
    }

    pub async fn check_memory_available(&self, required_mb: usize) -> Result<MemoryPermit> {
        let permit = self.memory_semaphore
            .acquire_many(required_mb as u32)
            .await
            .map_err(|_| Error::ResourceLimit("Memory limit exceeded"))?;

        Ok(MemoryPermit { permit })
    }
}

pub struct MemoryPermit {
    permit: tokio::sync::SemaphorePermit<'static>,
}

// Permit automatically released when dropped
```

---

## 13. Performance Testing

### 13.1 Load Testing

**Using `k6` for HTTP load tests:**

```javascript
// tests/load/assessment_load.js
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    stages: [
        { duration: '2m', target: 10 },   // Ramp up to 10 users
        { duration: '5m', target: 10 },   // Stay at 10 users
        { duration: '2m', target: 50 },   // Ramp up to 50 users
        { duration: '5m', target: 50 },   // Stay at 50 users
        { duration: '2m', target: 0 },    // Ramp down to 0 users
    ],
    thresholds: {
        http_req_duration: ['p(95)<5000'],  // 95% of requests < 5s
        http_req_failed: ['rate<0.01'],     // <1% failures
    },
};

export default function () {
    let response = http.post(
        'http://localhost:8080/v1/assessments',
        null,
        {
            headers: {
                'Authorization': `Bearer ${__ENV.JWT_TOKEN}`,
                'Content-Type': 'application/json',
            },
        }
    );

    check(response, {
        'status is 202': (r) => r.status === 202,
        'response time < 5s': (r) => r.timings.duration < 5000,
    });

    sleep(1);
}
```

**Run load test:**

```bash
k6 run tests/load/assessment_load.js
```

### 13.2 Stress Testing

**Find breaking point:**

```javascript
// tests/load/stress_test.js
export let options = {
    stages: [
        { duration: '1m', target: 100 },
        { duration: '3m', target: 100 },
        { duration: '1m', target: 200 },
        { duration: '3m', target: 200 },
        { duration: '1m', target: 300 },
        { duration: '3m', target: 300 },
        // Continue increasing until failure
    ],
};
```

### 13.3 Sustained Load Testing

**24-hour endurance test:**

```javascript
export let options = {
    duration: '24h',
    vus: 50,
    thresholds: {
        http_req_duration: ['p(95)<5000'],
        http_req_failed: ['rate<0.01'],
    },
};
```

---

## 14. Case Studies

### 14.1 Assessment Time Reduction

**Problem:** Assessments taking 10+ minutes

**Analysis:**
```bash
cargo flamegraph --bin joshua -- assess --dry-run
```

Flamegraph revealed:
- 60% time in data collection
- Serial data fetching
- No caching

**Solution:**

```rust
// Before: Serial collection
for collector in &self.collectors {
    let data = collector.collect().await?;
    all_data.extend(data);
}

// After: Parallel collection with caching
let futures: Vec<_> = self.collectors
    .iter()
    .map(|c| c.collect_cached())  // Added caching
    .collect();

let results = futures::future::join_all(futures).await;
```

**Result:**
- Assessment time: 10 minutes → 3 minutes (70% reduction)
- Cache hit rate: 85%
- Cost savings: $0.50 per assessment (reduced API calls)

### 14.2 Database Query Optimization

**Problem:** Historical queries taking 30+ seconds

**Analysis:**

```sql
EXPLAIN ANALYZE
SELECT * FROM assessments
WHERE assessment_date >= NOW() - INTERVAL '90 days'
ORDER BY assessment_date DESC;

-- Sequential Scan on assessments (cost=0.00..1234.56 rows=5000)
-- Planning Time: 0.123 ms
-- Execution Time: 31,234.567 ms
```

**Solution:**

```sql
-- Add descending index
CREATE INDEX idx_assessments_date_desc
    ON assessments(assessment_date DESC);

-- After optimization
EXPLAIN ANALYZE
SELECT * FROM assessments
WHERE assessment_date >= NOW() - INTERVAL '90 days'
ORDER BY assessment_date DESC;

-- Index Scan using idx_assessments_date_desc (cost=0.28..125.43 rows=5000)
-- Planning Time: 0.089 ms
-- Execution Time: 234.567 ms
```

**Result:**
- Query time: 31 seconds → 0.2 seconds (155x faster!)
- P95 latency: 35s → 0.3s
- User experience: Dramatically improved

### 14.3 Memory Usage Reduction

**Problem:** Application using 2GB memory

**Analysis:**

```bash
heaptrack target/release/joshua assess
heaptrack_gui heaptrack.joshua.*.gz
```

Found:
- String allocations: 1.2GB
- Repeated copies of source names
- Large JSON response caching without compression

**Solution:**

```rust
// 1. Use Arc for shared strings
let source_name = Arc::new(String::from("Reuters"));

// 2. String interning for repeated values
let mut interner = StringInterner::new();
let source_symbol = interner.get_or_intern("Reuters");

// 3. Compress cached responses
let compressed = lz4::block::compress(&serialized, None, false)?;
cache.insert(key, compressed);
```

**Result:**
- Memory usage: 2GB → 500MB (75% reduction)
- Meets target of <500MB
- No performance degradation

---

## 15. Performance Checklist

### 15.1 Pre-Deployment Checklist

```markdown
## Performance Validation Checklist

### Profiling
- [ ] CPU profiling completed (flamegraph analyzed)
- [ ] Memory profiling completed (no leaks detected)
- [ ] No blocking calls in async code
- [ ] Hot paths identified and optimized

### Database
- [ ] All queries use appropriate indexes
- [ ] EXPLAIN ANALYZE performed on slow queries
- [ ] Connection pool configured optimally
- [ ] Query results cached appropriately
- [ ] Batch operations used where applicable

### Caching
- [ ] Multi-level caching implemented
- [ ] Cache hit rate > 80%
- [ ] Cache invalidation strategy defined
- [ ] Memory limits enforced

### API Optimization
- [ ] Claude API costs within budget (<$1/assessment)
- [ ] Response caching enabled (6-hour TTL)
- [ ] Token usage optimized
- [ ] Appropriate model selection (Sonnet for standard)

### Concurrency
- [ ] Parallel data collection enabled
- [ ] Rate limiting configured
- [ ] Semaphores prevent resource exhaustion
- [ ] No data races or deadlocks

### Compilation
- [ ] Release profile optimized (LTO enabled)
- [ ] Binary size reasonable
- [ ] No debug code in release build

### Load Testing
- [ ] Load tests passed (50 concurrent users)
- [ ] Stress tests identify breaking point (>100 users)
- [ ] Sustained load test (24 hours) passed
- [ ] P95 latency < target

### Monitoring
- [ ] Metrics exported (Prometheus format)
- [ ] Dashboards created (Grafana)
- [ ] Alerts configured
- [ ] Baseline established

### Resource Limits
- [ ] Memory usage < 500MB
- [ ] Assessment time < 5 minutes
- [ ] Database queries < 100ms (P95)
- [ ] Concurrent assessments > 10

### Documentation
- [ ] Performance characteristics documented
- [ ] Optimization decisions explained
- [ ] Benchmarks recorded
```

### 15.2 Regular Maintenance Tasks

**Weekly:**
- Review performance metrics
- Check cache hit rates
- Monitor API costs
- Verify no performance regressions

**Monthly:**
- Run full benchmark suite
- Update performance baselines
- Review and vacuum database
- Audit resource usage

**Quarterly:**
- Comprehensive performance review
- Identify optimization opportunities
- Update capacity planning
- Review and update this document

---

**Document Version:** 1.0.0
**Last Updated:** October 27, 2025
**Maintained By:** WarGames/JOSHUA Development Team

---

*"Optimization is the art of making the right tradeoffs. Measure first, optimize second, document always."*

---

For operational performance monitoring, see:
- [Monitoring and Observability](11_Monitoring_and_Observability.md)
- [Deployment and Operations](07_Deployment_and_Operations_Guide.md)
- [Claude API Integration](10_Claude_Integration_Specifications.md)
