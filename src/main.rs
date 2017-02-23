#[macro_use]
extern crate clap;
extern crate iron;
extern crate logger;
extern crate simplelog;

use clap::{Arg, App};
use iron::prelude::*;
use iron::status;
use logger::Logger;
use simplelog::{LogLevelFilter, Config, SimpleLogger};

// TODO
// - prometheus metrics for http requests
// - prometheus metrics for hyper in general?
// - factor out actual handlers

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

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let mut chain = Chain::new(hello_world);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let _server = Iron::new(chain).http("localhost:3000").unwrap();
}
