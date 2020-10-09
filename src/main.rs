mod cache;

use cache::{
    Cache,
    cache_metrics::{CacheMetrics},
    hash_cache::{HashCache}
};

use std::{thread, time};

fn main() {
    let mut cache = CacheMetrics::with(HashCache::new());
    let key = 5;
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    do_something(&mut cache, key);
    println!("{}", cache.get(&key).unwrap());
    println!("hits: {}, misses: {}", cache.hits(), cache.misses());
}

fn do_something_expensive(key: &u32) -> String {
    println!("hit cache");
    let ten_millis = time::Duration::from_millis(1000);

    thread::sleep(ten_millis);
    format!("poop {}", key)
}

fn do_something(cache: &mut impl Cache<u32, String>, key: u32) {
    cache.insert_if_missing(&key, do_something_expensive);
}