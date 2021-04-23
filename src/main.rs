
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod logger;
pub(crate) mod modules;

use log::{debug,info};

use std::convert::{Infallible};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode, Method};


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn dispatch(request: Request<Body>) ->Result<Response<Body>> {
    match (request.method(), request.uri().path()) {
        (&Method::GET, "/shang") => {

            info!("{:?} | {} | ", request.method(),request.uri().path());
            Ok(Response::new(Body::from("Hello World!")))
        },
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        },
    }

}


#[tokio::main]
pub async fn main() -> Result<()> {
    logger::init();
    debug!("qsplatform app luanch!!!");

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(dispatch)) }
    });
    let addr = "127.0.0.1:3180".parse().unwrap();
    let server = Server::bind(&addr).serve(make_svc);
    debug!("linster in 127.0.0.1:3180 ");
    server.await?;
    Ok(())
}


//路由分发
