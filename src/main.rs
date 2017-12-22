extern crate iron;
extern crate router;
extern crate rustview;
extern crate serde;

#[macro_use]
extern crate serde_json;

use rustview::view::View;

use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use router::Router;

fn handler(req: &mut Request) -> IronResult<Response> {
    let admin_template = View::new("src/admin/templates/default/");
    let model = json!({
        "title": "Testing",
        "newTitle": "New Cool Title here :)",
        "helloUser": "Hi Andrei !",
        "testText": "It's working!!!!!",
        "user": "Andrei"
    });
    let mut response = Response::with((status::Ok, admin_template.render("index.html", model)));
    response.headers.set(ContentType::html());
    Ok(response)

}

fn query_handler(req: &mut Request) -> IronResult<Response> {
    let admin_template = View::new("src/admin/templates/default/");
    let model = json!({
        "title": "Sub Page",
        "newTitle": "Home Title Here",
        "helloUser": "Hi Alex :) !",
        "testText": "Homepage!!!!!",
        "child_user": "Alex"
    });
    let mut response = Response::with((status::Ok, admin_template.render("home.html", model)));
    response.headers.set(ContentType::html());
    Ok(response)
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", query_handler, "query");

    // let model = json!({
    //    "title": json!({"name": "Andrei"}),
    //    "newTitle": "New Cool Title here :)",
    //    "helloUser": "Hi Andrei !"
    // });
    // println!("Modle is: {:?}", model.as_object().unwrap().keys().next().unwrap());
    println!("Server running at http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
