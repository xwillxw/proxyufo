use std::fs::{self, File};
use std::io::Write;
use std::vec::Vec;
use reqwest;
use tokio;

#[tokio::main]
pub async fn scrape_proxies() {

    let path = "./preset/http.txt";
    let mut http_list = Vec::new();
    let mut split_list;
    let mut ip_list = Vec::new();
    let mut file = File::create("./out/proxies.txt").unwrap();

    // load proxy sources from text file into vec
    for line in fs::read_to_string(path).unwrap().lines() {
        http_list.push(line.to_string());
    }
    
    // iterate through vec, scraping all proxies
    for url in http_list {

        // scrape page content as string
        let result = reqwest::get(url).await;

        // convert scrape to text
        let scraped_text_result = match result {
            Ok(text) => text
                .text()
                .await,
            Err(error) => panic!("Problem scraping proxies: {:?}", error)
        };

        // handle any errors
        let scraped_text = match scraped_text_result {
            Ok(string) => string,
            Err(error) => panic!("Problem scraping proxies: {:?}", error)
        };
            

        // split list into vec of strings
        split_list = scraped_text.split("\n");

        // add strings to buffer
        for part in split_list {
            let _ = writeln!(ip_list, "{}", part);
        }
    }

    // write buffer to file
    let _ = file.write_all(&ip_list);
    let _ = file.flush();    
}