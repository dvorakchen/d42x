pub trait Cache<TKey, TValue> {
    /// insert a value by key, return the old value if already existed
    fn insert(&self, key: TKey, value: TValue) -> Option<TValue>;
    /// get value by key
    fn get(&self, key: &TKey) -> Option<&TValue>;
    /// remove by key, return the value
    fn remove(&self, key: &TKey) -> Option<TValue>;
    /// remove all cached value
    fn clear(&self);
}

pub struct MokaCache;

impl Cache<&'static str, String> for MokaCache {
    fn insert(&self, key: &'static str, value: String) -> Option<String> {
        None
    }

    fn get(&self, key: &&'static str) -> Option<&String> {
        None
    }

    fn remove(&self, key: &&'static str) -> Option<String> {
        None
    }

    fn clear(&self) {}
}
