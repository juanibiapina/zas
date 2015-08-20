extern crate hyper;

use std::env;
use std::thread;

use self::hyper::server::Server;

use http::dispatcher::Dispatcher;
use http::app::App;

pub fn run() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let app_home = env::var("ZAS_HOME").unwrap();

        let get_app = App::new("get", "12050", app_home);

        let dispatcher = Dispatcher::new(get_app);

        Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
    })
}
