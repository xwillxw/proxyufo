use std::fs::{self, File};
use std::io::Write;
use std::thread::{self, current, JoinHandle};
use std::time::Duration;
use futures::Future;
use reqwest::dns::Resolve;
use reqwest::{Client, Response, StatusCode};

#[derive(Debug, PartialEq)]
enum ProxyStatusCodes {
    Hit,
    Err
}

#[derive(PartialEq, Debug)]
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

        let mut checked_proxy_list: Vec<Proxy> = Vec::new();
        let mut checked_proxy_list_future: Vec<Future<()>> = Vec::new();
        let mut handle: JoinHandle<()>;

        // create client with timeout duration
        

        // send http head request from client, hit if http 200 ok
        for current_proxy in proxy_list {
            
            handle = thread::spawn(|| {
                let new_proxy = Proxy::send_request(Client::builder().timeout(Duration::from_millis(3000)).build().unwrap(), current_proxy);
                checked_proxy_list_future.push(new_proxy);
            });

            handle.join().unwrap();

        }    

        

        checked_proxy_list
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

    async fn send_request(client: Client, proxy: Proxy) -> Proxy {
        match client.head(&proxy.url).send().await {
            Ok(response) => {
                if response.status() == StatusCode::from_u16(200).unwrap() {
                    println!("HIT @ {}", &proxy.ip);
                    return Proxy::new(proxy.ip, proxy.url, ProxyStatusCodes::Hit);
                } else {
                    println!("No Hit, error code received: {}", response.status());
                    return Proxy::new(proxy.ip, proxy.url, ProxyStatusCodes::Err);
                }
            }
            Err(..) => {
                println!("No Hit!");
                return Proxy::new(proxy.ip, proxy.url, ProxyStatusCodes::Err);
            }
        }
    }

}