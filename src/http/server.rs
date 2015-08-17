extern crate hyper;

use std::thread;
use std::io::Read;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;

use self::hyper::server::Server;
use self::hyper::server::Request;
use self::hyper::server::Response;

use self::hyper::net::Fresh;

use self::hyper::client::Client;

use self::hyper::header::Connection;
use self::hyper::header::Host;

fn dispatcher(request: Request, response: Response<Fresh>) {
    let zas_home = env::var("ZAS_HOME").unwrap();

    let host: String = request.headers.get::<Host>().unwrap().hostname.to_string();
    let app_name = host.split(".").collect::<Vec<_>>().first().unwrap().to_string();

    let mut path = PathBuf::from(zas_home);
    path.push(app_name);

    let mut child_process = Command::new("foreman")
        .arg("start")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(path.as_path())
        .spawn().unwrap();

    thread::sleep_ms(1000);

    let client = Client::new();

    let mut app_response = client.get("http://localhost:12045")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    app_response.read_to_string(&mut body).unwrap();

    response.send(&body.into_bytes()).unwrap();

    child_process.kill().unwrap();
}

pub fn run() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        Server::http("127.0.0.1:12044").unwrap().handle(dispatcher).unwrap();
    })
}
