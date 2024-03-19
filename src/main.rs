use std::fs;
use std::vec::Vec;
use reqwest;
use tokio;
use std::str::Split;

enum Protocol {
    HTTP,
    SOCKS4,
    SOCKS5
}

struct Proxy {
    ip: String,
    port: String,
    protocol: Protocol
}

#[tokio::main]
async fn main() {

    let path = "./preset/http.txt";

    // create empty list in which proxy sources are stored
    let mut http_list: Vec<String> = Vec::new();
    let mut ip_list: Vec<&str> = Vec::new();

    // load proxy sources from text file into vec
    for line in fs::read_to_string(path).unwrap().lines() {
        http_list.push(line.to_string());
    }

    // iterate through vec, scraping all proxies
    for url in http_list {
        
        // scrape page content as string
        let result = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let split_array = result.split("\n");

        for part in split_array {
            println!("{}", part);
        }

    }

}