#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate fail;

use futures::{future, Future, Stream};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

// use std::collections::HashMap;
// use url::form_urlencoded;

static INDEX: &str = r#"Working. Try: curl localhost:8080/fail -XPOST -d'{"name": "index", "actions": "panic"}'"#;
// static MISSING: &[u8] = b"Missing field";
// static NOTNUMERIC: &[u8] = b"Number field is not numeric";

#[derive(Deserialize, Debug)]
struct FailPoint {
    name: String,
    actions: String,
}

#[derive(Deserialize, Debug)]
struct FailPointDelete {
    name: String,
}

// Using service_fn, we can turn this function into a `Service`.
fn param_example(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            fail_point!("index");
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::GET, "/home") => {
            fail_point!("home");
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::PUT, "/fail") => {
            Box::new(req.into_body().concat2().map(|b| {
                let fail_point: FailPoint = match serde_json::from_slice(&b) {
                    Ok(f) => f,
                    Err(e) => return Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(e.to_string().into())
                        .unwrap()
                };
                fail::cfg(fail_point.name.clone(), &fail_point.actions).unwrap();
                let body = format!("Add fail point with name: {}, actions: {}", fail_point.name, fail_point.actions);
                Response::new(body.into())
            }))
        },
        (&Method::DELETE, "/fail") => {
            Box::new(req.into_body().concat2().map(|b| {
                let fail_point: FailPointDelete = match serde_json::from_slice(&b) {
                    Ok(f) => f,
                    Err(e) => return Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(e.to_string().into())
                        .unwrap()
                };
                fail::remove(&fail_point.name);
                let body = format!("Delete fail point with name: {}", fail_point.name);
                Response::new(body.into())
            }))
        },
        (&Method::GET, "/fail") => {
            let list: Vec<String> = fail::list().into_iter().map(move|(s1, s2)| format!("{}: {}", s1, s2))
                .collect();

            let list = list.join("\n");
            Box::new(future::ok(Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(list.into())
                .unwrap()))
        },
        _ => {
            Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()))
        }
    }

}

fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(param_example))
        .map_err(|e| eprintln!("server error: {}", e));

    fail::setup();
    hyper::rt::run(server);
}