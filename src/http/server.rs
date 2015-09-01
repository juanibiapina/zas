extern crate hyper;

use std::str::FromStr;
use std::thread;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use self::hyper::server;

use config::Config;
use http::dispatcher::Dispatcher;
use http::app_manager::AppManager;

pub struct Server {
    pub thread: thread::JoinHandle<()>,
}

impl Server {
    pub fn create(config: &Config) -> Server {
        let app_manager = AppManager::new(config.http_port + 6, &config.app_dir, &config.log_dir);
        let dispatcher = Dispatcher::new(app_manager);

        let thread = Server::create_thread(dispatcher, config.http_port);

        Server{
            thread: thread,
        }
    }

    fn create_thread(dispatcher: Dispatcher, port: u16) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            server::Server::http(SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), port)).unwrap().handle_threads(dispatcher, 20).unwrap();
        })
    }
}

