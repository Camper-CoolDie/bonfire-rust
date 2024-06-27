use bonfire::{InsecureConnector, Session};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::header;
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use hyper::{Request, Response, Result};
use hyper_util::rt::TokioIo;
use serde_json::{json, Value};
use std::io::Write;
use tokio::net::TcpListener;

#[tokio::test]
async fn fake_server() {
    let host = "localhost";
    let addr = (host, 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    let handle = tokio::task::spawn(async move {
        let connector = InsecureConnector::new(host, addr);
        let mut session = Session::builder().connect(connector).await.unwrap();
        let response = session.request("/", json!({ "test": 1 })).await.unwrap();
        assert_eq!(json!({ "test": 2 }), response);
    });

    let (stream, _) = listener.accept().await.unwrap();
    let io = TokioIo::new(stream);
    Builder::new()
        .serve_connection(io, service_fn(fake_server_service))
        .await
        .unwrap();
    handle.await.unwrap();
}

async fn fake_server_service(mut request: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
    let length = request.headers().get(header::CONTENT_LENGTH).unwrap();
    let length = length.to_str().unwrap();
    let length: usize = length.parse().unwrap();

    let mut body: Vec<u8> = Vec::with_capacity(length);
    while let Some(next) = request.frame().await {
        let frame = next.unwrap();
        if let Some(chunk) = frame.data_ref() {
            body.write(chunk).unwrap();
        }
    }
    body.flush().unwrap();

    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json!({ "test": 1 }), body);

    let mut body: Vec<u8> = Vec::new();
    let _ = serde_json::to_writer(&mut body, &json!({ "test": 2 })).unwrap();
    Ok(Response::builder()
        .status(200)
        .body(Full::new(Bytes::from(body)))
        .unwrap())
}
