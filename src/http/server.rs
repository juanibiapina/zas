extern crate hyper;

use std::env;
use std::thread;
use std::collections::HashMap;

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

            let mut apps = HashMap::new();
            apps.insert("simple".to_string(), App::new("simple", "12050", &app_home));
            apps.insert("other".to_string(), App::new("other", "12051", &app_home));

            let dispatcher = Dispatcher::new(apps);

            server::Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
        })
    }
}

