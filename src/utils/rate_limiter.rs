//! Rate limiter implementation using token bucket algorithm.

use parking_lot::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limit configuration
#[derive(Debug, Clone, Copy)]
pub struct RateLimit {
    /// Maximum tokens (requests) per period
    pub max_tokens: u32,
    /// Period duration
    pub period: Duration,
}

impl RateLimit {
    /// Create a new rate limit
    pub fn new(max_tokens: u32, period: Duration) -> Self {
        Self { max_tokens, period }
    }

    /// Create a per-minute rate limit
    pub fn per_minute(tokens: u32) -> Self {
        Self::new(tokens, Duration::from_secs(60))
    }

    /// Create a per-hour rate limit
    pub fn per_hour(tokens: u32) -> Self {
        Self::new(tokens, Duration::from_secs(3600))
    }
}

/// Token bucket state
#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    config: RateLimit,
}

impl TokenBucket {
    fn new(config: RateLimit) -> Self {
        Self {
            tokens: config.max_tokens as f64,
            last_refill: Instant::now(),
            config,
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);

        if elapsed >= self.config.period {
            // Full refill
            self.tokens = self.config.max_tokens as f64;
            self.last_refill = now;
        } else {
            // Partial refill based on time elapsed
            let refill_rate = self.config.max_tokens as f64 / self.config.period.as_secs_f64();
            let tokens_to_add = refill_rate * elapsed.as_secs_f64();
            self.tokens = (self.tokens + tokens_to_add).min(self.config.max_tokens as f64);
            self.last_refill = now;
        }
    }

    fn try_consume(&mut self, count: f64) -> bool {
        self.refill();

        if self.tokens >= count {
            self.tokens -= count;
            true
        } else {
            false
        }
    }

    fn time_until_available(&self, count: f64) -> Duration {
        if self.tokens >= count {
            return Duration::from_secs(0);
        }

        let tokens_needed = count - self.tokens;
        let refill_rate = self.config.max_tokens as f64 / self.config.period.as_secs_f64();
        let seconds_needed = tokens_needed / refill_rate;

        Duration::from_secs_f64(seconds_needed)
    }
}

/// Rate limiter for managing multiple rate limits
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Register a rate limit for a resource
    pub fn register(&self, resource: String, limit: RateLimit) {
        let mut buckets = self.buckets.lock();
        buckets.insert(resource, TokenBucket::new(limit));
    }

    /// Try to acquire a token for a resource
    pub fn try_acquire(&self, resource: &str) -> bool {
        let mut buckets = self.buckets.lock();

        if let Some(bucket) = buckets.get_mut(resource) {
            bucket.try_consume(1.0)
        } else {
            // No rate limit configured, allow by default
            true
        }
    }

    /// Wait until a token is available for a resource
    pub async fn acquire(&self, resource: &str) {
        loop {
            let wait_duration = {
                let mut buckets = self.buckets.lock();

                if let Some(bucket) = buckets.get_mut(resource) {
                    if bucket.try_consume(1.0) {
                        return;
                    }
                    bucket.time_until_available(1.0)
                } else {
                    // No rate limit configured, proceed immediately
                    return;
                }
            };

            tokio::time::sleep(wait_duration).await;
        }
    }

    /// Get time until resource is available
    pub fn time_until_available(&self, resource: &str) -> Option<Duration> {
        let buckets = self.buckets.lock();
        buckets.get(resource).map(|b| b.time_until_available(1.0))
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_creation() {
        let limit = RateLimit::per_minute(60);
        assert_eq!(limit.max_tokens, 60);
        assert_eq!(limit.period, Duration::from_secs(60));
    }

    #[test]
    fn test_token_bucket_consume() {
        let config = RateLimit::new(10, Duration::from_secs(1));
        let mut bucket = TokenBucket::new(config);

        assert!(bucket.try_consume(1.0));
        assert!(bucket.try_consume(5.0));
        assert!((bucket.tokens - 4.0).abs() < 0.01); // Float comparison with tolerance
    }

    #[test]
    fn test_token_bucket_overflow() {
        let config = RateLimit::new(10, Duration::from_secs(1));
        let mut bucket = TokenBucket::new(config);

        assert!(!bucket.try_consume(11.0));
        assert_eq!(bucket.tokens, 10.0);
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let limiter = RateLimiter::new();
        limiter.register(
            "test".to_string(),
            RateLimit::new(2, Duration::from_millis(100)),
        );

        assert!(limiter.try_acquire("test"));
        assert!(limiter.try_acquire("test"));
        assert!(!limiter.try_acquire("test"));

        tokio::time::sleep(Duration::from_millis(150)).await;

        assert!(limiter.try_acquire("test"));
    }
}
