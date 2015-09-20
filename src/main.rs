extern crate zas;

#[cfg(not(test))]
use std::process::exit;

#[cfg(not(test))]
use zas::config::Config;
#[cfg(not(test))]
use zas::error::Error;

#[cfg(not(test))]
fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            print_error(e);
            return;
        }
    };

    let dns_server = zas::dns::server::Server::create(&config);
    let http_server = zas::http::server::Server::create(&config);

    dns_server.thread.join().unwrap();
    http_server.thread.join().unwrap();
}

#[cfg(not(test))]
fn print_error(error: Error) {
    match error {
        Error::InvalidUserHome => {
            println!("Can't read user $HOME");
            exit(1);
        },
        Error::InvalidPort(port) => {
            println!("Invalid port: {}", port);
            exit(1);
        }
    }
}
