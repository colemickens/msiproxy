extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::{Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;



fn token_handler(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/oauth2/token") => {
            // check header
            // check formdata/querystring

            let access_token = "whatever";
            
            *response.body_mut() = Body::from(access_token);
        }

        // Add support for POST to update the base token(s)
        
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {
    let addr = ("[::]:50342").parse().expect("failed to parse addr");

    let server = Server::bind(&addr)
        .serve(|| service_fn(token_handler))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
