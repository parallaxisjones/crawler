mod site;
#[macro_use]
extern crate serde_derive;
extern crate sitemap;
extern crate reqwest;
extern crate roxmltree;
extern crate url;

mod cache;
mod commands;
use structopt::StructOpt;
use cache::{
    cache_metrics::{CacheMetrics},
    hash_cache::{HashCache}
};
use tokio;

fn main() {

    let opt = commands::Opt::from_args();
    println!("{:#?}", opt);
    let mut cache = CacheMetrics::with(HashCache::new());
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(site::app(opt, &mut cache)) {
        Ok(_) => println!("OK"),
        Err(e) => println!("err: {}", e),
    };
}
