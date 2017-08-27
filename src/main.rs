extern crate zas;
#[macro_use]
extern crate clap;

use clap::{App, SubCommand};

use std::process::exit;
use std::error::Error as StdError;

use zas::config::Config;
use zas::error::Error;

fn main() {
    let matches = App::new("zas")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple router for local web development")
        .subcommand(SubCommand::with_name("install")
                    .about("install zas system wide hooks"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("install") {
        match zas::install::run_install() {
            Ok(()) => {},
            Err(e) => {
                print_error(e);
            }
        }
    } else {
        run_server();
    }
}

fn run_server() {
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
