use iron::middleware::Handler;
use utils::Utils;
use rustview::view::View;

pub mod home;
pub mod page;

pub struct Controllers {
    pub home: home::Index,
    pub page: page::Index,
}

impl Controllers {
    pub fn new(utils: Utils, admin_template: View) -> Controllers {
        Controllers {
            home: home::Index::new(utils.clone(), admin_template.clone()),
            page: page::Index::new(utils.clone(), admin_template.clone()),
        }
    }
}
