use std::fs;
use std::vec::Vec;
use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
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

fn main() {

    let mut http_list: Vec<String> = Vec::new();

    for line in fs::read_to_string("./preset/http.txt").unwrap().lines() {
        http_list.push(line.to_string())
    }
    //println!("{}", http_list[0]);

}