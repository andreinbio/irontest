use utils::Utils;
use rustview::view::View;

mod admin;

pub struct Controllers {
    pub admin: admin::Index,
}

impl Controllers {
    pub fn new(utils: Utils, admin_template: View) -> Controllers {
        Controllers {
            admin: admin::Index::new(utils.clone(), admin_template.clone()),
        }
    }
}
