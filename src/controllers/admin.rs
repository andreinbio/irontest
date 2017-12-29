use iron::prelude::*;
use iron::middleware::Handler;
use iron::status;
use iron::headers::{ContentType};

use utils::Utils;
use rustview::view::View;

pub struct Index {
    utils: Utils,
    admin_template: View,
}

pub struct Home {
    utils: Utils,
    admin_template: View,
}

impl Index {
    pub fn new(utils: Utils, admin_template: View) -> Index {
        Index {
            utils: utils,
            admin_template: admin_template,
        }
    }
}

impl Home {
    pub fn new(utils: Utils, admin_template: View) -> Home {
        Home {
            utils: utils,
            admin_template: admin_template,
        }
    }
}

impl Handler for Index {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let model = json!({
            "title": "Testing",
            "newTitle": "New Cool Title here :)",
            "helloUser": "Hi Andrei !",
            "testText": "It's working!!!!!",
            "user": "Andrei"
        });
        let mut response = Response::with((status::Ok, self.admin_template.render("index.html", model)));

        response.headers.set(ContentType::html());
        Ok(response)
    }
}

impl Handler for Home {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let model = json!({
            "title": "Sub Page",
            "newTitle": "Home Title Here",
            "helloUser": "Hi Alex :) !",
            "testText": "Homepage!!!!!",
            "child_user": "Alex"
        });
        let mut response = Response::with((status::Ok, self.admin_template.render("home.html", model)));

        response.headers.set(ContentType::html());
        Ok(response)
    }
}
