extern crate hyper;

use std::env;
use std::thread;
use std::io::Read;
use std::sync::Mutex;
use std::process::Command;

use self::hyper::net::Fresh;
use self::hyper::client::Client;
use self::hyper::header::Connection;
use self::hyper::header::Host;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;

use http::app::App;
use http::app_manager::AppManager;

const DEFAULT_APP_HOME: &'static str = "~/.zas";

pub struct Dispatcher {
    pub app_manager: Mutex<AppManager>,
}

impl Dispatcher {
    pub fn new(app_manager: AppManager) -> Dispatcher {
        Dispatcher {
            app_manager: Mutex::new(app_manager),
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
            let mut app_manager = self.app_manager.lock().unwrap();

            if !app_manager.apps.contains_key(&app_name) {
                let next_port = app_manager.next_port;
                app_manager.apps.insert(app_name.to_string(), App::new(app_name.to_string(), next_port, &app_home));
                app_manager.next_port = next_port + 1;

                block_until_port_open(next_port);
            }

            let app = app_manager.apps.get(&app_name).unwrap();
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

fn block_until_port_open(port: u16) {
    while !Command::new("nc").arg("-z").arg("localhost").arg(format!("{}", port)).status().unwrap().success() {
        thread::sleep_ms(300);
    }
}
