use std::env;
use std::path::PathBuf;

pub struct Config {
    pub dns_port: u16,
    pub http_port: u16,
    pub app_dir: String,
    pub log_dir: String,
}

impl Config {
    pub fn new() -> Config {
        let dns_port = env::var("ZAS_DNS_PORT").unwrap_or("12043".to_string()).parse::<u16>().unwrap();
        let http_port = env::var("ZAS_HTTP_PORT").unwrap_or("12044".to_string()).parse::<u16>().unwrap();
        let app_dir = env::var("ZAS_APP_DIR").unwrap_or(Config::default_app_dir());
        let log_dir = env::var("ZAS_LOG_DIR").unwrap_or(Config::default_log_dir());

        Config {
            dns_port: dns_port,
            http_port: http_port,
            app_dir: app_dir,
            log_dir: log_dir,
        }
    }

    fn default_app_dir() -> String {
        let mut path_buf = PathBuf::from(env::home_dir().unwrap().to_str().unwrap());
        path_buf.push(".zas/apps");

        path_buf.to_str().unwrap().to_string()
    }

    fn default_log_dir() -> String {
        let mut path_buf = PathBuf::from(env::home_dir().unwrap().to_str().unwrap());
        path_buf.push(".zas/logs");

        path_buf.to_str().unwrap().to_string()
    }

}
