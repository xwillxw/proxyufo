mod proxy;
use proxy::Proxy;


fn main() {

    let unchecked_list_name = "proxies.txt".to_owned();
    let checked_list_name = "checked_proxies.txt".to_owned();

    let proxy_list = Proxy::scrape();
    Proxy::write_proxies(&proxy_list, unchecked_list_name);

    println!("Proxies scraped");

    let checked_proxy_list = Proxy::check(proxy_list);
    Proxy::write_proxies(&checked_proxy_list, checked_list_name);
    
}
