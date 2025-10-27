//! Notification engine implementation.

use crate::prelude::*;
use async_trait::async_trait;

/// Notification to be sent
#[derive(Debug, Clone)]
pub struct Notification {
    /// Notification title
    pub title: String,

    /// Notification message
    pub message: String,

    /// Severity level
    pub severity: AlertLevel,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Associated assessment ID
    pub assessment_id: Option<Uuid>,
}

/// Trait for notification delivery
#[async_trait]
pub trait NotificationSender: Send + Sync {
    /// Send notification
    async fn send(&self, notification: &Notification) -> Result<()>;

    /// Notification channel name
    fn channel_name(&self) -> &str;

    /// Check if channel is available
    async fn is_available(&self) -> bool {
        true
    }
}

/// Notification engine
pub struct NotificationEngine {
    // TODO: Add senders
}

impl NotificationEngine {
    /// Create a new notification engine
    pub fn new() -> Self {
        Self {}
    }

    /// Send a notification through all available channels
    pub async fn send(&self, _notification: &Notification) -> Result<()> {
        // TODO: Implement notification sending
        Ok(())
    }
}

impl Default for NotificationEngine {
    fn default() -> Self {
        Self::new()
    }
}
