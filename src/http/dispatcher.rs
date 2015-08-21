extern crate hyper;

use std::env;
use std::thread;
use std::io::Read;
use std::collections::HashMap;
use std::sync::Mutex;

use self::hyper::net::Fresh;
use self::hyper::client::Client;
use self::hyper::header::Connection;
use self::hyper::header::Host;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;

use http::app::App;

const DEFAULT_APP_HOME: &'static str = "~/.zas";

pub struct Dispatcher {
    pub apps: Mutex<HashMap<String, App>>,
}

impl Dispatcher {
    pub fn new(apps: HashMap<String, App>) -> Dispatcher {
        Dispatcher {
            apps: Mutex::new(apps),
        }
    }
}

impl Handler for Dispatcher {
    fn handle(&self, request: Request, response: Response<Fresh>) {
        let app_home = env::var("ZAS_HOME").unwrap_or(DEFAULT_APP_HOME.to_string());
        let host: String = request.headers.get::<Host>().unwrap().hostname.to_string();
        let app_name = host.split(".").collect::<Vec<_>>().first().unwrap().to_string();

        let port: String;
        {
            let mut apps = self.apps.lock().unwrap();

            if !apps.contains_key(&app_name) {
                apps.insert(app_name.to_string(), App::new(app_name.to_string(), "12050", &app_home));
                // check that the port is open instead of sleep
                thread::sleep_ms(1000);
            }

            let app = apps.get(&app_name).unwrap();
            port = app.port.to_string();
        }

        let url = format!("http://localhost:{}", &port);

        let client = Client::new();

        let mut app_response = client.get(&url)
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        app_response.read_to_string(&mut body).unwrap();

        response.send(&body.into_bytes()).unwrap();
    }
}
