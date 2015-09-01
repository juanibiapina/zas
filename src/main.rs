extern crate zas;

use zas::config::Config;

#[cfg(not(test))]
fn main() {
    let config = Config::new();

    let dns_server = zas::dns::server::Server::create(&config);
    let http_server = zas::http::server::Server::create(&config);

    dns_server.thread.join().unwrap();
    http_server.thread.join().unwrap();
}
