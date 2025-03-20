use std::time::Duration;

pub trait Cache<TKey, TValue> {
    /// insert a value by key, return the old value if already existed
    fn insert(&self, key: TKey, value: TValue) -> Option<TValue>;
    /// get value by key
    fn get<'a>(&self, key: &'a TKey) -> Option<TValue>;
    /// remove by key, return the value
    fn remove<'a>(&self, key: &'a TKey) -> Option<TValue>;
    /// remove all cached value
    fn clear(&self);
}

const DEFAULT_CACHE_LENGTH: u64 = 1_000;

pub struct MokaCache {
    cache: moka::sync::Cache<String, String>,
}

impl MokaCache {
    pub fn new() -> Self {
        let cache = moka::sync::Cache::builder()
            .max_capacity(DEFAULT_CACHE_LENGTH)
            .time_to_live(Duration::from_secs(30 * 60))
            .build();

        Self { cache }
    }
}

impl Cache<String, String> for MokaCache {
    fn insert(&self, key: String, value: String) -> Option<String> {
        let pre = self.cache.get(&key);
        self.cache.insert(key, value);
        pre
    }

    fn get<'a>(&self, key: &'a String) -> Option<String> {
        self.cache.get(key)
    }

    fn remove<'a>(&self, key: &'a String) -> Option<String> {
        let pre = self.cache.get(key);
        self.cache.invalidate(key);
        pre
    }

    fn clear(&self) {
        self.cache.invalidate_all();
    }
}
