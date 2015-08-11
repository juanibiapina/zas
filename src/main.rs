extern crate zas;

fn main() {
    let dns_server_thread = zas::dns::server::run();

    dns_server_thread.join().unwrap();
}
