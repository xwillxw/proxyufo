
use std::fs::{self, File};
use std::io::Write;
use std::thread::{self, current, JoinHandle};
use std::time::Duration;
use futures::Future;
use reqwest::blocking::{self, get};
use reqwest::{Client, Response, StatusCode};
use threadpool::ThreadPool;

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
    pub async fn scrape() -> Vec<Proxy> {

        let mut proxy_list: Vec<Proxy> = Vec::new();
        let http_in_path = "./preset/http.txt";
        let mut http_list = Vec::new();

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

        proxy_list
    }

    // proxy checker
    #[tokio::main]
    pub async fn check(proxy_list: Vec<Proxy>) -> Vec<Proxy> {

        let mut temp_proxy_list = proxy_list.clone();
        let mut handle: JoinHandle<()>;


        // thread pool setup

        let n_workers = 16;
        let pool = ThreadPool::new(n_workers);

        // create client with timeout duration
        
        for current_proxy in proxy_list {
;
            pool.execute(|| {
                //println!("Thread spawned!");
                current_proxy.send_request();
                //thread::sleep(Duration::from_millis(3000));
            });

        }

        pool.join();
                    
        // send http head request from client, hit if http 200 ok
           
        

        temp_proxy_list
    }    

    pub fn write_proxies(list: &Vec<Proxy>, name: String) {

        let mut out_path = match File::create("./out/".to_owned() + &name) {
            Ok(path) => path,
            Err(error) => panic!("Failed to parse file path: {:?}", error)
        };

        let mut buffer = Vec::new();

        for proxy in list {
            let _ = writeln!(buffer, "{}", proxy.ip);
        }

        // write buffer to file
        let _ = out_path.write_all(&buffer);
        let _ = out_path.flush();
    }

    fn send_request(self) {

        let client_request = match reqwest::blocking::ClientBuilder::new().timeout(Duration::from_millis(3000)).build() {
            Ok(client_request) => client_request.get(self.url),
            Err(error) => panic!("Error building client request: {}", error)
        };

        match client_request.send()  {
            Ok(response) => {
                if response.status() == StatusCode::from_u16(200).unwrap() {
                    println!("HIT @ {}", self.ip);
                } else {
                    println!("No Hit, error code received: {}", response.status());
                }
            }
            Err(..) => {
                println!("No Hit!");
            }
        }
    }

}

//.get(self.url)