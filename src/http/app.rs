use std::path::PathBuf;
use std::process::Child;
use std::process::Command;

pub struct App {
    pub name: String,
    pub port: u16,
    #[allow(dead_code)]
    process: Child,
}

impl App {
    pub fn new(name: String, port: u16, app_home: &str) -> App {
        let mut path_buf = PathBuf::from(app_home);
        path_buf.push(&name);

        let child_process = Command::new("foreman")
            .arg("start")
            .current_dir(path_buf.as_path())
            .env("PORT", format!("{}", port))
            .spawn().unwrap();

        App{
            name: name.to_string(),
            port: port,
            process: child_process,
        }
    }
}
