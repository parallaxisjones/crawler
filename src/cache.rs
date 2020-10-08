pub(crate) enum Insert
{
    AlreadyPresent,
    Inserted,
}

pub(crate) trait Cache<K,V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn insert_if_missing(
        &mut self, key: &K, creator: impl FnOnce(&K) -> V
    ) -> Insert {
        self.get_or_insert(key, creator);
        self.get(&key).unwrap();
        Insert::Inserted
    }

    fn get_or_insert(
        &mut self, key: &K, creator: impl FnOnce(&K) -> V
    ) -> &V;
}
