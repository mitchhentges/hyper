use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 44309));

    let make_svc = make_service_fn(move |_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |mut req: Request<Body>| {
                async move {
                    // Uncomment to work around the issue:
                    // let wasm_bytes = hyper::body::to_bytes(req.body_mut()).await.unwrap();
                    // dbg!(wasm_bytes.len());
                    let result: Result<Response<Body>, Infallible> = Ok(Response::new(Body::empty()));
                    result
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Running on port 44309 :)");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
