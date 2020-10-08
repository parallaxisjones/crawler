    // let now = Instant::now();
    // let client = reqwest::blocking::Client::new();
    // let origin_url = "https://www.junglefriends.org/monkeys/";
    // let parser = SkeezParserKind::CRAWL("Crawl".to_string());
    // let command = Crawler {
    //     url: origin_url.to_string(),
    // };

    // let skeez = Skeez::new();
    // skeez.start(command);
// use core::borrow::Borrow;
// use std::io::Read;
// use select::document::Document;
// use select::predicate::Predicate;
// use select::predicate::Name;
// use std::collections::HashSet;
// use reqwest::Url;
// use std::path::Path;
// use std::time::Instant;
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
// const JF_DOMAIN: &str = "www.junglefriends.org";


// // scrape job
// #[derive(Debug)]
// struct Scrape<'a> {
//     name: &'a str,
//     url: &'a str
// }

// // application container
// struct Skeez {
//     client: reqwest::blocking::Client,
//     cache: HashSet<SkeezSession>,
//     startTime: Instant,
// }

// struct SessionOptions {
//     url: String,
// }

// impl Skeez {
//     fn new() -> Skeez {
//         let cache = HashSet::new();
//         Skeez {
//             client: reqwest::blocking::Client::new(),
//             cache: cache,
//             startTime: Instant::now()
//         }
//     }

//     fn getContent(&self, url: &str) -> String {
//         let mut res = self.client.get(url).send().unwrap();
//         let mut body = String::new();
//         println!("Status for {}: {}", url, res.status());
//         res.read_to_string(&mut body).unwrap();
//         body
//     }

//     fn create_session(&self, options: SessionOptions) -> SkeezSession {
//         let id = SkeezSession::calculate_hash(&options.url);
//         if self.cache.contains(&id) {
//             &self.cache.get(&id);
//         }
//         let session = SkeezSession::new(id, options);
//         &self.cache.insert(session);
        
//         session
//     }

//     fn get_session(&self, id: u64) -> Option<SkeezSession> {
//         if self.cache.contains(&id) {
//             Some(&self.cache.get(&id));
//         }
//         None;
//     }
//     fn getSessionDiff(&self, command: &mut dyn SkeezCommand, found_urls: HashSet<String>) -> HashSet<String> {
//         let diff = found_urls
//             .difference(&self.getSession(command).hits)
//             .map(|x| x.to_string())
//             .collect::<HashSet<String>>();
//         diff
//     }

//     fn start(&self, command: Box<dyn SkeezCommand>) -> Result<(), i32> {
//         let body = self.getContent(&command.get_url());
//         let found_urls = get_links_from_html(&body);
//         let session = self.getSession();
//         self.cache.insert(session);
//         let mut new_urls = self.getSessionDiff(command, found_urls);
        
//         while !new_urls.is_empty() {
//             let found_urls: HashSet<String> = new_urls.iter().map(|url| {
//                 let body = self.getContent(url);
//                 let links = get_links_from_html(&body);
//                 println!("Visited: {} found {} links", url, links.len());
//                 links
//         }).fold(HashSet::new(), |mut acc, x| {
//                 acc.extend(x);
//                 acc
//         });
//         session.hits.extend(new_urls);
        
//         new_urls = found_urls
//         	.difference(&session.hits)
//             .map(|x| x.to_string())
//             .collect::<HashSet<String>>();
//         println!("New urls: {}", new_urls.len())
//         }
//         println!("URLs: {:#?}", found_urls);
//         println!("{}", self.startTime.elapsed().as_secs());

//         Ok(for v in session.hits.iter()
//         .map(|v| (v)) {
//     println!("{}", v);
//         })
//     }
// }
// #[derive(Debug, Eq)]
// struct SkeezSession {
//     id: u64,
//     hits: HashSet<String>,
//     url: String,
//     name: String,
// }

// impl SkeezSession {
//     fn new(id: u64, options: SessionOptions) -> SkeezSession {
//         let session = SkeezSession {
//             id: id,
//             hits: HashSet::new(),
//             url: options.url,
//             name: "skeez session".to_string(),
//         };
//         session
//     }
//     fn calculate_hash<T: Hash>(t: &T) -> u64 {
//         let mut s = DefaultHasher::new();
//         t.hash(&mut s);
//         s.finish()
//     }
// }
// impl PartialEq for SkeezSession {
//     fn eq(&self, other: &SkeezSession) -> bool {
//         self.id == other.id
//     }
// }

