use crate::CACHE_TTL;
use std::{
    collections::{hash_map::Entry, HashMap},
    mem,
    sync::Arc,
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

/// State used by the server.
///
/// This state is useful to cache renders, for instance, so that we don’t have to generate the same content over and
/// over.
///
/// From time to time, a scheduler will run an eviction job on the cache entries for those who have passed their TTLs.
#[derive(Clone)]
pub struct Cache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl Cache {
    /// Create a new state.
    pub fn new(ttl: Duration) -> Self {
        let cache = Arc::new(Mutex::new(HashMap::new()));
        Self { cache, ttl }
    }

    /// Get a cached entry, if any, or compute it and cache it.
    pub fn cache(&self, key: &str, gen: impl FnOnce() -> String) -> String {
        let current = {
            self.cache
                .lock()
                .expect("cache lock")
                .get(key)
                .map(|entry| entry.content.clone())
        };
        current.unwrap_or_else(|| {
            let content = gen();
            let _ = self.insert(key, content.clone());
            content
        })
    }

    /// Get a cached entry, if any, or compute it and cache it, if any.
    pub fn cache_if_any(&self, key: &str, gen: impl FnOnce() -> Option<String>) -> Option<String> {
        let current = {
            self.cache
                .lock()
                .expect("cache lock")
                .get(key)
                .map(|entry| entry.content.clone())
        };
        current.or_else(|| {
            let content = gen()?;
            let _ = self.insert(key, content.clone());
            Some(content)
        })
    }

    /// Insert or update some content.
    ///
    /// If the key doesn’t have any associated content in the cache, a new cache entry is created.
    /// If the key does already have associated content, the content is replaced and the old one returned.
    pub fn insert(&self, key: impl Into<String>, content: String) -> Option<String> {
        match self.cache.lock().expect("cache lock").entry(key.into()) {
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                let old = mem::replace(&mut entry.content, content);
                entry.last_update_time = Instant::now();

                Some(old)
            }

            Entry::Vacant(entry) => {
                entry.insert(CacheEntry::new(content));
                None
            }
        }
    }

    /// Evict cache entries that have passed their TTLs.
    fn evict_due_entries(&self) {
        let ttl = self.ttl;
        self.cache.lock().expect("cache lock").retain(|key, entry| {
            let retain = entry.last_update_time.elapsed() <= ttl;

            if !retain {
                log::debug!("evicting cache entry: {}", key);
            }

            retain
        });
    }

    /// Run a scheduled job that will evict cache entries from time to time.
    pub fn schedule_eviction(&self) {
        let cache = self.clone();
        thread::spawn(move || loop {
            log::debug!("running cache eviction…");
            cache.evict_due_entries();
            thread::sleep(CACHE_TTL);
        });
    }
}

/// Cache entry.
///
/// This cache entry contains the actual rendered content along with a TTL (Time To Live).
#[derive(Clone)]
pub struct CacheEntry {
    content: String,
    last_update_time: Instant,
}

impl CacheEntry {
    pub fn new(content: String) -> Self {
        let last_update_time = Instant::now();
        Self {
            content,
            last_update_time,
        }
    }
}
