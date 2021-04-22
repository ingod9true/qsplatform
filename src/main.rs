mod config;
use config::Config;
use yansi::{Paint, Color, Style};
use std::convert::{Infallible};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode, Method};
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::io::Lines;

use serde_yaml;
use serde::{Deserialize, Serialize};
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn dispatch(request: Request<Body>) ->Result<Response<Body>> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/shang") => Ok(Response::new(Body::from("Hello World!"))),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        },
    }

}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConfFile {
    conf: Conf,
    log: Log
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub  struct Log {

    level: LogLevel,
    disk: bool,
    console: bool
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Conf {
    addr: String,
    port: u32,
    shutdown: bool

}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Critical,
}

#[tokio::main]
pub async fn main() -> Result<()> {

    let f = std::fs::File::open("./config.yaml").expect("file open err");
    let d: ConfFile = serde_yaml::from_reader(f).expect("yaml error");
    println!("Read YAML string: {:?}", d.log.level);

    if d.log.level ==  LogLevel::Error {
        println!("shang");
    }
    assert_eq!(d.log.level, LogLevel::Error);



    let alert = Style::new(Color::Red).bold().italic();
    println!("Alert! {}", alert.paint("This is serious business!"));
    println!("Hi! {}", alert.underline().paint("Super serious!"));
    println!("Alert! {}", Paint::new("Yet another.").with_style(alert));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(dispatch)) }
    });
    let addr = "127.0.0.1:3180".parse().unwrap();
    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}


//路由分发
