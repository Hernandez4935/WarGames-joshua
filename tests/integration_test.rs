//! Integration tests for WarGames/JOSHUA system.

use wargames_joshua::prelude::*;

#[tokio::test]
async fn test_system_initialization() {
    // Test that the system can be initialized
    let config = Config::default();
    assert_eq!(config.general.app_name, "WarGames/JOSHUA");
}

#[test]
fn test_risk_factor_creation() {
    let factor = RiskFactor::new(
        RiskCategory::RegionalConflicts,
        "Test Conflict".to_string(),
        0.75,
        ConfidenceLevel::High,
    );

    assert_eq!(factor.category, RiskCategory::RegionalConflicts);
    assert_eq!(factor.value, 0.75);
    assert_eq!(factor.confidence, ConfidenceLevel::High);
}

#[test]
fn test_assessment_creation() {
    let assessment = Assessment::new(89, "Test executive summary".to_string());

    assert_eq!(assessment.seconds_to_midnight, 89);
    assert!(assessment.raw_risk_score > 0.9);
    assert_eq!(assessment.risk_level(), "Critical");
}

#[test]
fn test_data_point_builder() {
    let dp = DataPoint::new(
        "Reuters".to_string(),
        "Test content".to_string(),
        DataCategory::NewsMedia,
    )
    .with_url("https://example.com".to_string())
    .with_title("Test Title".to_string())
    .with_reliability(0.85);

    assert_eq!(dp.source, "Reuters");
    assert_eq!(dp.reliability, 0.85);
    assert!(dp.source_url.is_some());
    assert!(dp.title.is_some());
}

#[test]
fn test_confidence_level_conversions() {
    assert_eq!(ConfidenceLevel::High.to_score(), 0.8);
    assert_eq!(ConfidenceLevel::from_score(0.95), ConfidenceLevel::VeryHigh);
    assert_eq!(ConfidenceLevel::from_score(0.25), ConfidenceLevel::VeryLow);
}

#[test]
fn test_risk_category_weights() {
    let categories = RiskCategory::all();
    let total_weight: f64 = categories.iter().map(|c| c.default_weight()).sum();

    // Weights should sum to 1.0
    assert!((total_weight - 1.0).abs() < 0.001);
}

#[test]
fn test_error_types() {
    let err = Error::validation("Invalid input");
    assert!(err.to_string().contains("Validation error"));

    let err = Error::not_found("Resource");
    assert!(err.to_string().contains("Not found"));
}
