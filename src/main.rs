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

mod utils;
mod content;
mod controllers;
mod routers;

use utils::Utils;
use controllers::Controllers;

fn main() {
    //utils
    let m_utils = Utils::new();
    //admin template
    let admin_template = View::new(m_utils.get_admin_path());
    //controllers
    let m_controllers = Controllers::new(m_utils, admin_template);
    //routes
    let my_router = routers::new(m_controllers);

    println!("Server running at http://localhost:3000");
    Iron::new(my_router).http("localhost:3000").unwrap();
}
