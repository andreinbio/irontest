use std::env;

#[derive(Clone)]
pub struct Utils {
    abspath: String,
    admin_path: String,
    content_path: String,
}

impl Utils {
    pub fn new() -> Utils {
        Utils {
            abspath: String::from(env::current_dir().unwrap().to_str().unwrap()),
            admin_path: Utils::get_current_lang_path("src/admin/templates/"),
            content_path: Utils::get_current_lang_path("src/admin/templates/"),
        }
    }

    /// # Get current lang
    // update code to check for other locales
    fn get_current_lang_path(s: &str) -> String {
        format!("{}{}", s, "default/")
    }

    /// # Get absolute path
    pub fn get_abs_path(&self) -> String {
        self.abspath.clone()
    }

    /// # Get admin path
    pub fn get_admin_path(&self) -> String {
        format!("{}/{}", &self.abspath[..], &self.admin_path[..])
    }

    /// # Get content path
    pub fn get_content_path(&self) -> String {
        format!("{}/{}", &self.abspath[..], &self.content_path[..])
    }
}
