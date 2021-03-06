use crate::traits::KVStore;
use std::collections::{hash_map, HashMap};

use log::info;

/// An implementation for a key-value store (implements `KVStore`) store backed by an underlying `HashMap`
pub struct MemKVStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemKVStore {
    #[allow(clippy::new_without_default)]
    /// Initializes a new `MemKVStore`
    pub fn new() -> Self {
        info!("creating a new in-memory kv");

        Self {
            map: HashMap::new(),
        }
    }

    /// Clears the key-value store
    pub fn clear(&mut self) {
        info!("clearing in-memory kv");

        self.map.clear();
    }

    /// Returns an iterator for the internal `HashMap`
    pub fn iter(&self) -> hash_map::Iter<Vec<u8>, Vec<u8>> {
        (&self.map).iter()
    }

    /// Returns an iterator over the keys
    pub fn keys(&self) -> hash_map::Keys<Vec<u8>, Vec<u8>> {
        self.map.keys()
    }
}

impl KVStore for MemKVStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let entry = self.map.get(key);

        if let Some(entry) = entry {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        info!("storing in-memory kv changeset");

        for (k, v) in changes {
            self.map.insert(k.to_vec(), v.to_vec());
        }
    }
}
