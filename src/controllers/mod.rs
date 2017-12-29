use utils::Utils;
use rustview::view::View;

mod admin;

pub struct Controllers {
    pub index: admin::Index,
    pub home: admin::Home,
}

impl Controllers {
    pub fn new(utils: Utils, admin_template: View) -> Controllers {
        Controllers {
            index: admin::Index::new(utils.clone(), admin_template.clone()),
            home: admin::Home::new(utils.clone(), admin_template.clone()),
        }
    }
}
