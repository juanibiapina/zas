extern crate hyper;
extern crate tokio_core;
extern crate futures;

use std::sync::{Arc};
use std::thread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use self::tokio_core::reactor::Core;
use self::tokio_core::net::TcpListener;
use self::hyper::server::Http;
use self::futures::stream::Stream;
use self::futures::Future;

use error::Error;
use config::Config;
use http::server::hyper::Chunk;
use http::dispatcher::Dispatcher;
use http::app_manager::AppManager;

pub struct Server {
    pub thread: thread::JoinHandle<()>,
}

impl Server {
    pub fn create(config: &Config) -> Result<Server, Error> {
        let app_manager = Arc::new(AppManager::new()?);

        let thread = Server::create_thread(app_manager, config.http_port);

        Ok(Server {
            thread: thread,
        })
    }

    fn create_thread(app_manager: Arc<AppManager>, port: u16) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let http = Http::<Chunk>::new();
            let mut core = Core::new().unwrap();
            let handle = core.handle();

            let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

            let listener = TcpListener::bind(&address, &handle).unwrap();

            let server = listener.incoming().for_each(|(tcp_stream, _)| {
                let service = Dispatcher::new(app_manager.clone(), handle.clone());
                let conn = http.serve_connection(tcp_stream, service);
                let fut = conn
                    .map(|_| ())
                    .map_err(|e| eprintln!("server connection error: {}", e));
                handle.spawn(fut);
                Ok(())
            });

            core.run(server).unwrap();
        })
    }
}

