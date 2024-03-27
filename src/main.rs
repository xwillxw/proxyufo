mod proxy;
use proxy::Proxy;

fn main() {
    Proxy::scrape(); // Remove .await here
    Proxy::check();  // Remove .await here
}
