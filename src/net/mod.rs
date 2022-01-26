use crate::OrderData;
use crate::Order;

use hyper::server::conn::AddrIncoming;
use std::convert::Infallible;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, server::Server, Body, Method, Request, Response};
use std::net::SocketAddr;

pub mod net {
    use super::*;

    pub async fn handle_incoming(
        req: Request<Body>,
        tx: tokio::sync::mpsc::Sender<Order>,
    ) -> Result<Response<Body>, hyper::Error> {
        let (head, body) = req.into_parts();
        match (head.method, head.uri.path()) {
            (Method::POST, "/") => {
                let body = body::to_bytes(body).await?;
                let order: Order = serde_json::from_slice(&body)
                    .map_err(|err| Order::None)
                    .unwrap();
                tx.send(order).await.unwrap();
                return Ok(Response::new(Body::from(
                    serde_json::to_string(&order).unwrap(),
                )));
            }
            _ => (),
        }
        let body = Body::from("Send POST Orders to '/'.");
        Ok(Response::new(body))
    }

    pub async fn listen_serve(socket: SocketAddr, tx: tokio::sync::mpsc::Sender<Order>) {
        let make_service = make_service_fn(move |_| {
            let c_tx = tx.clone();
            let service = service_fn(move |req| handle_incoming(req, c_tx.clone()));
            async move { Ok::<_, hyper::Error>(service) }
        });

        let server = Server::bind(&socket).serve(make_service);
        println!("Listening on: {}.", server.local_addr().ip());

        if let Err(e) = server.await {
            eprintln!("Error: {}", e)
        }
    }
}

// Test Module
