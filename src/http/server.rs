extern crate hyper;

use std::thread;
use std::collections::HashMap;

use self::hyper::server;

use http::dispatcher::Dispatcher;

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

            let apps = HashMap::new();

            let dispatcher = Dispatcher::new(apps);

            server::Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
        })
    }
}

