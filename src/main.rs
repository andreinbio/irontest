extern crate iron;
extern crate router;
extern crate rustview;

use rustview::view::View;

use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {
    let template = View::new();
    let mut response = Response::with((status::Ok, template.render("index.html")));
    response.headers.set(ContentType::html());
    Ok(response)

}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    println!("Server running at http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}