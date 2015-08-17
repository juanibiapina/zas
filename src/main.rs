extern crate zas;

#[cfg(not(test))]
fn main() {
    let dns_server_thread = zas::dns::server::run();
    let http_server_thread = zas::http::server::run();

    dns_server_thread.join().unwrap();
    http_server_thread.join().unwrap();
}
