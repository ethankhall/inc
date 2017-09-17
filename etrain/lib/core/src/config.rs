use std::collections::{HashMap, BTreeMap, HashSet};
use std::env::{current_dir, home_dir};
use std::path::PathBuf;
use std::string;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum ConfigValue {
    /// float types are stored as String, and parsed on demand.
    /// Note that f64 does NOT implement Eq trait and can NOT be stored in BTreeMap
    Real(string::String),
    /// Yaml int is stored as i64.
    Integer(i64),
    /// Yaml scalar.
    String(string::String),
    /// Yaml bool, e.g. `true` or `false`.
    Boolean(bool),
    /// Yaml array, can be access as a `Vec`.
    Array(Vec<ConfigValue>),
    /// Yaml bool, e.g. `null` or `~`.
    Null
}

#[derive(Debug)]
pub struct ConfigResults {
    project_config_map: BTreeMap<String, ConfigValue>,
    home_config_map: BTreeMap<String, ConfigValue>
}

pub fn find_configs() -> Result<ConfigResults, &'static str> {
    let result: HashSet<PathBuf> = search_up_for_config_files();
    
    // if let Some(path) = home_dir() {
    //     if let Some(config) = config_file(path) {
    //         result.insert(config);
    //     }
    // }
    return Ok(ConfigResults { project_config_map: BTreeMap::new(), home_config_map: BTreeMap::new() });
}

fn parse_config_file(path: PathBuf) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let yaml = YamlLoader::load_from_str(&*contents);

    let result_map = BTreeMap::new();

    if let Err(_) = yaml {
        return result_map;
    }

    for doc in yaml.unwrap().iter() {
        if let Some(hash) = doc.as_hash() {
            for (key, value) in hash {
                println!("{:?}: \"{:?}\"", key, value);
            }
        } 
    }
    return result_map;
}

fn build_flat_map(prefix: String, map: BTreeMap<Yaml, Yaml>) -> BTreeMap<String, ConfigValue> {
    let mut result_map = BTreeMap::new();

    for (key, value) in map {
        let key = key.as_str().unwrap();
        match value {
            Yaml::Real(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Real(value)),
            Yaml::Integer(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Integer(value)),
            Yaml::String(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::String(value)),
            Yaml::Boolean(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Boolean(value)),
            Yaml::Array(value) => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Array(value.to_vec())),
            Yaml::Hash(value) => println!("foo"),
            Yaml::Null => result_map.insert(format!("{}.{}", prefix, key), ConfigValue::Null),
            _ => {}
        };
    }

    return result_map;
}

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

fn search_up_for_config_files() -> HashSet<PathBuf> {
    let mut path = current_dir().unwrap();
    let mut result: HashSet<PathBuf> = HashSet::new();
    let mut at_root = false;

    while !at_root {
        if let Some(config) = config_file("", path.clone()) {
            result.insert(config);
        }

        match path.clone().parent() {
            Some(parent_path) => path = parent_path.to_path_buf(),
            None => at_root = true
        }
    }

    return result;   
}