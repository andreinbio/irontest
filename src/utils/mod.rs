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
            admin_path: Utils::get_current_lang_path("src/templates/admin/"),
            content_path: Utils::get_current_lang_path("src/templates/content/"),
        }
    }

    /// # Get current lang
    fn get_current_lang_path(s: &str) -> String {
        format!("{}{}", s, "default/")
    }

    /// # Get absolute path
    pub fn get_abs_path(&self) -> String {
        self.abspath.clone()
    }

    /// # Get admin_path
    pub fn get_admin_path(&self) -> String {
        format!("{}/{}", &self.abspath[..], &self.admin_path[..])
    }
}
