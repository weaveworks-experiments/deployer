extern crate iron;
extern crate logger;
extern crate simplelog;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use simplelog::{LogLevelFilter, Config, SimpleLogger};

fn main() {
    let _log = SimpleLogger::init(LogLevelFilter::Debug, Config::default());

    let (logger_before, logger_after) = Logger::new(None);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let mut chain = Chain::new(hello_world);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let _server = Iron::new(chain).http("localhost:3000").unwrap();
}
