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
use crate::site::Site;
use structopt::StructOpt;
// use cache::{
//     Cache,
//     cache_metrics::{CacheMetrics},
//     hash_cache::{HashCache}
// };
// use site::Sitemap;
// use futures::{stream, StreamExt}; // 0.3.5
// use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]

// const PARALLEL_REQUESTS: usize = 2;
// use std::{thread, time};
// use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = commands::Opt::from_args();
    println!("{:#?}", opt);
    // let mut cache = CacheMetrics::with(HashCache::new());
    for path in opt.url {
        // Copy each path into a new string
        // that can be consumed/captured by the task closure
        let path = path.clone();
        let mut site = Site::new(&path);
        // println!("{:#?}", path);
        match reqwest::get(&site.get_sitemap_url()).await {
            Ok(resp) => {

                match resp.text().await {
                    Ok(text) => {
                        let doc = roxmltree::Document::parse(&text).unwrap();
                        for node in doc.descendants() {
                            
                            if node.is_element() && node.has_tag_name("loc") {
                                println!("{:?} at {}", node.tag_name(), node.text().unwrap());
                            }
                        }
                        println!("RESPONSE: {} bytes from {}", text.len(), path);

                    }
                    Err(_) => println!("ERROR reading {}", path),
                }
            },
            Err(_) => println!("ERROR downloading {}", path),
        }
    }

    // Wait for them all to finish
    // println!("Started {} tasks. Waiting...", tasks.len());
    // join_all(tasks).await;
    // let urls = opt.url;

    // let bodies = stream::iter(urls)
    //     .map(|url| sitemap_spawn(&url));

    Ok(())
}

// fn do_something(cache: &mut impl Cache<String, Sitemap>, key: String) {
//     let sitemap_url = format!("{}/sitemap.xml", key);
//     println!("request for {}", sitemap_url);
//     let res = sitemap_spawn(sitemap_url);
//     // cache.insert_if_missing(&key, );
//     "fail".to_string();
// }

// fn sitemap_spawn(url: impl IntoUrl) -> Result<Sitemap, Box<dyn std::error::Error>> {
//     // let client = Client::new();
//     // let response = client.get(url).send();
//     // match response {
//     //     Ok(response) => {}
//     // }
//     // let expensive_closure = |res: &str| {
//     //     println!("calculating slowly..., {}", res);
//     // };
//     Ok(Sitemap::new("".to_string()))
// } 
// fn do_something_expensive(key: &u32) -> String {
//     println!("hit cache");
//     let ten_millis = time::Duration::from_millis(1000);

//     thread::sleep(ten_millis);
//     format!("poop {}", key)
// }

// fn do_something(cache: &mut impl Cache<u32, String>, key: u32) {
//     cache.insert_if_missing(&key, do_something_expensive);
// }

// use sitemap::reader::{SiteMapReader,SiteMapEntity};
// use std::fs::File;