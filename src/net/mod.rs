use futures::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, server::Server, Body, Method, Request, Response};
use std::convert::Infallible;
use std::io::Read;
use std::net::SocketAddr;
use crate::OrderData;
use std::fmt::Display;
use tokio;

pub mod net {
	use super::*;

    pub async fn handle_incoming(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let (head, body) = req.into_parts();
        match (head.method, head.uri.path()) {
            (Method::POST, "/") => {
                let body = body::to_bytes(body).await?;
                let order: OrderData = serde_json::from_slice(&body).unwrap();
                return Ok(Response::new(Body::from(
                    serde_json::to_string(&order).unwrap(),
                )));
            }
            _ => (),
        }
        let body = Body::from("Send POST Orders to '/'.");
        Ok(Response::new(body))
    }

    pub async fn http_connect(socket: SocketAddr) {
        let make_service = make_service_fn(move |conn| async move {
            Ok::<_, hyper::Error>(service_fn(handle_incoming))
        });

        let server = Server::bind(&socket).serve(make_service);
        println!(
            "Listening for Http requests on {}.",
            server.local_addr().ip()
        );

        if let Err(e) = server.await {
            eprintln!("Error: {}", e)
        }
    }
}
