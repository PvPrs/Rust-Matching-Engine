use crate::OrderData;

use futures::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body, server::Server, Body, Method, Request, Response};
use std::net::SocketAddr;

pub mod net {
    use super::*;
    use hyper::server::conn::AddrIncoming;
    use std::convert::Infallible;
    use std::ops::Deref;
    use std::sync::{Arc, Mutex};

    pub async fn handle_incoming(
        req: Request<Body>,
        data: Arc<Mutex<OrderData>>,
    ) -> Result<Response<Body>, hyper::Error> {
        let (head, body) = req.into_parts();
        match (head.method, head.uri.path()) {
            (Method::POST, "/") => {
                let body = body::to_bytes(body).await?;
                let order: OrderData = serde_json::from_slice(&body).unwrap();
                *data.lock().unwrap() = order;
                return Ok(Response::new(Body::from(
                    serde_json::to_string(&order).unwrap(),
                )));
            }
            _ => (),
        }
        let body = Body::from("Send POST Orders to '/'.");
        Ok(Response::new(body))
    }

    pub async fn listen_serve(socket: SocketAddr, data: Arc<Mutex<OrderData>>) {
        let make_service = make_service_fn(move |_| {
            let data = Arc::clone(&data);
            let service = service_fn(move |req| handle_incoming(req, Arc::clone(&data)));
            async move { Ok::<_, hyper::Error>(service) }
        });
        // let make_service = make_service_fn(move |conn| async move {
        //     let data = Arc::clone(&data);
        //     Ok::<_, hyper::Error>(service_fn(move |req| async move {
        //         handle_incoming(req, data).await
        //     }))
        // });
        let server = Server::bind(&socket).serve(make_service);
        println!("Listening on: {}.", server.local_addr().ip());

        if let Err(e) = server.await {
            eprintln!("Error: {}", e)
        }
    }
}

// Test Module
