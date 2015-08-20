extern crate zas;

#[cfg(not(test))]
fn main() {
    let dns_server = zas::dns::server::Server::create();
    let http_server_thread = zas::http::server::run();

    dns_server.thread.join().unwrap();
    http_server_thread.join().unwrap();
}
