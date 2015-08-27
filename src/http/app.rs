use std::thread;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;

pub struct App {
    pub port: u16,
    #[allow(dead_code)]
    process: Child,
}

impl App {
    pub fn new(name: &str, port: u16, app_home: &str) -> App {
        let mut path_buf = PathBuf::from(app_home);
        path_buf.push(&name);

        let child_process = match Command::new("foreman")
            .arg("start")
            .current_dir(path_buf.as_path())
            .env("PORT", port.to_string())
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
