mod site;
#[macro_use]
extern crate serde_derive;
extern crate sitemap;
extern crate reqwest;
extern crate roxmltree;

mod cache;
mod commands;
// use reqwest::{IntoUrl};
// use futures::io::Error;
// use tokio::task::JoinHandle;
// use futures::future::join_all;

use crate::commands::Opt;
use crate::site::sitemap_format;
use crate::site::Site;
use structopt::StructOpt;
use cache::{
    Cache,
    cache_metrics::{CacheMetrics},
    hash_cache::{HashCache}
};
// use site::Sitemap;
// use futures::{stream, StreamExt}; // 0.3.5
// use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]

// const PARALLEL_REQUESTS: usize = 2;
// use std::{thread, time};
// use std::fs::File;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app(opt: Opt, cache: &mut impl Cache<String, Site>) -> Result<()> {
    for path in opt.url {
        // Copy each path into a new string
        // that can be consumed/captured by the task closure
        let path = path.clone();
        let sitemap_url = &sitemap_format(path.clone());

        match reqwest::get(sitemap_url).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        cache.get_or_insert(&sitemap_url, move |k| {
                            let mut urls: Vec<String> = Vec::new();
                            let doc = roxmltree::Document::parse(&text).unwrap();
                            for node in doc.descendants() {
                                //todo: #1 flush this out so we capture the whole sitemap url object
                                if node.is_element() && node.has_tag_name("loc") {
                                    let link = node.text().unwrap();
                                    urls.push(link.to_string());
                                    println!("{:?} at {}", node.tag_name(), link);
                                }
                            }
                            println!("RESPONSE: {} bytes from {}", text.len(), path);
                            let s = Site::new(&k, urls);
                        //todo: this should take a whole representation of a url object
                            s
                        });
                        // jf_site.build_sitemap();
                    }
                    Err(_) => println!("ERROR reading {}", path),
                }
            },
            Err(_) => println!("ERROR downloading {}", path),
        }
    }
    Ok(())
}

fn main() {

    let opt = commands::Opt::from_args();
    println!("{:#?}", opt);
    let mut cache = CacheMetrics::with(HashCache::new());
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(app(opt, &mut cache)) {
        Ok(_) => println!("OK"),
        Err(e) => println!("err: {}", e),
    };
}
