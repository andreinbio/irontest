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
                Some(value) => {
                    result = Some(value.clone());
                    match *value {
                        toml::Value::Array(_) | toml::Value::Table(_) => {
                            config_value = value.clone();
                        },
                        _ => {
                            break;
                        },
                    }
                },
                None => (),
            }
            self.value = config_value.clone();
        }

        result
    }

    /// # Checks if the value of the preference is true
    pub fn is(&self, str: &str) -> bool {
        let value: Option<toml::Value> = *self.get(str);
        value.is_some()
    }
}