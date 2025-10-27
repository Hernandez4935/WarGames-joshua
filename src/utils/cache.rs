//! Caching implementation with TTL support.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// Timed cache entry
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

/// Thread-safe timed cache with TTL
pub struct TimedCache<K, V> {
    entries: RwLock<HashMap<K, CacheEntry<V>>>,
}

impl<K, V> TimedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new timed cache
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    /// Insert a value with TTL
    pub fn insert(&self, key: K, value: V, ttl: Duration) {
        let expires_at = Instant::now() + ttl;
        let entry = CacheEntry { value, expires_at };

        let mut entries = self.entries.write();
        entries.insert(key, entry);
    }

    /// Get a value if it exists and hasn't expired
    pub fn get(&self, key: &K) -> Option<V> {
        let entries = self.entries.read();

        if let Some(entry) = entries.get(key) {
            if Instant::now() < entry.expires_at {
                return Some(entry.value.clone());
            }
        }

        None
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) {
        let mut entries = self.entries.write();
        entries.remove(key);
    }

    /// Clear all entries
    pub fn clear(&self) {
        let mut entries = self.entries.write();
        entries.clear();
    }

    /// Clean up expired entries
    pub fn cleanup(&self) {
        let mut entries = self.entries.write();
        let now = Instant::now();
        entries.retain(|_, entry| now < entry.expires_at);
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        let entries = self.entries.read();
        entries.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        let entries = self.entries.read();
        entries.is_empty()
    }
}

impl<K, V> Default for TimedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_insert_and_get() {
        let cache = TimedCache::new();
        cache.insert("key1", "value1".to_string(), Duration::from_secs(1));

        let value = cache.get(&"key1");
        assert_eq!(value, Some("value1".to_string()));
    }

    #[test]
    fn test_cache_expiration() {
        let cache = TimedCache::new();
        cache.insert("key1", "value1".to_string(), Duration::from_millis(100));

        thread::sleep(Duration::from_millis(150));

        let value = cache.get(&"key1");
        assert_eq!(value, None);
    }

    #[test]
    fn test_cache_remove() {
        let cache = TimedCache::new();
        cache.insert("key1", "value1".to_string(), Duration::from_secs(1));

        cache.remove(&"key1");

        let value = cache.get(&"key1");
        assert_eq!(value, None);
    }

    #[test]
    fn test_cache_cleanup() {
        let cache = TimedCache::new();
        cache.insert("key1", "value1".to_string(), Duration::from_millis(100));
        cache.insert("key2", "value2".to_string(), Duration::from_secs(10));

        assert_eq!(cache.len(), 2);

        thread::sleep(Duration::from_millis(150));
        cache.cleanup();

        assert_eq!(cache.len(), 1);
    }
}