// impl Borrow<u64> for SkeezSession {
//     fn borrow(&self) -> &u64 {
//         &self.id
//     }
// }

// impl Hash for SkeezSession {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//     }
// }

// // #[derive(Debug, PartialEq, Eq)]
// enum SkeezParserKind {
//     CRAWL(String),
// }

// trait SkeezParser {
//     fn has_extension(url: &&str) -> bool {
//         Path::new(url).extension().is_none()
//     }
        
//     fn parse(html: &str) -> HashSet<String> {
//         Document::from(html)
//             .find(Name("a").or(Name("link")))
//             .filter_map(|n| n.attr("href"))
//             .filter(has_extension)
//             .filter_map(normalize_url)
//             .collect::<HashSet<String>>()
//     }
    
//     fn normalize_url(url: &str) -> Option<String> {
//         let new_url = Url::parse(url);
//         match new_url {
//             Ok(new_url) => {
//                 if new_url.has_host() && new_url.host_str().unwrap() == JF_DOMAIN {
//                     Some(url.to_string())
//                 } else {
//                     println!("found: {}", url.to_string());
//                     None
//                 }
//             },
//             Err(_e) => {
//                 // Relative urls are not parsed by Reqwesti
//                 println!("{}", url.to_string());
//                 if url.starts_with('/') {
//                     Some(format!("https://{},{}", JF_DOMAIN, url))
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
// }

// // input command
// trait SkeezCommand {
//     fn get_url(&self) -> String;
//     fn get_kind(&self) -> SkeezParserKind;
// }

// #[derive(Debug)]
// struct Crawler {
//     url: String,
// }

// impl SkeezCommand for Crawler {
//     fn get_kind(&self) -> SkeezParserKind {
//         SkeezParserKind::CRAWL("Crawl".to_string())
//     }
//     fn get_url(&self) -> String {
//         self.url
//     }
// }

// impl Scrape<'_> {
//     fn new(url: &str) -> Scrape<'_> {
//         Scrape {
//             name: url,
//             url: url
//         }
//     }
// }

// // struct SkeezUrl<'a> {
// //     url: &'a str
// // }

// // impl<'a> SkeezUrl<'a> {
// //     fn has_extension(&self) -> bool {
// //         Path::new(&self.url).extension().is_none();
// //     }

// //     fn normalize(url: &str) -> Option<SkeezUrl<'a>> {
// //         let new_url = Url::parse(url);
// //         match new_url {
// //             Ok(new_url) => {
// //                 if new_url.has_host() && new_url.host_str().unwrap() == JF_DOMAIN {
// //                     Some(url.to_string())
// //                 } else {
// //                     println!("found: {}", url.to_string());
// //                     None
// //                 }
// //             },
// //             Err(_e) => {
// //                 // Relative urls are not parsed by Reqwesti
// //                 println!("{}", url.to_string());
// //                 if url.starts_with('/') {
// //                     Some(format!("https://{},{}", JF_DOMAIN, url))
// //                 } else {
// //                     None
// //                 }
// //             }
// //         }
// //     }
// // }

// fn has_extension(url: &&str) -> bool {
//     Path::new(url).extension().is_none()
// }

// fn fetch_url(client: &reqwest::blocking::Client, url: &str) -> String {
//     let mut res = client.get(url).send().unwrap();
//     println!("Status for {}: {}", url, res.status());

//     let mut body  = String::new();
//     res.read_to_string(&mut body).unwrap();
//     body
// }

// fn get_links_from_html(html: &str) -> HashSet<String> {
//     Document::from(html)
//         .find(Name("a").or(Name("link")))
//         .filter_map(|n| n.attr("href"))
//         .filter(has_extension)
//         .filter_map(normalize_url)
//         .collect::<HashSet<String>>()
// }

// fn normalize_url(url: &str) -> Option<String> {
//     let new_url = Url::parse(url);
//     match new_url {
//         Ok(new_url) => {
//             if new_url.has_host() && new_url.host_str().unwrap() == JF_DOMAIN {
//                 Some(url.to_string())
//             } else {
//                 println!("found: {}", url.to_string());
//                 None
//             }
//         },
//         Err(_e) => {
//             // Relative urls are not parsed by Reqwesti
//             println!("{}", url.to_string());
//             if url.starts_with('/') {
//                 Some(format!("https://{},{}", JF_DOMAIN, url))
//             } else {
//                 None
//             }
//         }
//     }
// }
