use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use bonfire::Session;
use bonfire::session::InsecureConnector;
use hyper::{Request, Response, Result};
use hyper::body::{Incoming, Bytes};
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use http_body_util::{Full, BodyExt};
use tokio::net::TcpListener;

#[tokio::test]
async fn fake_server() {
	let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
	let listener = TcpListener::bind(addr).await.unwrap();

	let handle = tokio::task::spawn(async move {
		let connector = InsecureConnector::new("localhost", addr);
		let mut session = Session::builder()
			.connect(connector).await.unwrap();
		let response = session.request("/", json::object!{ test: 1 }).await.unwrap();
		assert_eq!(json::object!{ test: 2 }, response);
	});

	let (stream, _) = listener.accept().await.unwrap();
	let io = TokioIo::new(stream);
	Builder::new().serve_connection(io, service_fn(fake_server_service)).await.unwrap();
	handle.await.unwrap();
}

async fn fake_server_service(mut request: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
	let mut string = String::new();
	while let Some(next) = request.frame().await {
		let frame = next.unwrap();
		if let Some(chunk) = frame.data_ref() {
			string.push_str(std::str::from_utf8(chunk).unwrap());
		}
	}

	let body = json::parse(&string).unwrap();
	assert_eq!(json::object!{ test: 1 }, body);

	let body = json::stringify(json::object!{ test: 2 });
	Ok(Response::builder()
		.status(200)
		.body(Full::new(Bytes::from(body)))
		.unwrap())
}
