#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate url;
#[macro_use]
extern crate fail;

use futures::{future, Future, Stream};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

static INDEX: &str = r#"Working. Try: curl http://localhost:8080/failpoints/index -XPUT -d'panic' "#;
static MISSING_NAME: &[u8] = b"Missing param name";
static MISSING_ACTIONS: &[u8] = b"Missing param actions";
static FAIL_POINTS_PATH: &str = "/failpoints/";

// Using service_fn, we can turn this function into a `Service`.
fn param_example(req: Request<Body>) -> Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let path = req.uri().path().to_owned();
    let method = req.method().to_owned();

    // For paths start with /failpoints/
    if path.starts_with(FAIL_POINTS_PATH) {
        return match method {
            Method::PUT => {
                Box::new(req.into_body().concat2().map(move |chunk| {
                    let (_, name) = path.split_at(FAIL_POINTS_PATH.len());
                    if name.is_empty() {
                        return Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(MISSING_NAME.into())
                            .unwrap();
                    };

                    let actions = chunk.iter().cloned().collect::<Vec<u8>>();
                    let actions = String::from_utf8(actions).unwrap();
                    if actions.is_empty() {
                        return Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(MISSING_ACTIONS.into())
                            .unwrap();
                    };

                    if let Err(e) = fail::cfg(name.to_owned(), &actions) {
                        return Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(e.to_string().into())
                            .unwrap();
                    }
                    let body = format!("Add fail point with name: {}, actions: {}", name, actions);
                    Response::new(body.into())
                }))
            },
            Method::DELETE => {
                let (_, name) = path.split_at(FAIL_POINTS_PATH.len());
                if name.is_empty() {
                    return Box::new(future::ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(MISSING_NAME.into())
                        .unwrap()));
                };

                fail::remove(name);
                let body = format!("Delete fail point with name: {}", name);
                Box::new(future::ok(Response::new(body.into())))
            },
            Method::GET => {
                let list: Vec<String> = fail::list().into_iter()
                    .map(move |(s1, s2)| format!("{}={}", s1, s2))
                    .collect();

                let list = list.join("\n");
                Box::new(future::ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(list.into())
                    .unwrap()))
            },
            _ =>  {
                Box::new(future::ok(Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::empty())
                    .unwrap()))
            }
        }
    }

    match (method, path.as_ref()) {
        (Method::GET, "/") => {
            fail_point!("index");
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (Method::GET, "/home") => {
            fail_point!("home");
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (Method::GET, "/failpoints") => {
            let list: Vec<String> = fail::list().into_iter()
                .map(move |(s1, s2)| format!("{}={}", s1, s2))
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