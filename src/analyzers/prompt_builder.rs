//! Prompt construction for Claude analysis.
//!
//! This module handles building comprehensive prompts for Claude AI analysis,
//! incorporating historical context, data summaries, and structured output requirements.

use crate::collectors::AggregatedData;
use crate::models::Assessment;
use crate::prelude::*;
use chrono::Utc;

/// Prompt builder for Claude analysis
pub struct PromptBuilder {
    system_prompt: String,
}

impl PromptBuilder {
    /// Create a new prompt builder
    pub fn new() -> Self {
        Self {
            system_prompt: crate::constants::SYSTEM_PROMPT.to_string(),
        }
    }

    /// Get the system prompt
    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    /// Build risk assessment prompt from collected data
    pub fn build_risk_assessment_prompt(
        &self,
        data: &AggregatedData,
        previous_assessment: Option<&Assessment>,
    ) -> Result<String> {
        let mut prompt = String::new();

        // 1. Header
        prompt.push_str("# Nuclear Risk Assessment Request\n\n");
        prompt.push_str(&format!(
            "**Assessment Date**: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        prompt.push_str(&format!(
            "**Data Collection Period**: {} to {}\n\n",
            data.collection_start.format("%Y-%m-%d %H:%M UTC"),
            data.collection_end.format("%Y-%m-%d %H:%M UTC")
        ));

        // 2. Previous Assessment Baseline
        if let Some(prev) = previous_assessment {
            prompt.push_str("## Previous Assessment Baseline\n\n");
            prompt.push_str(&format!(
                "- **Date**: {}\n",
                prev.assessment_date.format("%Y-%m-%d")
            ));
            prompt.push_str(&format!(
                "- **Seconds to Midnight**: {}\n",
                prev.seconds_to_midnight
            ));
            prompt.push_str(&format!("- **Risk Level**: {}\n", prev.risk_level()));
            prompt.push_str(&format!(
                "- **Confidence**: {:?}\n",
                prev.overall_confidence
            ));
            if !prev.risk_factors.is_empty() {
                let top_factors = prev
                    .risk_factors
                    .iter()
                    .take(3)
                    .map(|f| format!("{} ({:.2})", f.name, f.value))
                    .collect::<Vec<_>>()
                    .join(", ");
                prompt.push_str(&format!("- **Top Risk Factors**: {}\n", top_factors));
            }
            prompt.push_str("\n");
        } else {
            prompt.push_str("## Previous Assessment\n\n");
            prompt.push_str(
                "**No previous assessment available.** This is a baseline assessment.\n\n",
            );
        }

        // 3. Data Collection Summary
        prompt.push_str("## Collected Intelligence Data\n\n");
        prompt.push_str(&format!(
            "- **Total Data Points**: {}\n",
            data.data_points.len()
        ));
        prompt.push_str(&format!(
            "- **Sources**: {} active, {} failed\n",
            data.sources_count,
            data.failed_sources.len()
        ));

        if !data.failed_sources.is_empty() {
            prompt.push_str(&format!(
                "- **Failed Sources**: {}\n",
                data.failed_sources.join(", ")
            ));
        }
        prompt.push_str("\n");

        // 4. Data by Category
        prompt.push_str("## Intelligence by Risk Category\n\n");

        let categories = [
            "Nuclear Arsenal",
            "Arms Control",
            "Regional Conflicts",
            "Leadership/Rhetoric",
            "Technical Incidents",
            "Communication",
            "Emerging Tech",
            "Economic Factors",
        ];

        for category in &categories {
            let category_data: Vec<_> = data
                .data_points
                .iter()
                .filter(|dp| self.matches_category(category, dp))
                .take(5) // Top 5 per category
                .collect();

            if !category_data.is_empty() {
                prompt.push_str(&format!("### {}\n\n", category));
                for point in category_data {
                    if let Some(title) = &point.title {
                        prompt.push_str(&format!("**[{}]** {}\n", point.source, title));
                    }
                    let summary = point.content.chars().take(200).collect::<String>();
                    prompt.push_str(&format!("{}\n\n", summary));
                }
            }
        }

        // 5. Assessment Instructions
        prompt.push_str("## Assessment Requirements\n\n");
        prompt.push_str("Please provide a comprehensive nuclear risk assessment in **valid JSON format** with the following structure:\n\n");
        prompt.push_str("```json\n");
        prompt.push_str("{\n");
        prompt.push_str("  \"seconds_to_midnight\": <integer 0-1440>,\n");
        prompt.push_str("  \"confidence_level\": \"VeryLow|Low|Moderate|High|VeryHigh\",\n");
        prompt.push_str("  \"trend_direction\": \"Increasing|Decreasing|Stable\",\n");
        prompt.push_str("  \"risk_factors\": {\n");
        prompt.push_str("    \"nuclear_arsenal_changes\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"arms_control_breakdown\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"regional_conflicts\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"leadership_instability\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"technical_incidents\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"communication_failures\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"emerging_tech_risks\": <float 0.0-1.0>,\n");
        prompt.push_str("    \"economic_pressure\": <float 0.0-1.0>\n");
        prompt.push_str("  },\n");
        prompt.push_str("  \"critical_developments\": [\n");
        prompt.push_str("    {\n");
        prompt.push_str("      \"event\": \"<description>\",\n");
        prompt.push_str("      \"impact\": \"Low|Medium|High|Critical\",\n");
        prompt.push_str("      \"affected_regions\": [\"<regions>\"],\n");
        prompt.push_str("      \"escalation_potential\": <float 0.0-1.0>\n");
        prompt.push_str("    }\n");
        prompt.push_str("  ],\n");
        prompt
            .push_str("  \"early_warning_indicators\": [\"<indicator 1>\", \"<indicator 2>\"],\n");
        prompt.push_str("  \"executive_summary\": \"<2-3 paragraph summary>\",\n");
        prompt.push_str("  \"detailed_analysis\": \"<comprehensive analysis with evidence>\",\n");
        prompt
            .push_str("  \"recommendations\": [\"<recommendation 1>\", \"<recommendation 2>\"]\n");
        prompt.push_str("}\n");
        prompt.push_str("```\n\n");

        prompt.push_str("**CRITICAL REQUIREMENTS:**\n");
        prompt.push_str("1. Respond ONLY with valid JSON (no markdown code blocks)\n");
        prompt.push_str("2. Base assessment on provided intelligence data\n");
        prompt.push_str("3. Compare to previous assessment if available\n");
        prompt.push_str("4. Provide specific evidence and citations\n");
        prompt.push_str("5. Quantify uncertainty in confidence levels\n");
        prompt.push_str("6. Identify actionable early warning indicators\n\n");

        Ok(prompt)
    }

