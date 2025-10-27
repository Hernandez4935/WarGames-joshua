//! Claude API request and response models.
//!
//! This module defines the structure for communicating with the Anthropic Claude API.
//! All types are based on the official Anthropic API specification.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Claude API message request
#[derive(Debug, Clone, Serialize)]
pub struct MessageRequest {
    /// Model to use for this request
    pub model: String,

    /// Maximum tokens to generate
    pub max_tokens: u32,

    /// Sampling temperature (0.0 to 1.0)
    pub temperature: f32,

    /// Top-p sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Top-k sampling parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// System prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Conversation messages
    pub messages: Vec<Message>,

    /// Stop sequences
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
}

impl MessageRequest {
    /// Create a new message request
    pub fn new(model: impl Into<String>, max_tokens: u32) -> Self {
        Self {
            model: model.into(),
            max_tokens,
            temperature: 0.3, // Default for consistency
            top_p: Some(0.95),
            top_k: None,
            system: None,
            messages: Vec::new(),
            stop_sequences: Vec::new(),
        }
    }

    /// Set system prompt
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Add a user message
    pub fn add_user_message(mut self, content: impl Into<String>) -> Self {
        self.messages.push(Message {
            role: "user".to_string(),
            content: content.into(),
        });
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    /// Estimate token count for this request (rough approximation)
    pub fn estimated_tokens(&self) -> u32 {
        let mut count = 0;

        if let Some(sys) = &self.system {
            count += (sys.len() / 4) as u32;
        }

        for msg in &self.messages {
            count += (msg.content.len() / 4) as u32;
        }

        count
    }
}

/// A single message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role (user or assistant)
    pub role: String,

    /// Message content
    pub content: String,
}

/// Claude API response
#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponse {
    /// Response ID
    pub id: String,

    /// Object type
    #[serde(rename = "type")]
    pub type_: String,

    /// Role of responder
    pub role: String,

    /// Content blocks
    pub content: Vec<ContentBlock>,

    /// Model used
    pub model: String,

    /// Stop reason
    pub stop_reason: Option<String>,

    /// Stop sequence that triggered the stop
    pub stop_sequence: Option<String>,

    /// Token usage statistics
    pub usage: Usage,

    /// Request latency (not from API, we add this)
    #[serde(skip)]
    pub latency: Option<Duration>,
}

impl MessageResponse {
    /// Extract text from response
    pub fn extract_text(&self) -> String {
        self.content
            .iter()
            .filter(|block| block.type_ == "text")
            .map(|block| block.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get total tokens used
    pub fn total_tokens(&self) -> u32 {
        self.usage.input_tokens + self.usage.output_tokens
    }

    /// Estimate cost in USD
    pub fn estimated_cost(&self) -> f64 {
        // Claude Sonnet 4 pricing
        const INPUT_COST_PER_MTK: f64 = 3.0;
        const OUTPUT_COST_PER_MTK: f64 = 15.0;

        let input_cost = (self.usage.input_tokens as f64 / 1_000_000.0) * INPUT_COST_PER_MTK;
        let output_cost = (self.usage.output_tokens as f64 / 1_000_000.0) * OUTPUT_COST_PER_MTK;

        input_cost + output_cost
    }
}

/// Content block in response
#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlock {
    /// Block type
    #[serde(rename = "type")]
    pub type_: String,

    /// Text content
    pub text: String,
}

/// Token usage statistics
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    /// Input tokens
    pub input_tokens: u32,

    /// Output tokens
    pub output_tokens: u32,
}

/// Error response from Claude API
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    /// Error type
    #[serde(rename = "type")]
    pub type_: String,

    /// Error details
    pub error: ErrorDetails,
}

/// Error details
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorDetails {
    /// Error type
    #[serde(rename = "type")]
    pub type_: String,

    /// Error message
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_request_builder() {
        let request = MessageRequest::new("claude-sonnet-4-20250514", 8000)
            .with_system("You are JOSHUA")
            .add_user_message("Analyze this data");

        assert_eq!(request.model, "claude-sonnet-4-20250514");
        assert_eq!(request.messages.len(), 1);
        assert!(request.system.is_some());
    }

    #[test]
    fn test_token_estimation() {
        let request = MessageRequest::new("test", 100).add_user_message("Hello world!");

        let tokens = request.estimated_tokens();
        assert!(tokens > 0);
    }
}
