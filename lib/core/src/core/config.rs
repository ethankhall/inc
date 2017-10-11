use std::vec::Vec;
use std::env::{current_dir, home_dir};
use std::path::PathBuf;
use toml::value::Value;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct ConfigContainer {
    project_config: Vec<Value>,
    home_config: Vec<Value>,
}

pub struct CheckoutConfig {
    pub default: Option<String>
}

impl ConfigContainer {
    pub fn new() -> Self {
        let project_config: Vec<Value> = collapse_the_configs(search_up_for_config_files());
        let home_configs: Vec<Value> = collapse_the_configs(search_for_home_config());
        return ConfigContainer {
            project_config: project_config,
            home_config: home_configs,
        };
    }

    pub fn get_checkout_configs(&self) -> CheckoutConfig {

        if self.home_config.is_empty() {
            return CheckoutConfig { default: None };
        }

        let config_entry = self.home_config[0].get("config");
        if config_entry.is_none() {
            return CheckoutConfig { default: None };
        }

        let config_entry = config_entry.unwrap();
        let checkout_default = config_entry.get("default");
        if checkout_default.is_none() {
            return CheckoutConfig { default: None };
        }

        let checkout_default = checkout_default.unwrap().as_str().map(|y| String::from(y));
        return CheckoutConfig { default: checkout_default };
    }
}

fn collapse_the_configs(config_files: Vec<PathBuf>) -> Vec<Value> {
    let mut return_configs: Vec<Value> = Vec::new();

    for val in config_files {
        match parse_config_file(val) {
            Some(config) => {
                return_configs.push(config);
            }
            _ => {}
        }
    }

    return return_configs;
}

fn parse_config_file(path: PathBuf) -> Option<Value> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    return contents.parse::<Value>().ok();
}

/**
  * Checks to see if either the yaml or yml file exists.
  */
fn config_file(prefix: &'static str, path: PathBuf) -> Option<PathBuf> {
    let config_search = path.join(format!("{}inc.toml", prefix));
    if config_search.exists() {
        return Some(config_search);
    }

    return None;
}

fn search_for_home_config() -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();

    let config_file = match home_dir() {
        Some(dir) => config_file(".", dir),
        None => None,
    };

    match config_file {
        Some(path) => result.push(path),
        _ => {}
    }

    return result;
}

fn search_up_for_config_files() -> Vec<PathBuf> {
    let current_dir = current_dir();
    if let Err(_) = current_dir {
        return Vec::new();
    }
    let mut path = current_dir.unwrap();
    let mut result: Vec<PathBuf> = Vec::new();
    let mut at_root = false;

    while !at_root {
        if let Some(config) = config_file("", path.clone()) {
            result.push(config);
        }

        match path.clone().parent() {
            Some(parent_path) => path = parent_path.to_path_buf(),
            None => at_root = true,
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use std::env::set_current_dir;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_get_values_from_config() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/sample-config1.yaml");

        let parsed = parse_config_file(d);
        if let None = parsed {
            assert!(false, "Unable to parse config file");
        }
        let parsed = parsed.unwrap();

        assert!(parsed.len() == 2);

        let result = find_value(
            parsed.get(0).unwrap().clone(),
            String::from("checkout.default"),
        );
        match result {
            Ok(value) => assert!(value == ConfigValue::String(String::from("crom"))),
            Err(e) => {
                assert!(
                    false,
                    format!("Unable to get checkout.default. Error was `{}`", e)
                )
            }
        }

        let result = find_value(parsed.get(0).unwrap().clone(), String::from("integer"));
        match result {
            Ok(value) => assert!(value == ConfigValue::Integer(1)),
            Err(e) => assert!(false, format!("Unable to get integer. Error was `{}`", e)),
        }

        let result = find_value(parsed.get(0).unwrap().clone(), String::from("array"));
        match result {
            Ok(value) => {
                match value {
                    ConfigValue::Array(_) => {}
                    _ => assert!(false, "type was not array"),
                }
            }
            Err(e) => assert!(false, format!("Unable to get array. Error was `{}`", e)),
        }

        let result = find_value(parsed.get(0).unwrap().clone(), String::from("null"));
        match result {
            Ok(value) => assert!(value == ConfigValue::Null),
            Err(e) => assert!(false, format!("Unable to get null. Error was `{}`", e)),
        }
    }

    #[test]
    fn test_config_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test");

        assert!(config_file("test1-", d.clone()).is_some());
        assert!(config_file("test2-", d).is_some());
    }

    #[test]
    #[allow(unused)]
    fn test_search_up_for_config_files() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/sub");

        set_current_dir(&d);

        assert_eq!(search_up_for_config_files().len(), 1);
    }
}
