extern crate hyper;

use std::io::Read;
use std::sync::Mutex;

use self::hyper::net::Fresh;
use self::hyper::client::Client;
use self::hyper::header::Connection;
use self::hyper::header::Host;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::uri::RequestUri::AbsolutePath;

use http::app_manager::AppManager;

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

    fn ensure_app_running(&self, app_name: &str) -> u16 {
        let mut app_manager = self.app_manager.lock().unwrap();

        app_manager.ensure_app_running(&app_name)
    }

    fn forward_uri<'a>(&'a self, request: &'a Request) -> &str {
        match request.uri {
            AbsolutePath(ref value) => value,
            _ => panic!(),
        }
    }
}

impl Handler for Dispatcher {
    fn handle(&self, request: Request, response: Response<Fresh>) {
        let app_name = self.extract_app_name(&request);
        let port = self.ensure_app_running(app_name);

        let uri = self.forward_uri(&request);

        let app_url = format!("http://localhost:{}{}", port, uri);

        let client = Client::new();

        let mut app_response = client.request(request.method.clone(), &app_url)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        app_response.read_to_string(&mut body).unwrap();

        response.send(&body.into_bytes()).unwrap();
    }
}
