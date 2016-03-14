use std::thread;
use std::path::PathBuf;
use std::process::Child;
use std::process::Stdio;
use std::process::Command;
use std::fs::OpenOptions;
use std::os::unix::prelude::FromRawFd;
use std::os::unix::prelude::AsRawFd;
use std::time::Duration;

pub struct App {
    pub port: u16,
    #[allow(dead_code)]
    process: Child,
    #[allow(dead_code)]
    pid: u32,
}

impl App {
    pub fn new(name: &str, port: u16, app_dir: &str, log_dir: &str) -> App {
        let mut path_buf = PathBuf::from(app_dir);
        path_buf.push(&name);

        let current_dir = path_buf.as_path();

        let mut path_buf = PathBuf::from(log_dir);
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
            .arg("-c")
            .current_dir(current_dir)
            .env("PORT", port.to_string())
            .stdout(stdio)
            .spawn() {
                Ok(value) => value,
                Err(e) => panic!("{}", e),
            };

        let pid = child_process.id();

        sleep_until_port_open(port);

        App{
            port: port,
            process: child_process,
            pid: pid,
        }
    }

    pub fn term(&mut self) {
        Command::new("kill")
            .arg(&self.pid.to_string())
            .status().unwrap();

        self.process.wait().unwrap();
    }
}

fn sleep_until_port_open(port: u16) {
    while !Command::new("nc").arg("-z").arg("localhost").arg(format!("{}", port)).status().unwrap().success() {
        thread::sleep(Duration::from_millis(300));
    }
}
