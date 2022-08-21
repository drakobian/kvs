#![deny(missing_docs)]
//! A key-value store crate implementing the kvs
//! project from PingCAP's learning plan

use std::collections::HashMap;

/// a simple key-value store
/// mapping String keys to String values
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// ```rust
    /// # use kvs::KvStore;
    ///
    /// # fn main() {
    ///     let mut kvs = KvStore::new();
    /// # }
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets a key in the store to the given value
    /// ```rust
    /// # use kvs::KvStore;
    ///
    /// # fn main() {
    ///     let mut kvs = KvStore::new();
    ///     kvs.set("my_cool_key".into(), "my_cool_value".into());
    /// # }
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets a value in the store from the given key
    /// ```rust
    /// # use kvs::KvStore;
    ///
    /// # fn main() {
    ///     let mut kvs = KvStore::new();
    ///     kvs.get("my_cool_key".into());
    /// # }
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes a value from the store at the given key
    /// Does nothing if the given key is not in the store
    /// ```rust
    /// # use kvs::KvStore;
    ///
    /// # fn main() {
    ///     let mut kvs = KvStore::new();
    ///     kvs.remove("my_cool_key".into());
    /// # }
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
