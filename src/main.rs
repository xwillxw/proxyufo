use std::fs;
use std::vec::Vec;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

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


async fn scrape(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {

    // create empty list in which proxy sources are stored
    let mut http_list: Vec<String> = Vec::new();

    // load proxy sources from text file into vec
    for line in fs::read_to_string(path).unwrap().lines() {
        http_list.push(line.to_string())
    }

    // iterate through vec, scraping all proxies
    for mut i in 0..http_list.len() {

         // Parse our URL and setup hyper client
        if let Ok(url) = http_list[i].parse::<hyper::Uri>() {

            // Get the host and the port
            let host = url.host().expect("uri has no host");
            let port = url.port_u16().unwrap_or(80);

            let address = format!("{}:{}", host, port);

            // open tcp connection with remote host
            let stream = TcpStream::connect(address).await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Create the Hyper client
            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {:?}", err);
                }
            });
        }
        i += 1;
    }

    Ok(())
}

fn main() {

    let x = scrape("./preset/http.txt");

}