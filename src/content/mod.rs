use iron::prelude::*;
use iron::middleware::Handler;
use iron::method;
use utils::Utils;
use rustview::view::View;

mod controllers;
use self::controllers::Controllers;

use utils;

pub struct Router {
    pub method: method::Method,
    pub glob: String,
    pub handler: Box<Handler>,
    pub route_id: String
}

pub struct Content {
    routers: Vec<Router>
}

impl Router {
    fn new<H: Handler>(method: method::Method, glob: &str, handler: H, route_id: &str) -> Router {
        Router {
            method: method,
            glob: glob.to_string(),
            handler: Box::new(handler),
            route_id: route_id.to_string(),
        }
    }
}

impl Content {
    pub fn new() -> Content {
        let utils = Utils::new();
        let content_template = View::new(utils.get_content_path());
        let controllers = Controllers::new(utils, content_template);
        let mut my_routers: Vec<Router> = vec!();

        my_routers.push(Router::new(method::Get, "/", controllers.home, "home"));
        my_routers.push(Router::new(method::Get, "/page", controllers.page, "page"));

        println!("routers length: {:?}", my_routers.len());

        Content { routers: my_routers}
    }

    pub fn get_routers(self) -> Vec<Router> {
        self.routers
    }
}