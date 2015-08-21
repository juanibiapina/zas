use std::collections::HashMap;

use http::app::App;

pub struct AppManager {
    pub next_port: u16,
    pub apps: HashMap<String, App>,
}

impl AppManager {
    pub fn new() -> AppManager {
        AppManager {
            next_port: 12050,
            apps: HashMap::new(),
        }
    }
}
