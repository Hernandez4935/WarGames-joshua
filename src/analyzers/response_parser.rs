//! Claude response parsing and validation.
//!
//! This module handles parsing Claude's JSON responses with comprehensive
//! validation, error recovery, and schema compliance checking.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Claude risk analysis response (matches expected JSON schema)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeRiskAnalysis {
    /// Seconds to midnight (0-1440)
    pub seconds_to_midnight: u32,

    /// Overall confidence level
    pub confidence_level: ConfidenceLevel,

    /// Trend direction
    pub trend_direction: TrendDirection,

    /// Risk factor scores by category
    pub risk_factors: HashMap<String, f64>,

    /// Critical developments identified
    pub critical_developments: Vec<CriticalDevelopment>,

    /// Early warning indicators
    pub early_warning_indicators: Vec<String>,

    /// Executive summary
    pub executive_summary: String,

    /// Detailed analysis
    pub detailed_analysis: String,

    /// Recommendations
    pub recommendations: Vec<String>,
}

/// A critical development event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalDevelopment {
    /// Event description
    pub event: String,

    /// Impact level
    pub impact: ImpactLevel,

    /// Affected regions
    pub affected_regions: Vec<String>,

    /// Escalation potential (0.0 to 1.0)
    pub escalation_potential: f64,
}

/// Response parser for Claude analysis
pub struct ResponseParser {
    confidence_threshold: f64,
}

impl ResponseParser {
    /// Create a new response parser
    pub fn new() -> Self {
        Self {
            confidence_threshold: 0.3, // Minimum confidence threshold
        }
    }

    /// Parse Claude's JSON response with validation
    pub fn parse_risk_analysis(&self, response_text: &str) -> Result<ClaudeRiskAnalysis> {
        // Clean the response (remove markdown code blocks if present)
        let cleaned = self.clean_json_response(response_text)?;

        // Parse JSON
        let raw_json: serde_json::Value = serde_json::from_str(&cleaned)
            .map_err(|e| Error::Parsing(format!("Failed to parse JSON: {}", e)))?;

        // Deserialize to structured type
        let analysis: ClaudeRiskAnalysis = serde_json::from_value(raw_json.clone())
            .map_err(|e| Error::Parsing(format!("Failed to deserialize: {}", e)))?;

        // Validate the analysis
        self.validate_analysis(&analysis)?;

        Ok(analysis)
    }

    /// Clean JSON response (remove markdown code blocks, extra text)
    fn clean_json_response(&self, text: &str) -> Result<String> {
        let text = text.trim();

        // Remove markdown code blocks if present
        let cleaned = if text.starts_with("```json") || text.starts_with("```") {
            text.lines()
                .skip(1) // Skip opening ```
                .take_while(|line| !line.trim().starts_with("```"))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            text.to_string()
        };

        // Find JSON boundaries
        let start = cleaned
            .find('{')
            .ok_or_else(|| Error::Parsing("No JSON object found in response".to_string()))?;
        let end = cleaned
            .rfind('}')
            .ok_or_else(|| Error::Parsing("No closing brace found in response".to_string()))?;

        Ok(cleaned[start..=end].to_string())
    }

    /// Validate analysis against business rules
    fn validate_analysis(&self, analysis: &ClaudeRiskAnalysis) -> Result<()> {
        // Validate seconds_to_midnight range
        if analysis.seconds_to_midnight > crate::constants::MAX_SECONDS_TO_MIDNIGHT {
            return Err(Error::Validation(format!(
                "Invalid seconds_to_midnight: {} (max: {})",
                analysis.seconds_to_midnight,
                crate::constants::MAX_SECONDS_TO_MIDNIGHT
            )));
        }

        // Validate risk factors
        let expected_factors = vec![
            "nuclear_arsenal_changes",
            "arms_control_breakdown",
            "regional_conflicts",
            "leadership_instability",
            "technical_incidents",
            "communication_failures",
            "emerging_tech_risks",
            "economic_pressure",
        ];

        for factor in &expected_factors {
            if let Some(&value) = analysis.risk_factors.get(*factor) {
                if !(0.0..=1.0).contains(&value) {
                    return Err(Error::Validation(format!(
                        "Invalid risk factor {}: {} (must be 0.0-1.0)",
                        factor, value
                    )));
                }
            } else {
                tracing::warn!("Missing risk factor: {}", factor);
            }
        }

        // Validate confidence threshold
        let confidence_score = analysis.confidence_level.to_score();
        if confidence_score < self.confidence_threshold {
            tracing::warn!(
                "Low confidence analysis: {:.2} (threshold: {:.2})",
                confidence_score,
                self.confidence_threshold
            );
        }

        // Validate required fields
        if analysis.executive_summary.is_empty() {
            return Err(Error::Validation("Missing executive summary".to_string()));
        }

        if analysis.detailed_analysis.is_empty() {
            return Err(Error::Validation("Missing detailed analysis".to_string()));
        }

        // Validate escalation potentials
        for dev in &analysis.critical_developments {
            if !(0.0..=1.0).contains(&dev.escalation_potential) {
                return Err(Error::Validation(format!(
                    "Invalid escalation_potential: {} (must be 0.0-1.0)",
                    dev.escalation_potential
                )));
            }
        }

        Ok(())
    }

