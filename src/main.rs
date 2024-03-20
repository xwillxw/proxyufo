mod checker;
mod scraper;
use checker::check_proxies;
use scraper::scrape_proxies;

struct Proxy {
    ip: i32,
    port: i32,
    active: bool
}

fn main() {
    let _ = scrape_proxies();
    let _ = check_proxies();
}