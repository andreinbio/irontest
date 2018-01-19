use iron::prelude::*;
use iron::middleware::Handler;
use iron::status;
use iron::headers::{ContentType};

use utils::Utils;
use rustview::view::View;

pub struct Index {
    utils: Utils,
    template: View,
}

impl Index {
    pub fn new(utils: Utils, admin_template: View) -> Index {
        Index {
            utils: utils,
            template: admin_template,
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
            "user": "Andrei",
            "child_user": "Pages"
        });
        let mut response = Response::with((status::Ok, self.template.render("home.html", model)));

        response.headers.set(ContentType::html());
        Ok(response)
    }
}