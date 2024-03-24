
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::time::Duration;
use futures::channel::oneshot::channel;
use futures::channel::{self, mpsc};
use reqwest::StatusCode;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

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
            ip: ip,
            url: url,
            status: status
        };
    }

    // proxy scraper
    #[tokio::main]
    pub async fn scrape() {

        let mut proxy_list: Vec<Proxy> = Vec::new();
        let http_in_path = "./preset/http.txt";
        let mut http_list = Vec::new();
        let mut out_path = match File::create("./out/proxies.txt".to_owned()) {
            Ok(path) => path,
            Err(error) => panic!("Failed to parse file path: {:?}", error)
        };
        let mut buffer = Vec::new();

        // load proxy sources from text file into vec
        for line in fs::read_to_string(http_in_path).unwrap().lines() {
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
        match out_path.write_all(&buffer) {
            Ok(write) => write,
            Err(error) => panic!("Error writing proxies.txt: {}", error)
        }

        match out_path.flush() {
            Ok(write) => write,
            Err(error) => panic!("Error flushing after scrape: {}", error)
        }

        println!("Proxies scraped");

    }

    // proxy checker
    #[tokio::main]
    pub async fn check() {

        let mut proxy_list: Vec<Proxy> = Vec::new();
        let http_proxies_in_path = "./out/proxies.txt";
        let mut http_output_file = match File::create("./out/http.txt".to_owned()) {
            Ok(path) => path,
            Err(error) => panic!("Failed to parse file path: {:?}", error)
        };
        let mut buffer = Vec::new();

        for line in fs::read_to_string(http_proxies_in_path).unwrap().lines() {
            proxy_list.push(Proxy::new(
                line.to_owned(),
                "http://".to_owned() + &line,
                ProxyStatusCodes::Err
            ))
        }

        // thread pool setup
        let n_workers = 200;
        let pool = ThreadPool::new(n_workers);
        let checked_proxy_list: Arc<Mutex<Vec<Proxy>>> = Arc::new(Mutex::new(Vec::new()));


        // create client with timeout duration
        for current_proxy in proxy_list { 

            let current_proxy_local = current_proxy.clone();
            let checked_proxy_list_local = Arc::clone(&checked_proxy_list);


            pool.execute(move || {


                let client_request = match reqwest::blocking::ClientBuilder::new().timeout(Duration::from_millis(3000)).build() {
                    Ok(client_request) => client_request.get(&current_proxy.url),
                    Err(error) => panic!("Error building client request: {}", error)
                };
        
                match client_request.send()  {
                    Ok(response) => {
                        if response.status() == StatusCode::from_u16(200).unwrap() {
                            println!("HIT @ {}", current_proxy.ip);
                            let mut checked_proxy_list_local = checked_proxy_list_local.lock().unwrap();
                            checked_proxy_list_local.push(current_proxy_local)
                        }
                    }
                    Err(..) => {}
                }
            }
        );}

        pool.join();

        let checked_proxy_list = checked_proxy_list.lock().unwrap();
        
        for proxy in checked_proxy_list.iter() {
            let _ = writeln!(buffer, "{}", proxy.ip);
        }

        // write buffer to file
        match http_output_file.write_all(&buffer) {
            Ok(write) => write,
            Err(error) => panic!("Error writing proxies.txt: {}", error)
        }

        match http_output_file.flush() {
            Ok(write) => write,
            Err(error) => panic!("Error flushing after scrape: {}", error)
        }

        println!("Proxies written!")
    }    
}