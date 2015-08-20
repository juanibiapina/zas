use std::path::PathBuf;
use std::process::Child;
use std::process::Command;

pub struct App {
    pub name: String,
    pub port: String,
    process: Child,
}

impl App {
    pub fn new(name: &str, port: &str, app_home: String) -> App {
        let mut path_buf = PathBuf::from(app_home);
        path_buf.push(&name);

        let child_process = Command::new("foreman")
            .arg("start")
            .current_dir(path_buf.as_path())
            .env("PORT", port)
            .spawn().unwrap();

        App{
            name: name.to_string(),
            port: port.to_string(),
            process: child_process,
        }
    }
}
