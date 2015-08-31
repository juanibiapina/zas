use std::thread;
use std::path::PathBuf;
use std::process::Child;
use std::process::Stdio;
use std::process::Command;
use std::fs::OpenOptions;
use std::os::unix::prelude::FromRawFd;
use std::os::unix::prelude::AsRawFd;

pub struct App {
    pub port: u16,
    #[allow(dead_code)]
    process: Child,
}

impl App {
    pub fn new(name: &str, port: u16, app_home: &str, log_home: &str) -> App {
        let mut path_buf = PathBuf::from(app_home);
        path_buf.push(&name);

        let current_dir = path_buf.as_path();

        let mut path_buf = PathBuf::from(log_home);
        path_buf.push(&name);

        let log_file_path = path_buf.as_path();

        let log_file = OpenOptions::new()
            .read(false)
            .write(true)
            .append(true)
            .create(true)
            .open(log_file_path).unwrap();

        let stdio: Stdio;
        unsafe {
            stdio = Stdio::from_raw_fd(log_file.as_raw_fd());
        }

        let child_process = match Command::new("foreman")
            .arg("start")
            .current_dir(current_dir)
            .env("PORT", port.to_string())
            .stdout(stdio)
            .spawn() {
                Ok(value) => value,
                Err(e) => panic!("{}", e),
            };

        sleep_until_port_open(port);

        App{
            port: port,
            process: child_process,
        }
    }
}

fn sleep_until_port_open(port: u16) {
    while !Command::new("nc").arg("-z").arg("localhost").arg(format!("{}", port)).status().unwrap().success() {
        thread::sleep_ms(300);
    }
}
