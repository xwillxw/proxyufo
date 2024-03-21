mod checker;
mod scraper;
use checker::check_proxies;
use scraper::scrape_proxies;

fn main() {

    scrape_proxies();
    check_proxies();
    
}
