#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::redundant_pub_crate)]

use std::{net::SocketAddr, time::Duration};

use http::{header, uri::PathAndQuery, Result, StatusCode, Uri};
use http_body_util::Empty;
use hyper::{
    body::{Bytes, Incoming},
    service::service_fn,
    Request, Response,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn::auto, graceful::GracefulShutdown},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_shutdown::Shutdown;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(&addr).await?;

    let server = auto::Builder::new(TokioExecutor::new());
    let graceful = GracefulShutdown::new();
    let shutdown = Shutdown::new()?;

    println!("listening on {addr}");

    loop {
        tokio::select! {
            conn = listener.accept() => {
                handle_connection(&server, &graceful, conn);
            }

            () = shutdown.handle() => {
                drop(listener);
                eprintln!("shutdown signal received");
                break;
            }
        }
    }

    tokio::select! {
        () = graceful.shutdown() => {
            eprintln!("gracefully shutdown");
        }
        () = tokio::time::sleep(Duration::from_secs(10)) => {
            eprintln!("shutdown timeout exceeded, stopping");
        }
    }

    Ok(())
}

fn handle_connection(
    server: &auto::Builder<TokioExecutor>,
    graceful: &GracefulShutdown,
    conn: std::io::Result<(TcpStream, SocketAddr)>,
) {
    let (stream, _) = match conn {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("accept error: {e}");
            return;
        }
    };

    let stream = TokioIo::new(Box::pin(stream));
    let conn = server.serve_connection_with_upgrades(stream, service_fn(forward));
    let conn = graceful.watch(conn.into_owned());

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {e}");
        }
    });
}

#[allow(clippy::unused_async)]
async fn forward(req: Request<Incoming>) -> Result<Response<Empty<Bytes>>> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(
            header::LOCATION,
            Uri::builder()
                .scheme("https")
                .authority("home.dnaka91.rocks")
                .path_and_query(req.uri().path_and_query().map_or("/", PathAndQuery::as_str))
                .build()
                .map_or_else(
                    |_| String::from("https://home.dnaka91.rocks"),
                    |u| u.to_string(),
                ),
        )
        .body(Empty::new())
}
