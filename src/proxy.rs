use async_std::fs::{self, File};
use std::time::Duration;
use reqwest::StatusCode;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use async_std::io::BufWriter;
use async_std::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum ProxyStatusCodes {
    Hit,
    Err
}

#[derive(PartialEq, Debug, Clone)]
pub struct Proxy {
    ip: String,
    url: String,
    status: ProxyStatusCodes
}

impl Proxy{

    // Constructor
    pub fn new(ip: String, url: String, status: ProxyStatusCodes) -> Proxy {
        return Proxy {
            ip,
            url,
            status
        };
    }

    // proxy scraper
    #[tokio::main]
    pub async fn scrape() {

        let http_in_path = "./preset/http.txt";

        let mut out_path = match fs::File::create("./out/proxies.txt".to_owned()).await {
            Ok(path) => BufWriter::new(path),
            Err(error) => panic!("Failed to parse file path: {:?}", error)
        };

        let mut buffer = Vec::new();
        
        let file_content = match fs::read_to_string(http_in_path).await {
            Ok(content) => content,
            Err(error) => panic!("Error reading file {:?}", error),
        };

        let mut http_list = Vec::new();
        let mut proxy_list: Vec<Proxy> = Vec::new();

        // load proxy sources from text file into vec
        for line in file_content.lines() {
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
            let split_list = scraped_text.split("\n");

            // push proxies to scraped list
            for current_ip in split_list {
                proxy_list.push(Proxy::new(
                    current_ip.to_owned(),
                    "http://".to_owned() + &current_ip,
                    ProxyStatusCodes::Err
                ))
            }
            
        }

        for proxy in proxy_list {
            let _ = writeln!(buffer, "{}", proxy.ip);
        }

        // write buffer to file
        match out_path.write_all(&buffer).await {
            Ok(write) => write,
            Err(error) => panic!("Error writing proxies.txt: {}", error)
        }

        match out_path.flush().await {
            Ok(write) => write,
            Err(error) => panic!("Error flushing after scrape: {}", error)
        }

        println!("Proxies scraped");

    }

    // proxy checker
    #[tokio::main]
    pub async fn check() {

        let http_proxies_in_path = "./out/proxies.txt";
        let http_output_in_path = "./out/http.txt";

        let file_content = match fs::read_to_string(http_proxies_in_path).await {
            Ok(content) => content,
            Err(error) => panic!("Failed to read proxies file: {:?}", error),
        };
    
        let proxy_list: Vec<Proxy> = file_content
            .lines()
            .map(|line| Proxy::new(line.to_owned(), format!("http://{}", line), ProxyStatusCodes::Err))
            .collect();
        

        // thread pool setup
        let n_workers = 200;
        let pool = ThreadPool::new(n_workers);
        let checked_proxy_list: Arc<Mutex<Vec<Proxy>>> = Arc::new(Mutex::new(Vec::new()));


        // create client with timeout duration
        for current_proxy in proxy_list { 
            println!("Checking proxy: {}", current_proxy.ip);
            let mut current_proxy_local = current_proxy.clone();
            let checked_proxy_list_local = Arc::clone(&checked_proxy_list);

            pool.execute(move || {
                let client_request = match reqwest::blocking::ClientBuilder::new()
                    .timeout(Duration::from_millis(3000))
                    .build() 
                {
                    Ok(client_request) => client_request.get(&current_proxy.url),
                    Err(error) => panic!("Error building client request: {}", error)
                };
            
                match client_request.send()  {
                    Ok(response) => {
                        if response.status() == StatusCode::from_u16(200).unwrap() {
                            println!("HIT @ {}", current_proxy.ip);
                            current_proxy_local.status = ProxyStatusCodes::Hit;
                        }
                    }
                    Err(error) => {
                        println!("Error with the request: {:?}", error);
            
                        if error.is_timeout() {
                            print!("Request Timed Out!!");
                        }
                    }
                }
            
                let mut checked_proxy_list_local = checked_proxy_list_local.lock().unwrap();
                checked_proxy_list_local.push(current_proxy_local);
            });
        }   

        pool.join();


        let checked_proxy_list = checked_proxy_list.lock().unwrap();
        
        let mut buffer = Vec::new();
        for proxy in checked_proxy_list.iter() {
            let status_msg = match proxy.status {
                ProxyStatusCodes::Hit => "HIT",
                ProxyStatusCodes::Err => "BAD",
            };
            let _ = writeln!(buffer, "{} @ {}", status_msg, proxy.ip);
        }

        
        let mut http_output_file = match File::create(http_output_in_path).await {
            Ok(file) => file,
            Err(error) => panic!("Failed to create file: {:?}", error),
        };
    
        if let Err(error) = http_output_file.write_all(&buffer).await {
            panic!("Error writing to file: {:?}", error);
        }
        
        if let Err(error) = http_output_file.flush().await {
            panic!("Error flushing after check: {:?}", error);
        }
    
        println!("Proxies checked and written to file!")
    }    
}
