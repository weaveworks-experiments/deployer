extern crate iron;

use iron::prelude::*;
use iron::status;

pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

