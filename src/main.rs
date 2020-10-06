use std::io::Read;
use select::document::Document;
use select::predicate::Predicate;
use select::predicate::Name;
use std::collections::HashSet;
use reqwest::Url;
use std::path::Path;
use std::time::Instant;

fn has_extension(url: &&str) -> bool {
    Path::new(url).extension().is_none()
}

fn fetch_url(client: &reqwest::blocking::Client, url: &str) -> String {
    let mut res = client.get(url).send().unwrap();
    println!("Status for {}: {}", url, res.status());

    let mut body  = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn get_links_from_html(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Name("a").or(Name("link")))
        .filter_map(|n| n.attr("href"))
        .filter(has_extension)
        .filter_map(normalize_url)
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str) -> Option<String> {
    let new_url = Url::parse(url);
    match new_url {
        Ok(new_url) => {
            if new_url.has_host() && new_url.host_str().unwrap() == "ghost.rolisz.ro" {
                Some(url.to_string())
            } else {
                None
            }
        },
        Err(_e) => {
            // Relative urls are not parsed by Reqwest
            if url.starts_with('/') {
                Some(format!("https://rolisz.ro{}", url))
            } else {
                None
            }
        }
    }
}

fn main() {
    let now = Instant::now();
    // let (dbclient, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
    // .await
    // .unwrap();
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";

    let body = fetch_url(&client, origin_url);

    let mut visited = HashSet::new();
    visited.insert(origin_url.to_string());
    let found_urls = get_links_from_html(&body);
    let mut new_urls = found_urls
    	.difference(&visited)
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

    while !new_urls.is_empty() {
        let found_urls: HashSet<String> = new_urls.iter().map(|url| {
            let body = fetch_url(&client, url);
            let links = get_links_from_html(&body);
            println!("Visited: {} found {} links", url, links.len());
            links
        }).fold(HashSet::new(), |mut acc, x| {
                acc.extend(x);
                acc
        });
        visited.extend(new_urls);
        
        new_urls = found_urls
        	.difference(&visited)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        println!("New urls: {}", new_urls.len())
    }
    println!("URLs: {:#?}", found_urls);
    println!("{}", now.elapsed().as_secs());
}