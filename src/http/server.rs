extern crate hyper;

use std::thread;

use self::hyper::server::Server;

use http::dispatcher::Dispatcher;

pub fn run() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let dispatcher = Dispatcher::new();
        Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
    })
}
