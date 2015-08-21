extern crate hyper;

use std::io::Read;
use std::collections::HashMap;

use self::hyper::net::Fresh;
use self::hyper::client::Client;
use self::hyper::header::Connection;
use self::hyper::header::Host;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;

use http::app::App;

pub struct Dispatcher {
    pub apps: HashMap<String, App>,
}

impl Dispatcher {
    pub fn new(apps: HashMap<String, App>) -> Dispatcher {
        Dispatcher {
            apps: apps,
        }
    }
}

impl Handler for Dispatcher {
    fn handle(&self, request: Request, response: Response<Fresh>) {
        let host: String = request.headers.get::<Host>().unwrap().hostname.to_string();
        let app_name = host.split(".").collect::<Vec<_>>().first().unwrap().to_string();

        let app = self.apps.get(&app_name).unwrap();

        let url = format!("http://localhost:{}", &app.port);

        let client = Client::new();

        let mut app_response = client.get(&url)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        app_response.read_to_string(&mut body).unwrap();

        response.send(&body.into_bytes()).unwrap();
    }
}
