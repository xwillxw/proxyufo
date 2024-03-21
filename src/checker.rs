use std::fs::{self, File};
use std::io::Write;
use std::time::Duration;
use crossterm::terminal::Clear;
use reqwest::{Client, Proxy, StatusCode};

#[derive(PartialEq, Debug)]
enum ProxyStatusCodes {
    Hit,
    Err,
    Nul,
}

struct ProxyTest {
    ip: String,
    url: String,
    status: ProxyStatusCodes,
}

//let _ = writeln!(checked_proxy_list, "{}", string);

impl ProxyTest{
    async fn check(&mut self) {
        match reqwest::Proxy::http(&self.url) {
            Ok(result) => {
                self.build_client(result).await;
            }
            Err(error) => {
                println!("Error setting proxy:, {}", error)
            }
            }
        }
    
    async fn build_client(&mut self, result: Proxy) {
        let timeout_duration = Duration::from_millis(3000);
        let client_result = Client::builder()
            .proxy(result)
            .timeout(timeout_duration)
            .build();

        //println!("{:?}",client_result);
        //println!("{:?}", self.status);

        match client_result {
            Ok(client) => {
                if self.send_request(client).await == StatusCode::from_u16(200).unwrap() {
                    self.status = ProxyStatusCodes::Hit;
                    
                }
                else {
                    self.status = ProxyStatusCodes::Err;
                }
            }
            Err(error) => {
                println!("Error building client: {}", error);
            }
        }

       // println!("{:?}", self.status);
    }
    
    async fn send_request(&self, client: Client) -> StatusCode {    
        match client.get(&self.url).send().await {
            Ok(response) => {
                //println!("{:?}", response);
                response.status()
            }
            Err(..) => {
                return StatusCode::from_u16(401).unwrap();
            }
        }
    }
}



#[tokio::main]
pub async fn check_proxies() {

    let in_path = "./out/proxies.txt";
    let mut out_path = File::create("./out/checked_proxies.txt").unwrap();
    let mut proxy_list = Vec::new();
    let mut checked_proxy_list = Vec::new();

    for line in fs::read_to_string(in_path).unwrap().lines() {
        proxy_list.push(line.to_string());
    }

    for ip in proxy_list {

        let current_address = "http://".to_owned() + &ip;

        let mut current_proxy = ProxyTest{
            ip: ip,
            url: current_address,
            status: ProxyStatusCodes::Nul,
        };
        
        current_proxy.check().await;
        if current_proxy.status == ProxyStatusCodes::Hit {
            println!("HIT @ {}", current_proxy.ip);
            let _ = writeln!(checked_proxy_list, "{}", current_proxy.ip);
        } else if current_proxy.status == ProxyStatusCodes::Err {
            println!("No Hit")
        } else {
            println!("Error")
        }
        

    // write buffer to file
    let _ = out_path.write_all(&checked_proxy_list);
    let _ = out_path.flush();
    }
}