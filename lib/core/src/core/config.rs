use std::vec::Vec;
use std::env::{current_dir, home_dir};
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub enum ConfigValue {
    String(String),
    Array(Vec<ConfigValue>),
    Real(f64),
    Integer(i64),
    Boolean(bool),
    Null,
}

pub enum ConfigSource {
    Home,
    WorkingDir,
}

#[derive(Debug)]
pub struct ConfigContainer {
    project_config: Vec<Yaml>,
    home_config: Vec<Yaml>,
}

impl ConfigContainer {
    pub fn new() -> Self {
        let project_config: Vec<Yaml> = collapse_the_configs(search_up_for_config_files());
        let home_configs: Vec<Yaml> = collapse_the_configs(search_for_home_config());
        return ConfigContainer {
            project_config: project_config,
            home_config: home_configs,
        };
    }

    pub fn get_from_source_default(
        &self,
        path: String,
        source: ConfigSource,
        default: String,
    ) -> String {
        let result = self.get_from_source(path, source);
        let result = result.unwrap_or_else(|_| return ConfigValue::String(default.clone()));

        return match result {
            ConfigValue::String(value) => value,
            _ => default,
        };
    }

    pub fn get(&self, path: String) -> Result<ConfigValue, String> {
        let result = self.get_from_source(path.clone(), ConfigSource::WorkingDir);
        return match result {
            Ok(val) => Ok(val),
            Err(_) => self.get_from_source(path, ConfigSource::Home),
        };
    }

    pub fn get_from_source(
        &self,
        path: String,
        source: ConfigSource,
    ) -> Result<ConfigValue, String> {
        return match source {
            ConfigSource::WorkingDir => find_first_value(self.project_config.clone(), path),
            ConfigSource::Home => find_first_value(self.home_config.clone(), path),
        };
    }
}

fn find_first_value(yaml: Vec<Yaml>, path: String) -> Result<ConfigValue, String> {
    for doc in yaml.into_iter() {
        let result = find_value(doc, path.clone());
        if result.is_ok() {
            return result;
        }
    }

    return Err(format!("Unable to find any entry for {}", path));
}

fn find_value(yaml: Yaml, path: String) -> Result<ConfigValue, String> {
    let mut current_yaml = &yaml;
    let mut split_path: Vec<&str> = path.split(".").collect();
    let last_key = split_path.pop().unwrap();

    let mut seen_path: Vec<&str> = Vec::new();

    for key_part in split_path.iter() {
        let hash = current_yaml.as_hash();
        if let None = hash {
            return Err(String::from("Yaml isn't a map"));
        }
        let hash = hash.unwrap();

        let next_yaml = hash.get(&Yaml::String(String::from(*key_part)));
        if let None = next_yaml {
            return Err(format!(
                "No key `{}.{}` found",
                seen_path.join("."),
                key_part
            ));
        }

        current_yaml = next_yaml.unwrap();
        seen_path.push(key_part);
    }

    let hash = current_yaml.as_hash();
    if let None = hash {
        return Err(String::from("Yaml isn't a map"));
    }
    let hash = hash.unwrap();
    let last_key_value = Yaml::String(String::from(last_key));
    seen_path.push(last_key);

    if !hash.contains_key(&last_key_value) {
        return Err(String::from(
            format!("Yaml doesn't contain key {}", seen_path.join(".")),
        ));
    }

    return match pull_value(hash.get(&last_key_value).unwrap().clone()) {
        Ok(value) => Ok(value),
        Err(e) => Err(format!("Unable to parse {}: `{}`", seen_path.join("."), e)),
    };
}

fn pull_value(val: Yaml) -> Result<ConfigValue, &'static str> {
    return match val {
        Yaml::Real(value) => Ok(ConfigValue::Real(value.parse().unwrap())),
        Yaml::Integer(value) => Ok(ConfigValue::Integer(value)),
        Yaml::String(value) => Ok(ConfigValue::String(value)),
        Yaml::Boolean(value) => Ok(ConfigValue::Boolean(value)),
        Yaml::Array(value) => {
            let arr = value
                .into_iter()
                .map(|x| pull_value(x.clone()))
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();
            Ok(ConfigValue::Array(arr))
        }
        Yaml::Null => Ok(ConfigValue::Null),
        Yaml::Hash(_) => Err("Unable pull value from Hash"),
        _ => Err("Unable to parse type"),
    };
}

fn collapse_the_configs(config_files: Vec<PathBuf>) -> Vec<Yaml> {
    let mut return_configs: Vec<Yaml> = Vec::new();

    for val in config_files {
        match parse_config_file(val) {
            Some(confs) => {
                for config in confs.into_iter() {
                    return_configs.push(config);
                }
            }
            _ => {}
        }
    }

    return return_configs;
}

fn parse_config_file(path: PathBuf) -> Option<Vec<Yaml>> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Unable to read the file",
    );

    return match YamlLoader::load_from_str(&*contents) {
        Ok(yaml) => Some(yaml),
        Err(_) => None,
    };
}

/**
  * Checks to see if either the yaml or yml file exists.
  */
fn config_file(prefix: &'static str, path: PathBuf) -> Option<PathBuf> {
    let config_search = path.join(format!("{}inc.yaml", prefix));
    if config_search.exists() {
        return Some(config_search);
    }

    let config_search = path.join(format!("{}inc.yml", prefix));
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
