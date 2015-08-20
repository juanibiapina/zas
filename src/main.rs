extern crate zas;

#[cfg(not(test))]
fn main() {
    let dns_server = zas::dns::server::Server::create();
    let http_server = zas::http::server::Server::create();

    dns_server.thread.join().unwrap();
    http_server.thread.join().unwrap();
}
