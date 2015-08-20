extern crate hyper;

use std::env;
use std::thread;

use self::hyper::server;

use http::dispatcher::Dispatcher;
use http::app::App;

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
            let app_home = env::var("ZAS_HOME").unwrap();

            let simple_app = App::new("simple", "12050", app_home);

            let dispatcher = Dispatcher::new(simple_app);

            server::Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
        })
    }
}

