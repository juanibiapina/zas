use std::env;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use http::app::App;
use common::error::Error;

const DEFAULT_APP_HOME: &'static str = "~/.zas";

pub struct AppManager {
    next_port: u16,
    app_home: String,
    apps: HashMap<String, App>,
}

impl AppManager {
    pub fn new() -> AppManager {
        let app_home = env::var("ZAS_HOME").unwrap_or(DEFAULT_APP_HOME.to_string());

        AppManager {
            next_port: 12050,
            app_home: app_home,
            apps: HashMap::new(),
        }
    }

    fn start_app(&mut self, app_name: &str) {
        let next_port = self.next_port;
        let app = App::new(&app_name, next_port, &self.app_home);
        self.apps.insert(app_name.to_string(), app);
        self.next_port = next_port + 1;
    }

    pub fn ensure_app_running(&mut self, app_name: &str) -> Result<u16, Error> {
        if !self.apps.contains_key(app_name) {
            let mut path_buf = PathBuf::from(&self.app_home);
            path_buf.push(&app_name);

            if fs::metadata(path_buf.as_path()).is_err() {
                return Err(Error::AppNotConfigured);
            }

            self.start_app(app_name);
        }

        let app = self.apps.get(app_name).unwrap();

        Ok(app.port)
    }
}
