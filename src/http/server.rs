extern crate hyper;

use std::str::FromStr;
use std::env;
use std::thread;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use self::hyper::server;

use http::dispatcher::Dispatcher;
use http::app_manager::AppManager;

pub struct Server {
    pub thread: thread::JoinHandle<()>,
}

impl Server {
    pub fn create() -> Server {
        let thread = Server::create_thread();

        Server{
            thread: thread,
        }
    }

    fn create_thread() -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let port = env::var("ZAS_HTTP_PORT").unwrap_or("12044".to_string()).parse::<u16>().unwrap();

            let app_manager = AppManager::new(port + 6);
            let dispatcher = Dispatcher::new(app_manager);

            server::Server::http(SocketAddrV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), port)).unwrap().handle_threads(dispatcher, 20).unwrap();
        })
    }
}

