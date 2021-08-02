#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::used_underscore_binding)]

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::http::{Result, Uri};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(forward)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("listening on {}", server.local_addr());

    let server = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[allow(clippy::unused_async)]
async fn forward(req: Request<Body>) -> Result<Response<Body>> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(
            "Location",
            Uri::builder()
                .scheme("https")
                .authority("home.dnaka91.rocks")
                .path_and_query(req.uri().path_and_query().map_or("/", |pq| pq.as_str()))
                .build()
                .map_or_else(
                    |_| String::from("https://home.dnaka91.rocks"),
                    |u| u.to_string(),
                ),
        )
        .body(Body::empty())
}

async fn shutdown_signal() {
    if tokio::signal::ctrl_c().await.is_err() {
        eprintln!("failed to install CTRL+C signal handler");
    }

    println!("shutting down");
}
