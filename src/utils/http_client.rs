//! HTTP client with retry logic and rate limiting.

use crate::constants::{MAX_RETRY_ATTEMPTS, RETRY_DELAY_SECONDS};
use crate::prelude::*;
use reqwest::{Client, Response};
use std::time::Duration;
use tracing::{debug, warn};

/// Shared HTTP client with retry logic
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    max_retries: u32,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(format!(
                "{}/{}",
                crate::constants::APP_NAME,
                crate::constants::APP_VERSION
            ))
            .build()?;

        Ok(Self {
            client,
            max_retries: MAX_RETRY_ATTEMPTS,
        })
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .user_agent(format!(
                "{}/{}",
                crate::constants::APP_NAME,
                crate::constants::APP_VERSION
            ))
            .build()?;

        Ok(Self {
            client,
            max_retries: MAX_RETRY_ATTEMPTS,
        })
    }

    /// Get a URL with retry logic
    pub async fn get_with_retry(&self, url: &str) -> Result<Response> {
        let mut attempts = 0;

        loop {
            attempts += 1;

            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!(url = url, attempts = attempts, "HTTP GET succeeded");
                        return Ok(response);
                    } else if response.status().is_server_error() && attempts < self.max_retries {
                        // Retry on server errors
                        warn!(
                            url = url,
                            status = response.status().as_u16(),
                            attempt = attempts,
                            "Server error, retrying"
                        );
                        let delay = Duration::from_secs(RETRY_DELAY_SECONDS.pow(attempts));
                        tokio::time::sleep(delay).await;
                    } else {
                        // Client error or max retries reached
                        return Err(Error::Http(response.error_for_status().unwrap_err()));
                    }
                }
                Err(e) if attempts < self.max_retries => {
                    warn!(
                        url = url,
                        attempt = attempts,
                        error = %e,
                        "HTTP request failed, retrying"
                    );
                    let delay = Duration::from_secs(RETRY_DELAY_SECONDS.pow(attempts));
                    tokio::time::sleep(delay).await;
                }
                Err(e) => {
                    return Err(Error::Http(e));
                }
            }
        }
    }

    /// Get a URL and return text with retry logic
    pub async fn get_text(&self, url: &str) -> Result<String> {
        let response = self.get_with_retry(url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    /// Get a URL and return JSON with retry logic
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.get_with_retry(url).await?;
        let json = response.json().await?;
        Ok(json)
    }

    /// Get the underlying reqwest client
    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_client_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_http_client_with_timeout() {
        let client = HttpClient::with_timeout(Duration::from_secs(10));
        assert!(client.is_ok());
    }
}
