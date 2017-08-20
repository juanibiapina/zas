extern crate hyper;
extern crate tokio_core;
extern crate futures;
extern crate regex;

use std::sync::{Arc, Mutex};
use std::str::FromStr;

use self::regex::Regex;
use self::futures::Future;
use self::hyper::Uri;
use self::hyper::header::Host;
use self::hyper::header::ContentLength;
use self::hyper::header::Connection;
use self::hyper::server::{Request, Response, Service};
use self::tokio_core::reactor::{Handle};
use self::hyper::client;

use http::app_manager::AppManager;
use common::error::Error;

pub struct Dispatcher {
    app_manager: Arc<Mutex<AppManager>>,
    handle: Handle,
}

impl Dispatcher {
    pub fn new(app_manager: Arc<Mutex<AppManager>>, handle: Handle) -> Dispatcher {
        Dispatcher {
            app_manager: app_manager,
            handle: handle,
        }
    }

    fn extract_app_name(&self, request: &Request) -> String {
        let host = &request.headers().get::<Host>().unwrap().hostname();
        let host_parts = host.split(".").collect::<Vec<_>>();

        match host_parts.split_last() {
            Some((_, parts)) => {
                parts.join(".")
            },
            None => {
                panic!("zas: invalid Host header");
            },
        }
    }

    fn ensure_app_running(&self, app_name: &str) -> Result<u16, Error> {
        let mut app_manager = self.app_manager.lock().unwrap();

        app_manager.ensure_app_running(&app_name)
    }

    fn handle_zas_request(&self, request: Request) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let path = request.path();

        let mut app_manager = self.app_manager.lock().unwrap();

        let mut response = Response::new();

        let term_app_regex = Regex::new("/apps/([[:alpha:]]+)/term").unwrap();

        if term_app_regex.is_match(path) {
            let app_name = term_app_regex.captures(path).unwrap().get(1).unwrap().as_str();
            app_manager.term(app_name);

            response.set_body("OK");
        } else {
            let app_dir = &app_manager.app_dir;

            let result = format!("ZAS_APP_DIR: {}", app_dir);

            response.set_body(result);
        }

        return futures::future::ok(response).boxed();
    }

    fn handle_app_request(&self, request: Request, app_name: String) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let result = self.ensure_app_running(&app_name);

        let port = match result {
            Ok(value) => value,
            Err(_) => {
                return futures::future::ok(
                    Response::new()
                        .with_header(ContentLength("App not configured".len() as u64))
                        .with_body("App not configured")
                    ).boxed();
            },
        };

        let connection_header = match request.headers().get::<Connection>() {
            Some(value) => value.clone(),
            None => Connection::close(),
        };

        let app_url = format!("http://localhost:{}{}", port, request.path());
        let app_uri = Uri::from_str(&app_url).unwrap();

        let mut client_req = client::Request::new(request.method().clone(), app_uri);
        client_req.headers_mut().extend(request.headers().iter());
        client_req.headers_mut().set(Connection::close());
        client_req.set_body(request.body());

        let client = hyper::Client::configure()
            .keep_alive(false)
            .build(&self.handle);

        let resp = client.request(client_req)
                         .then(move |result| {
                             match result {
                                 Ok(client_resp) => {
                                     Ok(Response::new()
                                            .with_status(client_resp.status())
                                            .with_headers(client_resp.headers().clone())
                                            .with_header(connection_header)
                                            .with_body(client_resp.body()))
                                 }
                                 Err(e) => {
                                     println!("{:?}", &e);
                                     Err(e)
                                 }
                             }
                         });

        Box::new(resp)

    }
}

impl Service for Dispatcher {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        let app_name = self.extract_app_name(&request);

        if app_name == "zas" {
            self.handle_zas_request(request)
        } else {
            self.handle_app_request(request, app_name)
        }
    }
}
