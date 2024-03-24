mod proxy;
use proxy::Proxy;


fn main() {

    Proxy::scrape();
    Proxy::check();
    
}
