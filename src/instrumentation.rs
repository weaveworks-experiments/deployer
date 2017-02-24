extern crate iron;
extern crate prometheus;
extern crate router;

use std::time::{Instant, Duration};

use iron::prelude::*;
use iron::middleware::{AroundMiddleware, Handler};
use iron::mime::Mime;
use iron::status;
use prometheus::{Encoder, HistogramVec, TextEncoder};

/// Serve the /metrics page.
pub struct PrometheusHandler;

impl Handler for PrometheusHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        // TODO: Want to support protobuf encoding as well, ideally
        // parametrized by 'Accept' header, but one step at a time.
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        // Panic if the encoder has a bad format type. Is there a better way?
        let content_type = encoder.format_type().parse::<Mime>().unwrap();
        // TODO: Figure out how to not have to buffer this in memory first.
        let mut buffer = vec![];
        match encoder.encode(&metric_families, &mut buffer) {
            Err(_) => Ok(Response::with((status::InternalServerError, "Could not encode metrics"))),
            Ok(_) => Ok(Response::with((status::Ok, buffer)).set(content_type))
        }
    }
}

/// Configuration for Prometheus instrumentation.
pub struct PrometheusConfig {
    /// A histogram vector for recording request duration. Must expect method
    /// and status code labels.
    pub duration: HistogramVec,
}

impl AroundMiddleware for PrometheusConfig {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(InstrumentedHandler { config: self, handler: handler })
    }
}

/// A handler that has been instrumented with Prometheus metrics.
struct InstrumentedHandler {
    config: PrometheusConfig,
    handler: Box<Handler>,
}

impl Handler for InstrumentedHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let start = Instant::now();
        let response = self.handler.handle(req);
        let elapsed = start.elapsed();
        // XXX: We can do better.
        let status_code = label_for_status_code(&response).map_or(String::from("none"), |i| i.to_string());
        self.config.duration
            .with_label_values(&[
                &*req.method.to_string(),
                &status_code,
            ])
            .observe(duration_to_seconds(elapsed));
        response
    }
}

/// Get the status code from a result as an optional integer.
fn label_for_status_code<'a>(response: &'a IronResult<Response>) -> Option<u16> {
    let status = match *response {
        Ok(ref r) => r.status,
        Err(ref err) => err.response.status,
    };
    status.map(|s| (s.to_u16()))
}

/// `duration_to_seconds` converts Duration to seconds.
#[inline]
pub fn duration_to_seconds(d: Duration) -> f64 {
    let nanos = d.subsec_nanos() as f64 / 1e9;
    d.as_secs() as f64 + nanos
}
