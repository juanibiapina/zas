extern crate zas;

#[cfg(not(test))]
use std::process::exit;
#[cfg(not(test))]
use std::error::Error as StdError;

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
        }
    };

    let dns_server = zas::dns::server::Server::create(&config);
    let http_server = match zas::http::server::Server::create(&config) {
        Ok(server) => server,
        Err(e) => {
            print_error(e);
        }
    };

    dns_server.thread.join().unwrap();
    http_server.thread.join().unwrap();
}

#[cfg(not(test))]
fn print_error(error: Error) -> ! {
    match error {
        Error::InvalidUserHome => {
            println!("Can't read user $HOME");
        },
        Error::InvalidPort(port) => {
            println!("Invalid port: {}", port);
        },
        Error::IoError(err) => {
            println!("IO Error: {}", err.description());
        },
        Error::XdgError(err) => {
            println!("IO Error: {}", err.description());
        },
        Error::ConfigDeserializationError(err) => {
            println!("IO Error: {}", err.description());
        },
        Error::AppNotConfigured => {
            println!("App not configured");
        }
    }

    exit(1);
}
