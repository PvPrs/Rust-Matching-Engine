use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

pub mod net {
	use super::*;

	async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
		println!("Packet from client");
		Ok(Response::new(Body::from("Response from Server!")))
	}

	pub async fn http_connect(socket: SocketAddr) {
		let make_service = make_service_fn(move | conn | async move {
			Ok::<_, Infallible>(service_fn(handle))
		});

		let server = Server::bind(&socket).serve(make_service);
		println!("Listening for Http requests on {}.", server.local_addr().ip());
		if let Err(e) = server.await {
			eprintln!("Error: {}", e)
		}
	}
}