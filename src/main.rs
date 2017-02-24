#[macro_use]
extern crate clap;
extern crate iron;
extern crate logger;
#[macro_use]
extern crate prometheus;
extern crate simplelog;

extern crate deployer;

use clap::{Arg, App};
use iron::prelude::*;
use logger::Logger;
use simplelog::{LogLevelFilter, Config, SimpleLogger};

// TODO
// - prometheus metrics for hyper in general?
// - quickly check to see what metrics we export in Weave golang stuff
// - add endpoint to handle alerts
// - send deploy request to flux

fn main() {
    let matches = App::new("deployer")
        .version(crate_version!())
        .author("Jonathan M. Lange <jml@mumak.net>")
        .about("Deploy images in response to Prometheus alerts")
        .arg(Arg::with_name("log-level")
             .long("log-level")
             .value_name("LOG_LEVEL")
             .default_value("Info")
             .help("Level to log at")
             .possible_values(&["Trace", "Debug", "Info", "Warn", "Error", "Off"])
             .takes_value(true))
        .get_matches();

    let log_level = value_t_or_exit!(matches, "log-level", LogLevelFilter);

    let _log = SimpleLogger::init(log_level, Config::default());

    let (logger_before, logger_after) = Logger::new(None);

    let request_duration = register_histogram_vec!(
            "deployer_http_request_duration_seconds",
            "Time spent on HTTP requests",
            &["method", "status_code"]
    ).unwrap();

    let prometheus_config = deployer::PrometheusConfig{
        duration: request_duration,
    };

    let mut chain = Chain::new(deployer::api());
    chain.link_around(prometheus_config);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let _server = Iron::new(chain).http("localhost:3000").unwrap();
}
