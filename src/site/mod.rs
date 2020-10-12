extern crate serde;
extern crate serde_xml_rs;
// use reqwest::Url;
// use std::str::FromStr;
#[derive(Deserialize, Debug)]
struct UrlSet {
    name: String,
    #[serde(rename="layer")]
    layers: Vec<Layer>,
}

#[derive(Deserialize, Debug)]
struct Layer {
    content_type: String,
    count: u8,
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
struct Data {
    id: u8,
    #[serde(rename="$value")]
    content: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Sitemap {
    pub url: String,
    pub name: String,
}

pub(crate) fn sitemap_format(domain: String) -> String {
    format!("{}/sitemap.xml", domain)
}

impl Sitemap {
    pub fn new(url: String) -> Sitemap {
        Sitemap { url: url, name: "".to_string() }
    }
}

pub(crate) struct Site { 
    domain: String, 
    urls: Vec<String> 
}

// This should follow the same convention as the cache
// where it's 
impl Site {
    pub fn new(url: &str, links: Vec<String>) -> Site {
        Site { domain: url.to_string(), urls: links }
    }

    pub fn build_sitemap(&self) {
        // let sitemap_url = format!("{}/sitemap.xml", key);
        // println!("request for {}", sitemap_url);
        // &self.cache.insert_if_missing(&key, );
        "fail".to_string();
    }

    // pub fn get_sitemap(&mut self) -> Sitemap {
    //     let domain = self.domain.to_string();
    //     let url = &sitemap_format(domain);
    //     Sitemap::new(url.to_string())
    // }
    // pub fn get_sitemap_url(&mut self) -> String {
    //     sitemap_format(self.domain.to_string())
    // }
}