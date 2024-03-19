mod checker;
mod scraper;
use checker::check_proxies;
use scraper::scrape_proxies;

fn main() {
    let _ = scrape_proxies();
    let _ = check_proxies();
}