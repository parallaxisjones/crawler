extern crate serde;
extern crate serde_xml_rs;
extern crate sitemap;
// use reqwest::Url;
// use std::str::FromStr;

use sitemap::structs::UrlEntry;
// use sitemap::structs::{ChangeFreq,LastMod};
use crate::cache::Cache;
use crate::commands::Opt;

#[derive(Debug)]
pub(crate) struct Sitemap {
    entries: Vec<UrlEntry>,
}

pub(crate) fn sitemap_format(domain: String) -> String {
    format!("{}/sitemap.xml", domain)
}

impl Sitemap {
    // pub fn get<Site: std::iter::FromIterator<Site>>(&mut self, map: impl Fn(String) -> Site) -> &Site {
    //     &self.entries.into_iter()
    // }
    pub fn new(urls: Vec<UrlEntry>) -> Sitemap {
        Sitemap { entries: urls }
    }
}
#[derive(Debug)]
pub(crate) struct Site { 
    sitemap: Sitemap 
}

// This should follow the same convention as the cache
// where it's 

impl Site {
    pub fn new(sitemap: Sitemap) -> Site {
        Site { sitemap }
    }
}
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) async fn app(opt: Opt, cache: &mut impl Cache<String, Site>) -> Result<()> {
    for path in opt.url {
        // Copy each path into a new string
        // that can be consumed/captured by the task closure
        let path = path.clone();
        let sitemap_url = &sitemap_format(path.clone());

        match reqwest::get(sitemap_url).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        cache.get_or_insert(&sitemap_url, move |_| {
                            let mut urls: Vec<UrlEntry> = Vec::new();
                            let doc = roxmltree::Document::parse(&text).unwrap();
                            for node in doc.descendants()
                            .filter(|n| { n.has_tag_name("url") }) {
                                for nn in node.descendants() {
                                    //todo: #1 flush this out so we capture the whole sitemap url object
                                    // if nn.is_element() && nn.has_tag_name("changefreq") {
                                    //     let link = nn.text().unwrap().to_string();
                                    //     url_link = url_link.changefreq(ChangeFreq::from(link));
                                    // }
                                    
                                    // if nn.is_element() && nn.has_tag_name("lastmod") {
                                    //     let link = nn.text().unwrap();
                                    //     let last_mod = LastMod::from(link.to_string());
                                    //     url_link = url_link.lastmod(last_mod.get_time().unwrap());
                                    // }
                                    if nn.is_element() && nn.has_tag_name("loc") {
                                        let link = nn.text().unwrap();
                                        let built = UrlEntry::builder()
                                        .loc(link.to_string())
                                        .build().unwrap();
                                        urls.push(built);
                                    }
                                }
                            }
                            let sitemap = Sitemap::new(urls);
                            let s = Site::new(sitemap);
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
        let cached = cache.get(sitemap_url).unwrap();
        println!("GOT FROM CACHE: {:?}", cached);
    }
    Ok(())
}