    /// Match data point to category
    fn matches_category(&self, category: &str, point: &crate::models::DataPoint) -> bool {
        let content_lower = point.content.to_lowercase();
        let title_lower = point
            .title
            .as_ref()
            .map(|t| t.to_lowercase())
            .unwrap_or_default();

        match category {
            "Nuclear Arsenal" => {
                content_lower.contains("warhead")
                    || content_lower.contains("icbm")
                    || content_lower.contains("nuclear arsenal")
                    || title_lower.contains("arsenal")
            }
            "Arms Control" => {
                content_lower.contains("treaty")
                    || content_lower.contains("arms control")
                    || content_lower.contains("start")
                    || content_lower.contains("nuclear agreement")
            }
            "Regional Conflicts" => {
                content_lower.contains("conflict")
                    || content_lower.contains("war")
                    || content_lower.contains("tension")
                    || content_lower.contains("crisis")
            }
            "Leadership/Rhetoric" => {
                content_lower.contains("president")
                    || content_lower.contains("threat")
                    || content_lower.contains("warning")
                    || title_lower.contains("statement")
            }
            "Technical Incidents" => {
                content_lower.contains("incident")
                    || content_lower.contains("accident")
                    || content_lower.contains("false alarm")
                    || content_lower.contains("malfunction")
            }
            "Communication" => {
                content_lower.contains("diplomatic")
                    || content_lower.contains("hotline")
                    || content_lower.contains("talks")
                    || content_lower.contains("negotiation")
            }
            "Emerging Tech" => {
                content_lower.contains("hypersonic")
                    || content_lower.contains("ai weapon")
                    || content_lower.contains("cyber")
                    || content_lower.contains("space weapon")
            }
            "Economic Factors" => {
                content_lower.contains("sanction")
                    || content_lower.contains("economic")
                    || content_lower.contains("trade war")
                    || content_lower.contains("embargo")
            }
            _ => false,
        }
    }

