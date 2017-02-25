#[macro_use]
extern crate chrono;
extern crate iron;
#[macro_use]
extern crate log;
extern crate prometheus;
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error;

use iron::prelude::*;
use iron::status;

mod alerts;
mod instrumentation;

pub use instrumentation::PrometheusConfig;

/// Top-level API definition.
pub fn api() -> router::Router {
    let mut router = router::Router::new();
    router.get("/", hello_world, "root");
    router.get("/metrics", instrumentation::PrometheusHandler{}, "metrics");
    router.post("/alert", alert_webhook, "alert");
    router
}

fn hello_world(_: &mut iron::request::Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn alert_webhook(req: &mut Request) -> IronResult<Response> {
    let result: serde_json::Result<alerts::AlertMessage> = serde_json::from_reader(&mut req.body);
    match result {
        Err(err) => Ok(Response::with((status::BadRequest, err.description()))),
        Ok(msg) => {
            debug!("Got alert message: {:?}", msg);
            Ok(Response::with((status::Ok, "received alert")))
        }
    }
}
