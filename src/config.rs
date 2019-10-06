use std::env;
use std::path::PathBuf;

use crate::error::Error;

pub struct Config {
    pub dns_port: u16,
    pub http_port: u16,
    pub app_dir: String,
    pub log_dir: String,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        let dns_port = read_dns_port()?;
        let http_port = read_http_port()?;
        let app_dir = env::var("ZAS_APP_DIR").unwrap_or(default_app_dir()?);
        let log_dir = env::var("ZAS_LOG_DIR").unwrap_or(default_log_dir()?);

        Ok(Config {
            dns_port,
            http_port,
            app_dir,
            log_dir,
        })
    }
}

fn read_dns_port() -> Result<u16, Error> {
    match env::var("ZAS_DNS_PORT") {
        Ok(value) => parse_port(value),
        Err(_) => parse_port("12043".to_string()),
    }
}

fn read_http_port() -> Result<u16, Error> {
    match env::var("ZAS_HTTP_PORT") {
        Ok(value) => parse_port(value),
        Err(_) => parse_port("12044".to_string()),
    }
}

fn parse_port(port: String) -> Result<u16, Error> {
    match port.parse::<u16>() {
        Ok(value) => Ok(value),
        Err(_) => Err(Error::InvalidPort(port)),
    }
}

fn default_app_dir() -> Result<String, Error> {
    let mut path_buf = home_dir_path()?;
    path_buf.push(".zas/apps");

    Ok(path_buf.to_str().unwrap().to_string())
}

fn default_log_dir() -> Result<String, Error> {
    let mut path_buf = home_dir_path()?;
    path_buf.push(".zas/logs");

    Ok(path_buf.to_str().unwrap().to_string())
}

fn home_dir_path() -> Result<PathBuf, Error> {
    let home_dir = match dirs::home_dir() {
        Some(value) => value,
        None => return Err(Error::InvalidUserHome),
    };

    let home_dir = match home_dir.to_str() {
        Some(value) => value,
        None => return Err(Error::InvalidUserHome),
    };

    Ok(PathBuf::from(home_dir))
}
