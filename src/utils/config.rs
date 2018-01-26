// #[derive(Debug, Deserialize)]
// struct Server {
//     ip: Option<String>,
//     port: Option<u64>,
// }
//
// #[derive(Debug, Deserialize)]
// struct Paths {
//     template_folder: Option<String>,
//     admin_theme: Option<String>,
//     content_themes: Option<String>,
//     active_theme: Option<String>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Config {
//     server: Option<Server>,
//     paths: Option<Paths>,
// }

use toml;

pub struct Config {
    value: toml::Value
}

impl Config {
    pub fn new(str: &str) -> Config {
        Config {
            value: str.parse::<toml::Value>().unwrap()
        }
    }

    pub fn get(&self, str: &str) ->() {// toml::Value {
        let mut config_object: toml::Value;
        let strings: Vec<&str> = str.split(".").collect::<Vec<&str>>();

        for item in &strings {
            match self.value.get(item) {
                Some(value) => println!("Some is: {:?}", value),
                None => println!("None is: none"),
            }
            println!("string is : {:?}", item);
        }

        // println!("test... is: {:?}", strings);
    }
}