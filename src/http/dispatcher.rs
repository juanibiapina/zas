extern crate hyper;

use std::io::copy;
use std::sync::Mutex;

use self::hyper::net::Fresh;
use self::hyper::client::Client;
use self::hyper::client::RedirectPolicy;
use self::hyper::header::Connection;
use self::hyper::header::Host;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::uri::RequestUri::AbsolutePath;

use http::app_manager::AppManager;
use common::error::Error;

pub struct Dispatcher {
    pub app_manager: Mutex<AppManager>,
}

impl Dispatcher {
    pub fn new(app_manager: AppManager) -> Dispatcher {
        Dispatcher {
            app_manager: Mutex::new(app_manager),
        }
    }

    fn extract_app_name<'a>(&'a self, request: &'a Request) -> &str {
        let host = &request.headers.get::<Host>().unwrap().hostname;
        let host_parts = host.split(".").collect::<Vec<_>>();

        host_parts.first().unwrap()
    }

    fn ensure_app_running(&self, app_name: &str) -> Result<u16, Error> {
        let mut app_manager = self.app_manager.lock().unwrap();

        app_manager.ensure_app_running(&app_name)
    }

    fn forward_uri<'a>(&'a self, request: &'a Request) -> &str {
        match request.uri {
            AbsolutePath(ref value) => value,
            _ => panic!(),
        }
    }

    fn handle_zas_request(&self, response: Response) {
        let app_manager = self.app_manager.lock().unwrap();

        let zas_home = &app_manager.app_home;

        response.send(&format!("ZAS_APP_HOME: {}", zas_home).into_bytes().to_owned()).unwrap();
    }

    fn handle_app_request(&self, mut request: Request, mut response: Response, port: u16) {
        let uri = self.forward_uri(&request).to_string();

        let app_url = format!("http://localhost:{}{}", port, uri);

        let connection_header = match request.headers.get::<Connection>() {
            Some(value) => value.clone(),
            None => Connection::close(),
        };

        let mut client = Client::new();
        client.set_redirect_policy(RedirectPolicy::FollowNone);

        let mut app_response = client.request(request.method.clone(), &app_url)
            .headers(request.headers.clone())
            .header(Connection::close())
            .body(&mut request)
            .send().unwrap();

        *response.status_mut() = app_response.status.clone();

        response.headers_mut().clear();
        response.headers_mut().extend(app_response.headers.iter());
        response.headers_mut().set(connection_header);

        let mut stream = response.start().unwrap();
        copy(&mut app_response, &mut stream).unwrap();
        stream.end().unwrap();
    }
}

impl Handler for Dispatcher {
    fn handle(&self, request: Request, response: Response<Fresh>) {
        let app_name = self.extract_app_name(&request).to_string();

        if app_name == "zas" {
            self.handle_zas_request(response);
        } else {
            let result = self.ensure_app_running(&app_name);

            let port = match result {
                Ok(value) => value,
                Err(_) => {
                    response.send(b"App not configured").unwrap();
                    return;
                },
            };

            self.handle_app_request(request, response, port);
        }
    }
}
