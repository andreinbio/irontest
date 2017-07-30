extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, "<strong>Test</strong> <sup>this</sup>"));
    response.headers.set(ContentType::html());
    Ok(response)
}

fn main() {
    let mut router = Router::new();           // Alternative syntax:
    router.get("/", handler, "index");        // let router = router!(index: get "/" => handler,
    router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);

    Iron::new(router).http("localhost:3000").unwrap();
}
