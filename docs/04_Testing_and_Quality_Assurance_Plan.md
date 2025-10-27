# WarGames/JOSHUA: Testing & Quality Assurance Plan
## Comprehensive Testing Strategy for Nuclear Risk Assessment
### Version 1.0.0 | October 2025

---

## Executive Summary

This document defines the complete testing and quality assurance strategy for the WarGames/JOSHUA system. Given the critical nature of nuclear risk assessment, we employ a multi-layered testing approach encompassing unit tests, integration tests, property-based tests, historical validation, and continuous quality monitoring.

### Quality Targets

| Metric | Target | Critical Threshold |
|--------|--------|-------------------|
| Line Coverage | 95%+ | 90% minimum |
| Branch Coverage | 90%+ | 85% minimum |
| Integration Test Pass Rate | 100% | 100% required |
| Historical Validation Correlation | >0.8 | >0.7 minimum |
| Assessment Completion Time | <5 min | <10 min maximum |
| Zero Critical Bugs | 100% | 100% required |
| Memory Leaks | 0 | 0 tolerated |

---

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Unit Testing Strategy](#unit-testing-strategy)
3. [Integration Testing](#integration-testing)
4. [Property-Based Testing](#property-based-testing)
5. [Historical Validation](#historical-validation)
6. [Performance Testing](#performance-testing)
7. [Security Testing](#security-testing)
8. [Chaos Engineering](#chaos-engineering)
9. [Continuous Integration](#continuous-integration)
10. [Quality Metrics](#quality-metrics)

---

## 1. Testing Philosophy

### 1.1 Core Principles

```rust
/// Test pyramid structure
/// 
///        /\
///       /  \
///      / E2E\      <- 10% (End-to-end tests)
///     /______\
///    /        \
///   /Integration\ <- 30% (Integration tests)
///  /____________\
/// /              \
/// /    Unit Tests  \ <- 60% (Unit tests)
/// /__________________\
```

**Testing Principles:**

1. **Fast Feedback**: Unit tests run in <1s, full suite in <5min
2. **Deterministic**: Tests produce same results every run
3. **Isolated**: Tests don't depend on each other
4. **Comprehensive**: Cover all code paths and edge cases
5. **Maintainable**: Tests are readable and easy to update
6. **Realistic**: Test data mirrors real-world scenarios

### 1.2 Test Categories

```rust
#[cfg(test)]
mod tests {
    /// Unit tests: Test individual functions/methods
    #[test]
    fn test_risk_score_calculation() {
        let factors = vec![/* test factors */];
        let score = calculate_risk_score(&factors);
        assert!(score >= 0.0 && score <= 1.0);
    }
    
    /// Integration tests: Test component interactions
    #[tokio::test]
    async fn test_data_collection_pipeline() {
        let collector = DataCollector::new();
        let data = collector.collect_all().await.unwrap();
        assert!(!data.is_empty());
    }
    
    /// Property tests: Test invariants
    #[quickcheck]
    fn prop_risk_score_bounded(factors: Vec<RiskFactor>) -> bool {
        let score = calculate_risk_score(&factors);
        score >= 0.0 && score <= 1.0
    }
    
    /// Benchmark tests: Measure performance
    #[bench]
    fn bench_risk_calculation(b: &mut Bencher) {
        let factors = generate_test_factors();
        b.iter(|| calculate_risk_score(&factors));
    }
}
```

---

## 2. Unit Testing Strategy

### 2.1 Risk Calculation Tests

```rust
#[cfg(test)]
mod risk_calculation_tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_weighted_score_calculation() {
        let factors = vec![
            RiskFactor {
                category: RiskCategory::NuclearArsenalChanges,
                value: 0.5,
                weight: 0.15,
                confidence: ConfidenceLevel::High,
            },
            RiskFactor {
                category: RiskCategory::RegionalConflicts,
                value: 0.8,
                weight: 0.20,
                confidence: ConfidenceLevel::High,
            },
        ];
        
        let scorer = WeightedScorer::new();
        let score = scorer.calculate_base_score(&factors);
        
        // Expected: (0.5 * 0.15 + 0.8 * 0.20) / (0.15 + 0.20)
        let expected = (0.075 + 0.16) / 0.35;
        assert_relative_eq!(score, expected, epsilon = 0.001);
    }
    
    #[test]
    fn test_confidence_weighting() {
        let low_confidence = RiskFactor {
            value: 0.8,
            confidence: ConfidenceLevel::Low,
            ..Default::default()
        };
        
        let high_confidence = RiskFactor {
            value: 0.8,
            confidence: ConfidenceLevel::High,
            ..Default::default()
        };
        
        let scorer = WeightedScorer::new();
        let low_score = scorer.calculate_with_confidence(&[low_confidence]);
        let high_score = scorer.calculate_with_confidence(&[high_confidence]);
        
        // High confidence should have more impact
        assert!(high_score > low_score);
    }
    
    #[test]
    fn test_seconds_to_midnight_conversion() {
        let calculator = RiskCalculator::new();
        
        // Max risk (1.0) should be 0 seconds (midnight)
        assert_eq!(calculator.score_to_seconds(1.0), 0);
        
        // Min risk (0.0) should be 1440 seconds (noon)
        assert_eq!(calculator.score_to_seconds(0.0), 1440);
        
        // Mid risk (0.5) should be 720 seconds
        assert_eq!(calculator.score_to_seconds(0.5), 720);
    }
    
    #[test]
    fn test_empty_factors_handling() {
        let scorer = WeightedScorer::new();
        let score = scorer.calculate_base_score(&[]);
        
        // Empty factors should return baseline (minimal risk)
        assert_relative_eq!(score, 0.0, epsilon = 0.001);
    }
    
    #[test]
    #[should_panic(expected = "Invalid factor value")]
    fn test_invalid_factor_value() {
        let invalid_factor = RiskFactor {
            value: 1.5,  // Out of bounds!
            ..Default::default()
        };
        
        let scorer = WeightedScorer::new();
        scorer.calculate_base_score(&[invalid_factor]);
    }
}
```

### 2.2 Data Collection Tests

```rust
#[cfg(test)]
mod data_collection_tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_rss_feed_parsing() {
        let mock_feed = r#"
            <?xml version="1.0"?>
            <rss version="2.0">
              <channel>
                <item>
                  <title>Nuclear Test Conducted</title>
                  <description>Test description</description>
                  <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
                </item>
              </channel>
            </rss>
        "#;
        
        let _m = mock("GET", "/feed.xml")
            .with_status(200)
            .with_body(mock_feed)
            .create();
        
        let collector = RssFeedCollector::new();
        let articles = collector.parse_feed(&format!("{}/feed.xml", server_url())).await.unwrap();
        
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Nuclear Test Conducted");
    }
    
    #[tokio::test]
    async fn test_deduplication() {
        let duplicate_articles = vec![
            Article {
                title: "Same Story".to_string(),
                content: "This is the content of the story".to_string(),
                ..Default::default()
            },
            Article {
                title: "Same Story".to_string(),
                content: "This is the content of the story".to_string(),
                ..Default::default()
            },
            Article {
                title: "Different Story".to_string(),
                content: "Completely different content here".to_string(),
                ..Default::default()
            },
        ];
        
        let deduplicator = ContentDeduplicator::new();
        let unique = deduplicator.deduplicate(duplicate_articles).unwrap();
        
        assert_eq!(unique.len(), 2);
    }
    
    #[tokio::test]
    async fn test_rate_limiting() {
        let rate_limiter = RateLimiter::new(RateLimit {
            requests_per_minute: 2,
            requests_per_hour: 100,
            requests_per_day: 1000,
        });
        
        let start = std::time::Instant::now();
        
        // First two requests should be immediate
        rate_limiter.wait("test").await.unwrap();
        rate_limiter.wait("test").await.unwrap();
        
        assert!(start.elapsed() < Duration::from_millis(100));
        
        // Third request should be delayed
        let third_request_start = std::time::Instant::now();
        rate_limiter.wait("test").await.unwrap();
        
        assert!(third_request_start.elapsed() > Duration::from_secs(25));
    }
    
    #[tokio::test]
    async fn test_retry_on_failure() {
        let mut attempt = 0;
        
        let operation = || async {
            attempt += 1;
            if attempt < 3 {
                Err(anyhow::anyhow!("Temporary failure"))
            } else {
                Ok("Success")
            }
        };
        
        let retry = RetryStrategy::new(5, Duration::from_millis(10));
        let result = retry.execute(operation).await;
        
        assert!(result.is_ok());
        assert_eq!(attempt, 3);
    }
}
```

### 2.3 Bayesian Network Tests

```rust
#[cfg(test)]
mod bayesian_network_tests {
    use super::*;
    
    #[test]
    fn test_correlation_calculation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];  // Perfect correlation
        
        let network = BayesianNetwork::new();
        let correlation = network.pearson_correlation(&x, &y);
        
        assert_relative_eq!(correlation, 1.0, epsilon = 0.001);
    }
    
    #[test]
    fn test_belief_propagation_convergence() {
        let network = create_test_network();
        
        // Set evidence
        network.set_evidence("factor_a", 0.8);
        
        // Run belief propagation
        let beliefs = network.belief_propagation().unwrap();
        
        // Check that beliefs sum to 1.0 for each node
        for (node, belief) in beliefs {
            let sum: f64 = belief.probabilities.iter().sum();
            assert_relative_eq!(sum, 1.0, epsilon = 0.001);
        }
    }
    
    #[test]
    fn test_dag_cycle_detection() {
        let mut network = BayesianNetwork::new();
        
        // Create a cycle: A -> B -> C -> A
        network.add_edge("A", "B");
        network.add_edge("B", "C");
        
        // This should fail
        let result = network.add_edge("C", "A");
        assert!(result.is_err());
    }
}
```

### 2.4 Test Coverage Requirements

```rust
/// Coverage goals by module
/// 
/// Module                     Line Coverage    Branch Coverage
/// =====================================================
/// risk_calculation          100%             95%
/// data_collection           95%              90%
/// bayesian_network          95%              90%
/// visualization             90%              85%
/// report_generation         90%              85%
/// database                  95%              90%
/// claude_integration        95%              90%
/// utilities                 85%              80%
```

---

## 3. Integration Testing

### 3.1 End-to-End Pipeline Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_complete_assessment_pipeline() {
        // Initialize test system
        let mut system = WarGamesSystem::new_test();
        
        // Run complete assessment
        let assessment = system.run_complete_assessment().await.unwrap();
        
        // Verify assessment structure
        assert!(assessment.seconds_to_midnight > 0);
        assert!(assessment.seconds_to_midnight <= 1440);
        assert!(!assessment.risk_factors.is_empty());
        assert!(!assessment.report.is_empty());
        assert!(!assessment.visualizations.is_empty());
        
        // Verify database storage
        let stored = system.database
            .get_assessment(assessment.id)
            .await
            .unwrap();
        assert_eq!(stored.id, assessment.id);
    }
    
    #[tokio::test]
    async fn test_data_collection_to_analysis() {
        let collector = DataCollectionEngine::new_test();
        let analyzer = ClaudeAnalysisEngine::new_test();
        
        // Collect data
        let data = collector.collect_all().await.unwrap();
        assert!(!data.is_empty());
        
        // Analyze with Claude
        let analysis = analyzer.analyze(&data).await.unwrap();
        assert!(analysis.seconds_to_midnight > 0);
        assert!(!analysis.reasoning.is_empty());
    }
    
    #[tokio::test]
    async fn test_analysis_to_visualization() {
        let analysis = create_test_analysis();
        let visualizer = VisualizationEngine::new();
        
        // Generate visualizations
        let viz = visualizer.generate_all(&analysis).unwrap();
        
        // Verify all expected visualizations exist
        assert!(viz.doomsday_clock.is_some());
        assert!(viz.risk_timeline.is_some());
        assert!(viz.risk_matrix.is_some());
        assert!(viz.heat_map.is_some());
    }
    
    #[tokio::test]
    async fn test_database_round_trip() {
        let db = Database::new_test().await;
        
        // Create test assessment
        let assessment = create_test_assessment();
        
        // Store in database
        let id = db.save_assessment(&assessment).await.unwrap();
        
        // Retrieve from database
        let retrieved = db.get_assessment(id).await.unwrap();
        
        // Verify data integrity
        assert_eq!(retrieved.seconds_to_midnight, assessment.seconds_to_midnight);
        assert_eq!(retrieved.risk_factors.len(), assessment.risk_factors.len());
    }
}
```

### 3.2 Component Interaction Tests

```rust
#[cfg(test)]
mod component_interaction_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_collector_to_validator() {
        let collector = NewsCollector::new_test();
        let validator = DataValidator::new();
        
        let articles = collector.collect().await.unwrap();
        let validated = validator.validate_batch(&articles);
        
        // All collected articles should pass validation
        assert_eq!(validated.valid.len(), articles.len());
        assert!(validated.invalid.is_empty());
    }
    
    #[tokio::test]
    async fn test_calculator_to_database() {
        let calculator = RiskCalculator::new();
        let db = Database::new_test().await;
        
        let factors = generate_test_factors();
        let score = calculator.calculate_risk(&factors);
        
        // Store risk score
        let id = db.store_risk_score(&score).await.unwrap();
        
        // Verify storage
        let retrieved = db.get_risk_score(id).await.unwrap();
        assert_relative_eq!(
            retrieved.raw_score,
            score.raw_score,
            epsilon = 0.001
        );
    }
}
```

---

## 4. Property-Based Testing

### 4.1 Risk Calculation Properties

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};
    
    /// Risk score must always be in [0, 1]
    #[quickcheck]
    fn prop_risk_score_bounded(factors: Vec<TestRiskFactor>) -> bool {
        let calculator = RiskCalculator::new();
        let factors: Vec<RiskFactor> = factors.into_iter()
            .map(|f| f.into())
            .collect();
        
        let score = calculator.calculate_risk(&factors);
        score.raw_score >= 0.0 && score.raw_score <= 1.0
    }
    
    /// Adding more risk factors should not decrease risk
    #[quickcheck]
    fn prop_risk_monotonic(
        factors1: Vec<TestRiskFactor>,
        factors2: Vec<TestRiskFactor>
    ) -> TestResult {
        if factors1.is_empty() || factors2.is_empty() {
            return TestResult::discard();
        }
        
        let calculator = RiskCalculator::new();
        
        let score1 = calculator.calculate_risk(
            &factors1.iter().map(|f| (*f).into()).collect::<Vec<_>>()
        );
        
        let mut combined = factors1.clone();
        combined.extend(factors2);
        let score2 = calculator.calculate_risk(
            &combined.iter().map(|f| (*f).into()).collect::<Vec<_>>()
        );
        
        TestResult::from_bool(score2.raw_score >= score1.raw_score)
    }
    
    /// Confidence should correlate with data quality
    #[quickcheck]
    fn prop_confidence_data_quality(quality_score: f64) -> TestResult {
        if quality_score < 0.0 || quality_score > 1.0 {
            return TestResult::discard();
        }
        
        let assessment = create_assessment_with_quality(quality_score);
        let confidence = assessment.overall_confidence.to_numeric();
        
        // Higher data quality should generally lead to higher confidence
        // (with some tolerance for other factors)
        TestResult::from_bool(
            (confidence - quality_score).abs() < 0.3
        )
    }
    
    /// Bayesian adjustment should preserve bounds
    #[quickcheck]
    fn prop_bayesian_adjustment_bounded(
        base_score: f64,
        factors: Vec<TestRiskFactor>
    ) -> TestResult {
        if base_score < 0.0 || base_score > 1.0 {
            return TestResult::discard();
        }
        
        let network = BayesianNetwork::new_test();
        let factors: Vec<RiskFactor> = factors.into_iter()
            .map(|f| f.into())
            .collect();
        
        let adjusted = network.adjust_score(base_score, &factors, &[]);
        
        TestResult::from_bool(adjusted >= 0.0 && adjusted <= 1.0)
    }
    
    /// Arbitrary implementation for test generation
    #[derive(Clone, Copy, Debug)]
    struct TestRiskFactor {
        value: f64,
        category_index: usize,
    }
    
    impl Arbitrary for TestRiskFactor {
        fn arbitrary(g: &mut Gen) -> Self {
            TestRiskFactor {
                value: f64::arbitrary(g).abs().min(1.0),
                category_index: usize::arbitrary(g) % 7,  // 7 categories
            }
        }
    }
    
    impl From<TestRiskFactor> for RiskFactor {
        fn from(t: TestRiskFactor) -> Self {
            RiskFactor {
                value: t.value,
                category: RiskCategory::from_index(t.category_index),
                confidence: ConfidenceLevel::High,
                ..Default::default()
            }
        }
    }
}
```

### 4.2 Data Collection Properties

```rust
#[cfg(test)]
mod data_collection_properties {
    use super::*;
    
    /// Deduplication should be idempotent
    #[quickcheck]
    fn prop_deduplication_idempotent(articles: Vec<TestArticle>) -> bool {
        let deduplicator = ContentDeduplicator::new();
        let articles: Vec<Article> = articles.into_iter()
            .map(|a| a.into())
            .collect();
        
        let once = deduplicator.deduplicate(articles.clone()).unwrap();
        let twice = deduplicator.deduplicate(once.clone()).unwrap();
        
        once.len() == twice.len()
    }
    
    /// Cache hit should return same data as miss
    #[quickcheck]
    fn prop_cache_consistency(key: String, data: Vec<u8>) -> TestResult {
        if key.is_empty() {
            return TestResult::discard();
        }
        
        let mut cache = CacheManager::new_test();
        
        // Store in cache
        cache.set(&key, data.clone(), Duration::from_secs(60)).unwrap();
        
        // Retrieve from cache
        let cached = cache.get(&key).unwrap();
        
        TestResult::from_bool(cached == data)
    }
}
```

---

## 5. Historical Validation

### 5.1 Known Event Testing

```rust
#[cfg(test)]
mod historical_validation_tests {
    use super::*;
    
    /// Cuban Missile Crisis (October 1962)
    /// Expert Assessment: ~89 seconds to midnight (extremely high risk)
    #[test]
    fn test_cuban_missile_crisis_assessment() {
        let factors = cuban_missile_crisis_factors();
        let calculator = RiskCalculator::new();
        
        let score = calculator.calculate_risk(&factors);
        let seconds = calculator.score_to_seconds(score.raw_score);
        
        // Should be very close to midnight (0-120 seconds)
        assert!(seconds < 120, "Cuban Missile Crisis should show extreme risk");
        
        // Correlation with expert assessment
        let expert_score = 0.938;  // ~89 seconds
        assert_relative_eq!(
            score.raw_score,
            expert_score,
            epsilon = 0.10  // Within 10%
        );
    }
    
    /// 1983 Soviet False Alarm (Petrov Incident)
    /// Expert Assessment: ~100 seconds to midnight
    #[test]
    fn test_petrov_incident_assessment() {
        let factors = petrov_incident_factors();
        let calculator = RiskCalculator::new();
        
        let score = calculator.calculate_risk(&factors);
        let seconds = calculator.score_to_seconds(score.raw_score);
        
        assert!(seconds < 150);
        
        // Should identify technical incident as major risk driver
        let top_driver = score.primary_drivers.first().unwrap();
        assert!(matches!(
            top_driver.category,
            RiskCategory::TechnicalIncidents
        ));
    }
    
    /// Post-Cold War Minimum (early 1990s)
    /// Expert Assessment: ~17 minutes (1020 seconds)
    #[test]
    fn test_post_cold_war_minimum() {
        let factors = post_cold_war_factors();
        let calculator = RiskCalculator::new();
        
        let score = calculator.calculate_risk(&factors);
        let seconds = calculator.score_to_seconds(score.raw_score);
        
        // Should be far from midnight
        assert!(seconds > 900);
        assert!(seconds < 1200);
    }
    
    /// 2025 Current Situation
    /// Expert Assessment: 89 seconds to midnight
    #[test]
    fn test_2025_current_assessment() {
        let factors = current_2025_factors();
        let calculator = RiskCalculator::new();
        
        let score = calculator.calculate_risk(&factors);
        let seconds = calculator.score_to_seconds(score.raw_score);
        
        // Should match Bulletin of Atomic Scientists assessment
        assert!(seconds >= 85 && seconds <= 95);
    }
    
    /// Historical correlation test
    #[test]
    fn test_historical_correlation() {
        let historical_events = load_historical_events();
        let calculator = RiskCalculator::new();
        
        let mut predictions = Vec::new();
        let mut actuals = Vec::new();
        
        for event in historical_events {
            let predicted = calculator.calculate_risk(&event.factors);
            predictions.push(predicted.raw_score);
            actuals.push(event.expert_assessment);
        }
        
        let correlation = calculate_correlation(&predictions, &actuals);
        
        // Must achieve >0.7 correlation with expert assessments
        assert!(
            correlation > 0.7,
            "Historical correlation {} below threshold",
            correlation
        );
    }
}

fn cuban_missile_crisis_factors() -> Vec<RiskFactor> {
    vec![
        RiskFactor {
            category: RiskCategory::RegionalConflicts,
            value: 0.95,  // Active military confrontation
            confidence: ConfidenceLevel::VeryHigh,
            ..Default::default()
        },
        RiskFactor {
            category: RiskCategory::LeadershipAndRhetoric,
            value: 0.90,  // Explicit nuclear threats
            confidence: ConfidenceLevel::VeryHigh,
            ..Default::default()
        },
        RiskFactor {
            category: RiskCategory::CommunicationBreakdown,
            value: 0.85,  // Limited channels
            confidence: ConfidenceLevel::High,
            ..Default::default()
        },
        RiskFactor {
            category: RiskCategory::DoctrineAndPosture,
            value: 0.80,  // DEFCON 2
            confidence: ConfidenceLevel::VeryHigh,
            ..Default::default()
        },
    ]
}
```

### 5.2 Backtesting Framework

```rust
#[cfg(test)]
mod backtesting_tests {
    use super::*;
    
    #[test]
    fn test_walk_forward_validation() {
        let historical_data = load_historical_assessments();
        let calculator = RiskCalculator::new();
        
        let mut errors = Vec::new();
        
        // Walk forward through history
        for window in historical_data.windows(12) {  // 12-month windows
            let training = &window[0..11];
            let test = &window[11];
            
            // Train/calibrate on historical data
            let calibrated_calculator = calculator.calibrate(training);
            
            // Predict test point
            let prediction = calibrated_calculator.calculate_risk(&test.factors);
            
            // Calculate error
            let error = (prediction.raw_score - test.actual_score).abs();
            errors.push(error);
        }
        
        // Average error should be low
        let mean_error = errors.iter().sum::<f64>() / errors.len() as f64;
        assert!(mean_error < 0.15, "Mean prediction error too high: {}", mean_error);
    }
    
    #[test]
    fn test_direction_accuracy() {
        let historical_data = load_historical_assessments();
        let calculator = RiskCalculator::new();
        
        let mut correct_directions = 0;
        let mut total = 0;
        
        for i in 1..historical_data.len() {
            let prev = &historical_data[i-1];
            let curr = &historical_data[i];
            
            let prev_pred = calculator.calculate_risk(&prev.factors);
            let curr_pred = calculator.calculate_risk(&curr.factors);
            
            let predicted_direction = if curr_pred.raw_score > prev_pred.raw_score {
                Direction::Increasing
            } else if curr_pred.raw_score < prev_pred.raw_score {
                Direction::Decreasing
            } else {
                Direction::Stable
            };
            
            let actual_direction = if curr.actual_score > prev.actual_score {
                Direction::Increasing
            } else if curr.actual_score < prev.actual_score {
                Direction::Decreasing
            } else {
                Direction::Stable
            };
            
            if predicted_direction == actual_direction {
                correct_directions += 1;
            }
            total += 1;
        }
        
        let direction_accuracy = correct_directions as f64 / total as f64;
        
        // Should correctly predict direction >70% of time
        assert!(
            direction_accuracy > 0.7,
            "Direction accuracy {} below threshold",
            direction_accuracy
        );
    }
}
```

---

## 6. Performance Testing

### 6.1 Benchmark Tests

```rust
#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn bench_risk_calculation(c: &mut Criterion) {
        let factors = generate_realistic_factors(50);
        let calculator = RiskCalculator::new();
        
        c.bench_function("risk_calculation", |b| {
            b.iter(|| {
                calculator.calculate_risk(black_box(&factors))
            })
        });
    }
    
    fn bench_bayesian_inference(c: &mut Criterion) {
        let network = BayesianNetwork::new_test();
        let factors = generate_realistic_factors(30);
        
        c.bench_function("bayesian_inference", |b| {
            b.iter(|| {
                network.belief_propagation(black_box(&factors))
            })
        });
    }
    
    fn bench_monte_carlo_simulation(c: &mut Criterion) {
        let simulator = MonteCarloSimulator::new(1000);  // 1000 iterations
        let initial_state = WorldState::default();
        
        c.bench_function("monte_carlo_1000", |b| {
            b.iter(|| {
                simulator.simulate(
                    black_box(&initial_state),
                    Duration::from_days(365)
                )
            })
        });
    }
    
    fn bench_data_collection(c: &mut Criterion) {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let collector = DataCollectionEngine::new_test();
        
        c.bench_function("data_collection", |b| {
            b.iter(|| {
                runtime.block_on(async {
                    collector.collect_all().await
                })
            })
        });
    }
    
    criterion_group!(
        benches,
        bench_risk_calculation,
        bench_bayesian_inference,
        bench_monte_carlo_simulation,
        bench_data_collection
    );
    criterion_main!(benches);
}

/// Performance targets
const PERFORMANCE_TARGETS: &[(&str, Duration)] = &[
    ("risk_calculation", Duration::from_millis(100)),
    ("bayesian_inference", Duration::from_secs(1)),
    ("monte_carlo_1000", Duration::from_secs(10)),
    ("data_collection", Duration::from_secs(30)),
    ("visualization_generation", Duration::from_secs(2)),
    ("report_generation", Duration::from_secs(5)),
    ("complete_assessment", Duration::from_secs(300)),  // 5 minutes
];
```

### 6.2 Load Testing

```rust
#[cfg(test)]
mod load_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_concurrent_assessments() {
        let system = WarGamesSystem::new_test();
        let num_concurrent = 10;
        
        let start = Instant::now();
        
        let handles: Vec<_> = (0..num_concurrent)
            .map(|_| {
                let system = system.clone();
                tokio::spawn(async move {
                    system.run_complete_assessment().await
                })
            })
            .collect();
        
        let results = futures::future::join_all(handles).await;
        let elapsed = start.elapsed();
        
        // All should complete successfully
        assert!(results.iter().all(|r| r.is_ok()));
        
        // Should complete in reasonable time even under load
        assert!(elapsed < Duration::from_secs(600));
    }
    
    #[tokio::test]
    async fn test_memory_usage() {
        let system = WarGamesSystem::new_test();
        
        let initial_mem = get_memory_usage();
        
        // Run 100 assessments
        for _ in 0..100 {
            let _ = system.run_complete_assessment().await.unwrap();
        }
        
        let final_mem = get_memory_usage();
        
        // Memory growth should be minimal (no leaks)
        let growth = final_mem - initial_mem;
        assert!(growth < 100_000_000, "Memory leak detected: {} bytes", growth);
    }
}
```

---

## 7. Security Testing

### 7.1 Input Validation Tests

```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_sql_injection_prevention() {
        let db = Database::new_test().await;
        
        // Malicious input
        let malicious_query = "'; DROP TABLE assessments; --";
        
        // Should safely handle without executing malicious SQL
        let result = db.search_assessments(malicious_query).await;
        
        // Either safe error or empty results, but no SQL execution
        assert!(result.is_ok() || result.is_err());
        
        // Verify table still exists
        let count = db.count_assessments().await.unwrap();
        assert!(count >= 0);
    }
    
    #[test]
    fn test_xss_prevention_in_reports() {
        let generator = ReportGenerator::new();
        
        let malicious_data = Assessment {
            executive_summary: "<script>alert('XSS')</script>".to_string(),
            ..Default::default()
        };
        
        let html = generator.generate_html_report(&malicious_data).unwrap();
        
        // Script tags should be escaped
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;") || html.contains("alert"));
    }
    
    #[test]
    fn test_api_key_encryption() {
        let security_manager = SecurityManager::new();
        
        let api_key = "sk-ant-test-key-12345";
        let encrypted = security_manager.encrypt_api_key(api_key).unwrap();
        
        // Encrypted should not contain plain key
        assert!(!format!("{:?}", encrypted).contains(api_key));
        
        // Should be able to decrypt
        let decrypted = security_manager.decrypt_api_key(&encrypted).unwrap();
        assert_eq!(decrypted, api_key);
    }
    
    #[test]
    fn test_path_traversal_prevention() {
        let file_manager = FileManager::new();
        
        // Malicious paths
        let malicious_paths = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32",
            "/etc/passwd",
            "C:\\Windows\\System32",
        ];
        
        for path in malicious_paths {
            let result = file_manager.read_file(path);
            
            // Should reject all malicious paths
            assert!(result.is_err(), "Path traversal not prevented: {}", path);
        }
    }
}
```

---

## 8. Chaos Engineering

### 8.1 Failure Injection Tests

```rust
#[cfg(test)]
mod chaos_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_network_failure_resilience() {
        let mut system = WarGamesSystem::new_test();
        
        // Inject network failures
        system.inject_network_failure_rate(0.5);  // 50% failure rate
        
        // Should still complete with degraded performance
        let result = system.run_complete_assessment().await;
        
        assert!(result.is_ok(), "System should handle network failures");
        
        let assessment = result.unwrap();
        // Should flag data quality issues
        assert!(assessment.data_quality_warnings.len() > 0);
    }
    
    #[tokio::test]
    async fn test_api_timeout_handling() {
        let claude_client = ClaudeClient::new_test();
        
        // Set very short timeout
        claude_client.set_timeout(Duration::from_millis(1));
        
        let result = claude_client.analyze(&test_data()).await;
        
        // Should handle timeout gracefully
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Timeout));
    }
    
    #[tokio::test]
    async fn test_database_connection_loss() {
        let mut system = WarGamesSystem::new_test();
        
        // Start assessment
        let assessment_future = system.run_complete_assessment();
        
        // Drop database connection mid-assessment
        tokio::time::sleep(Duration::from_secs(1)).await;
        system.database.disconnect();
        
        // Should handle gracefully with appropriate error
        let result = assessment_future.await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_partial_data_source_failure() {
        let mut collector = DataCollectionEngine::new_test();
        
        // Disable 50% of data sources
        collector.disable_random_sources(0.5);
        
        let result = collector.collect_all().await;
        
        // Should still succeed with reduced data
        assert!(result.is_ok());
        
        let data = result.unwrap();
        assert!(data.len() > 0, "Should collect from available sources");
    }
}
```

---

## 9. Continuous Integration

### 9.1 CI Pipeline Configuration

```yaml
# .github/workflows/ci.yml
name: WarGames CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Lint with Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run tests
        run: cargo test --all-features --verbose
      
      - name: Run doc tests
        run: cargo test --doc
      
      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml --output-dir coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: coverage/cobertura.xml
      
      - name: Check coverage threshold
        run: |
          COVERAGE=$(cargo tarpaulin --print-summary | grep -oP '\d+\.\d+(?=%)')
          if (( $(echo "$COVERAGE < 95.0" | bc -l) )); then
            echo "Coverage $COVERAGE% below 95% threshold"
            exit 1
          fi

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run benchmarks
        run: cargo bench --verbose
      
      - name: Check performance regressions
        run: |
          # Compare against baseline
          cargo bench --save-baseline main
```

---

## 10. Quality Metrics Dashboard

### 10.1 Automated Quality Tracking

```rust
/// Quality metrics collection
pub struct QualityMetrics {
    pub test_coverage: Coverage,
    pub test_pass_rate: f64,
    pub benchmark_results: BenchmarkResults,
    pub code_quality: CodeQuality,
    pub documentation_coverage: f64,
}

pub struct Coverage {
    pub line_coverage: f64,
    pub branch_coverage: f64,
    pub function_coverage: f64,
}

pub struct BenchmarkResults {
    pub risk_calculation_time: Duration,
    pub data_collection_time: Duration,
    pub complete_assessment_time: Duration,
    pub regressions: Vec<PerfRegression>,
}

pub struct CodeQuality {
    pub clippy_warnings: u32,
    pub complexity_score: f64,
    pub duplicate_code_percentage: f64,
    pub technical_debt_ratio: f64,
}

impl QualityMetrics {
    /// Generate quality report
    pub fn generate_report(&self) -> QualityReport {
        let mut issues = Vec::new();
        
        // Check coverage
        if self.test_coverage.line_coverage < 0.95 {
            issues.push(QualityIssue {
                severity: Severity::High,
                message: format!(
                    "Line coverage {}% below 95% threshold",
                    self.test_coverage.line_coverage * 100.0
                ),
            });
        }
        
        // Check performance
        if self.benchmark_results.complete_assessment_time > Duration::from_secs(300) {
            issues.push(QualityIssue {
                severity: Severity::Medium,
                message: "Assessment time exceeds 5-minute target".to_string(),
            });
        }
        
        // Check code quality
        if self.code_quality.clippy_warnings > 0 {
            issues.push(QualityIssue {
                severity: Severity::Low,
                message: format!(
                    "{} Clippy warnings present",
                    self.code_quality.clippy_warnings
                ),
            });
        }
        
        QualityReport {
            timestamp: Utc::now(),
            overall_status: if issues.is_empty() {
                Status::Passing
            } else {
                Status::NeedsAttention
            },
            metrics: self.clone(),
            issues,
        }
    }
}
```

---

## Conclusion

This comprehensive testing and QA strategy ensures the WarGames/JOSHUA system maintains the highest standards of quality, reliability, and accuracy. By combining multiple testing approachesâ€”unit, integration, property-based, historical validation, performance, and security testingâ€”we create a robust safety net that catches bugs early and validates correctness continuously.

### Implementation Checklist

- [ ] Unit tests for all modules (95%+ coverage)
- [ ] Integration tests for component interactions
- [ ] Property-based tests for invariants
- [ ] Historical validation against known events
- [ ] Performance benchmarks meeting targets
- [ ] Security tests for common vulnerabilities
- [ ] Chaos engineering tests for resilience
- [ ] CI/CD pipeline with automated quality gates
- [ ] Quality metrics dashboard
- [ ] Documentation of test cases and procedures

**Quality is not negotiable when assessing nuclear war risk.**

*"The only way to ensure quality is to test relentlessly."*