    /// Build delta explanation prompt
    pub fn build_delta_explanation_prompt(
        &self,
        current: &Assessment,
        previous: &Assessment,
    ) -> Result<String> {
        let mut prompt = String::new();

        prompt.push_str("# Risk Assessment Change Explanation\n\n");
        prompt.push_str("Please explain the changes between these two risk assessments:\n\n");

        prompt.push_str("## Previous Assessment\n");
        prompt.push_str(&format!(
            "- Date: {}\n",
            previous.assessment_date.format("%Y-%m-%d")
        ));
        prompt.push_str(&format!(
            "- Seconds to Midnight: {}\n",
            previous.seconds_to_midnight
        ));
        prompt.push_str(&format!("- Risk Level: {}\n\n", previous.risk_level()));

        prompt.push_str("## Current Assessment\n");
        prompt.push_str(&format!(
            "- Date: {}\n",
            current.assessment_date.format("%Y-%m-%d")
        ));
        prompt.push_str(&format!(
            "- Seconds to Midnight: {}\n",
            current.seconds_to_midnight
        ));
        prompt.push_str(&format!("- Risk Level: {}\n\n", current.risk_level()));

        if let Some(delta) = current.delta_from_previous {
            prompt.push_str(&format!("**Change**: {} seconds ", delta));
            if delta < 0 {
                prompt.push_str("(risk INCREASED)\n\n");
            } else {
                prompt.push_str("(risk DECREASED)\n\n");
            }
        }

        prompt.push_str("Please provide a clear, concise explanation of:\n");
        prompt.push_str("1. The primary drivers of this change\n");
        prompt.push_str("2. Specific events or developments that influenced the assessment\n");
        prompt.push_str(
            "3. Whether this change represents a significant shift or normal variation\n",
        );
        prompt.push_str("4. Any early warning indicators that emerged\n\n");

        prompt.push_str("Respond in 2-3 paragraphs suitable for an executive summary.\n");

        Ok(prompt)
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_prompt_builder_creation() {
        let builder = PromptBuilder::new();
        assert!(!builder.system_prompt().is_empty());
    }

    #[test]
    fn test_build_risk_assessment_prompt() {
        let builder = PromptBuilder::new();
        let data = AggregatedData {
            data_points: Vec::new(),
            collection_start: Utc::now(),
            collection_end: Utc::now(),
            sources_count: 5,
            failed_sources: Vec::new(),
            collection_duration: std::time::Duration::from_secs(10),
        };

        let prompt = builder.build_risk_assessment_prompt(&data, None);
        assert!(prompt.is_ok());
        let prompt_text = prompt.unwrap();
        assert!(prompt_text.contains("Nuclear Risk Assessment"));
        assert!(prompt_text.contains("JSON"));
    }

    #[test]
    fn test_category_matching() {
        let builder = PromptBuilder::new();
        let point = crate::models::DataPoint {
            id: uuid::Uuid::new_v4(),
            source: "test".to_string(),
            source_url: None,
            title: Some("Nuclear Warhead Test".to_string()),
            content: "Country conducts nuclear warhead test".to_string(),
            published_at: Some(Utc::now()),
            collected_at: Utc::now(),
            category: crate::types::DataCategory::NuclearArsenal,
            reliability: 0.9,
            metadata: std::collections::HashMap::new(),
        };

        assert!(builder.matches_category("Nuclear Arsenal", &point));
    }
}
