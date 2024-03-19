use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::vec::Vec;
use reqwest;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let path = "./preset/http.txt";
    let mut http_list = Vec::new();
    let mut split_list;
    let mut ip_list = Vec::new();
    let mut file = BufWriter::new(File::create("proxies.txt")?);

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

        // split list into vec of strings
        split_list = result.split("\n");

        // add strings to buffer
        for part in split_list {
            let _ = writeln!(ip_list, "{}", part);
        }
    }

    // write buffer to file
    let _ = file.write_all(&ip_list);
    let _ = file.flush();

    Ok(())
    
}