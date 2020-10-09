pub mod cache_metrics;
pub mod hash_cache;

pub(crate) enum Insert
{
    AlreadyPresent,
    Inserted,
}

pub(crate) trait Cache<K,V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn insert_if_missing(
        &mut self, key: &K, creator: impl FnOnce(&K) -> V
    ) -> Insert;

    fn get_or_insert(
        &mut self, key: &K, creator: impl FnOnce(&K) -> V
    ) -> &V;
}
