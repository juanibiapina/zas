extern crate toml;
extern crate xdg;

use self::toml::Value;

use std::str::FromStr;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use error::Error;

pub struct AppManager {
    apps: HashMap<String, u16>,
}

impl AppManager {
    pub fn new() -> Result<AppManager, Error> {
        let config_path = get_config_path()?;

        let value = Value::from_str(&read_config_file(&config_path)?).unwrap();
        let data = value.as_table().unwrap();

        let apps: HashMap<String, u16> = data.iter().map(|(k, v)| (k.to_owned(), v.as_integer().unwrap() as u16)).collect();

        Ok(AppManager {
            apps: apps,
        })
    }

    pub fn get_port(&self, app_name: &str) -> Option<u16> {
        if !self.apps.contains_key(app_name) {
            return None;
        }

        let port = *self.apps.get(app_name).unwrap();

        Some(port)
    }
}

fn get_config_path() -> Result<PathBuf, Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("zas")?;

    return Ok(xdg_dirs.place_config_file("apps.toml")?);
}

fn read_config_file(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
