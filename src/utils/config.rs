use toml;

pub struct Config {
    value: toml::Value,
}

impl Config {
    pub fn new(str: &str) -> Config {
        Config {
            value: str.parse::<toml::Value>().unwrap()
        }
    }

    /// # Get the value of the preference
    pub fn get(&mut self, str: &str) -> Option<toml::Value> {
        let strings: Vec<&str> = str.split(".").collect::<Vec<&str>>();
        let mut result: Option<toml::Value> = None;
        let mut config_value: toml::Value = self.value.clone();

        for item in &strings {
            match self.value.get(item) {
                Some(value) => match *value {
                    toml::Value::String(ref string) => {
                        result = Some(value.clone());
                        break;
                    },
                    toml::Value::Integer(ref integer) => {
                        result = Some(value.clone());
                        break;
                    },
                    toml::Value::Float(ref float) => {
                        result = Some(value.clone());
                        break;
                    },
                    toml::Value::Boolean(ref boolean) => {
                        result = Some(value.clone());
                        break;
                    },
                    toml::Value::Datetime(ref datetime) => {
                        result = Some(value.clone());
                        break;
                    },
                    toml::Value::Array(ref array) => {
                        config_value = value.clone();
                        result = Some(value.clone());
                    },
                    toml::Value::Table(ref table) => {
                        config_value = value.clone();
                        result = Some(value.clone());
                    },
                },
                None => (),
            }
            self.value = config_value.clone();
        }

        result
    }

    /// # Checks if the value of the preference is true
    pub fn is(&self, str: &str) -> () {

    }
}