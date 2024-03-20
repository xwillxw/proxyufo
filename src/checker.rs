use std::fs::{self, File};
use std::io::{BufWriter, Write};
use reqwest::{Client, Response};


//let _ = writeln!(checked_proxy_list, "{}", string);

#[tokio::main]
pub async fn check_proxies() -> std::io::Result<()>  {

    let in_path = "./out/proxies.txt";
    let mut out_path = BufWriter::new(File::create("./out/checked_proxies.txt")?);
    let mut proxy_list = Vec::new();
    let mut checked_proxy_list = Vec::new();
    let client = Client::new();

    for line in fs::read_to_string(in_path).unwrap().lines() {
        proxy_list.push(line.to_string());
    }

    for ip in proxy_list {

        // scrape page content as string
        let address = "http://".to_owned() + &ip;

        // build get request
        match client.get(address).build() {
            Ok(request) => {
                // execute request
                match client.execute(request).await {
                    Ok(response) => {

                        // convert response to text
                        match response.text().await {
                            Ok(response_text) => {

                                // add response to buffer
                                let _ = writeln!(checked_proxy_list, "{}", response_text);
                            } Err(..) => println!("No Hit")
                        }
                    } Err(..) => println!("No Hit")
                }
            } Err(..) => println!("No Hit")
        }

        println!("Next")
        
    }

    // write buffer to file
    let _ = out_path.write_all(&checked_proxy_list);
    let _ = out_path.flush();

    Ok(())
}