    /// Attempt to extract partial JSON on parse failure (recovery strategy)
    pub fn extract_partial(&self, response_text: &str) -> Option<ClaudeRiskAnalysis> {
        // Try to find key fields with regex patterns
        // This is a fallback for malformed responses
        use regex::Regex;

        let seconds_re = Regex::new(r#""seconds_to_midnight"\s*:\s*(\d+)"#).ok()?;
        let seconds = seconds_re
            .captures(response_text)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok())?;

        // If we can extract at least seconds_to_midnight, create a minimal analysis
        Some(ClaudeRiskAnalysis {
            seconds_to_midnight: seconds,
            confidence_level: ConfidenceLevel::Low,
            trend_direction: TrendDirection::Uncertain,
            risk_factors: HashMap::new(),
            critical_developments: Vec::new(),
            early_warning_indicators: Vec::new(),
            executive_summary: "Partial analysis extracted from malformed response".to_string(),
            detailed_analysis: response_text.to_string(),
            recommendations: Vec::new(),
        })
    }

    /// Convert Claude analysis to Assessment
    pub fn to_assessment(&self, analysis: ClaudeRiskAnalysis) -> Assessment {
        let mut assessment = Assessment::new(
            analysis.seconds_to_midnight,
            analysis.executive_summary.clone(),
        );

        assessment.overall_confidence = analysis.confidence_level;
        assessment.trend_direction = analysis.trend_direction;
        assessment.detailed_analysis = analysis.detailed_analysis;
        assessment.recommendations = analysis.recommendations;

        // Convert risk factors
        for (category, score) in analysis.risk_factors {
            let mut risk_factor = RiskFactor::new(
                self.parse_risk_category(&category),
                category.clone(),
                score,
                analysis.confidence_level,
            );
            risk_factor.set_trend(analysis.trend_direction);
            assessment.risk_factors.push(risk_factor);
        }

        // Add critical warnings
        for dev in analysis.critical_developments {
            let warning = format!(
                "{} (Impact: {:?}, Escalation: {:.2})",
                dev.event, dev.impact, dev.escalation_potential
            );
            assessment.critical_warnings.push(warning);
        }

        assessment
    }

    fn parse_risk_category(&self, name: &str) -> RiskCategory {
        match name {
            "nuclear_arsenal_changes" => RiskCategory::NuclearArsenalChanges,
            "arms_control_breakdown" => RiskCategory::ArmsControlBreakdown,
            "regional_conflicts" => RiskCategory::RegionalConflicts,
            "leadership_instability" => RiskCategory::LeadershipInstability,
            "technical_incidents" => RiskCategory::TechnicalIncidents,
            "communication_failures" => RiskCategory::CommunicationFailures,
            "emerging_tech_risks" => RiskCategory::EmergingTechRisks,
            "economic_pressure" => RiskCategory::EconomicPressure,
            _ => RiskCategory::RegionalConflicts, // Default
        }
    }
}

impl Default for ResponseParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_json_with_markdown() {
        let parser = ResponseParser::new();
        let input = "```json\n{\"test\": \"value\"}\n```";
        let cleaned = parser.clean_json_response(input).unwrap();
        assert_eq!(cleaned, "{\"test\": \"value\"}");
    }

    #[test]
    fn test_clean_json_without_markdown() {
        let parser = ResponseParser::new();
        let input = "{\"test\": \"value\"}";
        let cleaned = parser.clean_json_response(input).unwrap();
        assert_eq!(cleaned, "{\"test\": \"value\"}");
    }

    #[test]
    fn test_parse_valid_analysis() {
        let parser = ResponseParser::new();
        let json = r#"{
            "seconds_to_midnight": 90,
            "confidence_level": "High",
            "trend_direction": "Deteriorating",
            "risk_factors": {
                "nuclear_arsenal_changes": 0.75,
                "arms_control_breakdown": 0.80,
                "regional_conflicts": 0.85,
                "leadership_instability": 0.60,
                "technical_incidents": 0.45,
                "communication_failures": 0.55,
                "emerging_tech_risks": 0.70,
                "economic_pressure": 0.40
            },
            "critical_developments": [],
            "early_warning_indicators": ["Test indicator"],
            "executive_summary": "Test summary",
            "detailed_analysis": "Detailed test analysis",
            "recommendations": ["Test recommendation"]
        }"#;

        let result = parser.parse_risk_analysis(json);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert_eq!(analysis.seconds_to_midnight, 90);
    }

    #[test]
    fn test_validate_invalid_seconds() {
        let parser = ResponseParser::new();
        let analysis = ClaudeRiskAnalysis {
            seconds_to_midnight: 2000, // Invalid: > 1440
            confidence_level: ConfidenceLevel::High,
            trend_direction: TrendDirection::Stable,
            risk_factors: HashMap::new(),
            critical_developments: Vec::new(),
            early_warning_indicators: Vec::new(),
            executive_summary: "Test".to_string(),
            detailed_analysis: "Test".to_string(),
            recommendations: Vec::new(),
        };

        let result = parser.validate_analysis(&analysis);
        assert!(result.is_err());
    }
}
