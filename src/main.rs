#[macro_use]
extern crate clap;
extern crate iron;
extern crate logger;
extern crate simplelog;

extern crate deployer;

use clap::{Arg, App};
use iron::prelude::*;
use logger::Logger;
use simplelog::{LogLevelFilter, Config, SimpleLogger};

// TODO
// - prometheus metrics for http requests
// - prometheus metrics for hyper in general?

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

    let mut chain = Chain::new(deployer::hello_world);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let _server = Iron::new(chain).http("localhost:3000").unwrap();
}
