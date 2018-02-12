use std::env;
use toml;
mod config;

#[derive(Clone)]
pub struct Utils {
    base_folder: String,
    abspath: String,
    admin_path: String,
    content_path: String,
}

impl Utils {
    pub fn new() -> Utils {
        Utils {
            base_folder: String::from("src"),
            abspath: String::from(env::current_dir().unwrap().to_str().unwrap()),
            admin_path: Utils::get_current_lang_path("src/admin/templates/"),
            content_path: Utils::get_current_lang_path("src/admin/templates/"),
        }
    }

    /// # Get current lang
    // update code to check for other locales
    fn get_current_lang_path(s: &str) -> String {
        //@TODO more logic here to get the template for the current language
        format!("{}{}", s, "default/")
    }

    /// # Get absolute path
    pub fn get_abs_path(&self) -> String {
        self.abspath.clone()
    }

    /// # Get admin path
    pub fn get_admin_path(&self) -> String {
        // format!("{}/{}", &self.abspath[..], &self.admin_path[..])
        // format!("{}/{}", )
        // src, admin / templates / default /
        let config_object = self.get_config("config");
        let admin_folder = config_object.get("server.port");

        if admin_folder.is_none() {
            panic!("no 'admin_folder' configuration exist!");
        }

        println!("admin_folder is: {:?}", admin_folder.unwrap().as_integer().unwrap());
        // let admin_folder_name = admin_folder.unwrap().unwrap();
        //
        // println!("port is : {:?}", admin_folder_name);

        String::from("")
    }

    /// # Get content path
    pub fn get_content_path(&self) -> String {
        format!("{}/{}", &self.abspath[..], &self.content_path[..])
    }

    /// # Load configuration file
    fn load_file(&self, file_path: &str) -> String {
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::Read;

        let mut f = File::open(file_path).expect("Unable to open");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Error reading file");

        contents
    }

    /// # Parse toml file
    pub fn get_config(&self, str: &str) -> config::Config {
        let file_path = format!("{}/{}/{}.toml", &self.get_abs_path()[..], &self.base_folder[..], str);
        config::Config::new(&self.load_file(&file_path[..])[..])
    }
}
