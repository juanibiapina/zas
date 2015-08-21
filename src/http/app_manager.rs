use std::env;
use std::collections::HashMap;

use http::app::App;

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

    pub fn ensure_app_running(&mut self, app_name: &str) -> u16 {
        if !self.apps.contains_key(app_name) {
            let next_port = self.next_port;
            let app = App::new(&app_name, next_port, &self.app_home);
            self.apps.insert(app_name.to_string(), app);
            self.next_port = next_port + 1;
        }

        let app = self.apps.get(app_name).unwrap();

        app.port
    }
}
