extern crate iron;
extern crate prometheus;
extern crate router;

use iron::prelude::*;
use iron::status;

mod instrumentation;

pub use instrumentation::PrometheusConfig;

/// Top-level API definition.
pub fn api() -> router::Router {
    let mut router = router::Router::new();
    router.get("/", hello_world, "root");
    router.get("/metrics", instrumentation::PrometheusHandler{}, "metrics");
    router
}

fn hello_world(_: &mut iron::request::Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}
