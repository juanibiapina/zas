use std::collections::HashMap;

use http::app::App;

pub struct AppManager {
    pub apps: HashMap<String, App>,
}

impl AppManager {
    pub fn new() -> AppManager {
        AppManager {
            apps: HashMap::new(),
        }
    }
}
