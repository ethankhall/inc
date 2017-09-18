use std::collections::{HashMap, BTreeMap};
use std::vec::Vec;
use std::env::{current_dir, home_dir};
use std::path::PathBuf;
use std::string;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::cell::Cell;

pub enum ConfigValue {
    String(String),
    Array(Vec<ConfigValue>),
    Number(f64),
    Null
}

pub enum ConfigSource {
    Home,
    WorkingDir
}

#[derive(Debug)]
struct ConfigContainer {
    project_config: Vec<Yaml>,
    home_config: Vec<Yaml>
}

trait ConfigParser {
    fn new() -> Self;
    fn get(&self, path: String) -> Option<ConfigValue>;
    fn get_from_source(&self, path: String, source: ConfigSource) -> Option<ConfigValue>;
}

impl ConfigParser for ConfigContainer {
    fn new() -> Self {
        let project_config: Vec<Yaml> = collapse_the_configs(search_up_for_config_files());
        let home_configs: Vec<Yaml> = collapse_the_configs(search_for_home_config());
        return ConfigContainer { project_config: project_config, home_config: home_configs };
    }

    fn get(&self, path: String) -> Option<ConfigValue> {
        
        return None;
    }

    fn get_from_source(&self, path: String, source: ConfigSource) -> Option<ConfigValue> {
        return None;
    }
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
            return Err(format!("No key `{}.{}` found", seen_path.join("."), key_part))
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

    if !hash.contains_key(&last_key_value) {
        return Err(String::from(format!("Yaml doesn't contain key {}", last_key)));
    }

    let value = hash.get(&last_key_value).unwrap();

    return Err(String::from("No key found"));
}

fn collapse_the_configs(config_files: Vec<PathBuf>) -> Vec<Yaml> {
    let mut return_configs: Vec<Yaml> = Vec::new();

    for val in config_files {
        match parse_config_file(val) {
            Some(confs) => { 
                for config in confs.into_iter() {
                    return_configs.push(config);
                }
            },
            _ => {}
        }
    }

    return return_configs;
}

fn parse_config_file(path: PathBuf) -> Option<Vec<Yaml>> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    return match YamlLoader::load_from_str(&*contents) {
        Ok(yaml) => Some(yaml),
        Err(_) => None
    };
}

// fn build_flat_map(prefix: String, map: BTreeMap<Yaml, Yaml>) -> BTreeMap<String, ConfigValue> {
//     let mut result_map = BTreeMap::new();

//     for (key, value) in map {
//         let key = key.as_str().unwrap();
//         match value {
//             Yaml::Real(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Real(value)),
//             Yaml::Integer(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Integer(value)),
//             Yaml::String(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::String(value)),
//             Yaml::Boolean(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Boolean(value)),
//             Yaml::Array(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Array(value.to_vec())),
//             Yaml::Hash(value) => println!("foo"),
//             Yaml::Null => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Null),
//             _ => {}
//         };
//     }

//     return result_map;
// }

fn config_file(prefix: &'static str, path: PathBuf) -> Option<PathBuf> {
    let config_search = path.join(format!("{}etrain.yaml", prefix));
    if config_search.exists() {
        return Some(config_search);
    }

    let config_search = path.join(format!("{}etrain.yaml", prefix));
    if config_search.exists() {
        return Some(config_search);
    }

    return None;
}

fn search_for_home_config() -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();

    let config_file = match home_dir() {
        Some(dir) => config_file(".", dir),
        None => None
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
            None => at_root = true
        }
    }

    return result;   
}