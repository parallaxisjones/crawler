use crate::cache::Insert;
use crate::cache::Cache;
use std::collections::HashMap;
use core::hash::Hash;

pub(crate) struct HashCache<K: Hash + Eq, V> {
    map: HashMap<K, V>,
}

impl<K: Hash + Eq, V> HashCache<K, V> {
    pub(crate) fn new() -> HashCache<K, V> {
        HashCache { map: HashMap::new() }
    }
}

impl<K, V> Cache<K, V> for HashCache<K, V> 
where K: Hash + Eq + Clone, {
    fn get(&self, key: &K) -> Option<&V> { self.map.get(key) }

    fn get_or_insert(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> &V {
        match self.insert_if_missing(&key, creator) {
            _ => self.map.get(&key).unwrap(),
        }
    }
    fn insert_if_missing(&mut self, key: &K, creator: impl FnOnce(&K) -> V) -> Insert {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), creator(&key));
            Insert::AlreadyPresent
        } else {
            Insert::Inserted
        }
    }
}