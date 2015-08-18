extern crate hyper;

use std::thread;
use std::env;

use self::hyper::server::Server;

use http::dispatcher::Dispatcher;

pub fn run() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let app_home = env::var("ZAS_HOME").unwrap();

        let dispatcher = Dispatcher::new(app_home);
        Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
    })
}
