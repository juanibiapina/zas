extern crate hyper;

use std::thread;

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
            let app_manager = AppManager::new();
            let dispatcher = Dispatcher::new(app_manager);

            server::Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
        })
    }
}

