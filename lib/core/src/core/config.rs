use std::vec::Vec;
use std::env::{current_dir, home_dir};
use std::path::PathBuf;
use toml::value::Value;
use toml::value::Table;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConfigContainer {
    project_config: Vec<Value>,
    home_config: Vec<Value>,
}

pub struct CheckoutConfig {
    pub default: Option<String>
}

pub struct ExecCommandConfig {
    pub commands: Option<Vec<String>>,
    pub ignore_failures: bool
}

pub struct ExecConfig {
    pub commands: HashMap<String, ExecCommandConfig>
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

    pub fn get_exec_configs(&self) -> ExecConfig {
        let command_map: HashMap<String, ExecCommandConfig> = HashMap::new();
        for config in self.project_config.clone().into_iter() {
            debug!("Processing new config entry");
            let config_entry = config.get("exec");
            if config_entry.is_none() {
                continue;
            }

            let config_entry = config_entry.unwrap().as_table();
            if config_entry.is_none() {
                continue;
            }

            let config_entry = config_entry.unwrap();
            for config_key in config_entry.keys() {
                if command_map.contains_key(config_key) {
                    continue;
                }

                let command_entry = config_entry.get(config_key).unwrap();
                let command_table = command_entry.as_table();
                if command_table.is_none() {
                    debug!("{} was not a table, but instead was {}", config_key, command_entry.type_str());
                    continue;
                }

                let commands = self.get_commands_from_table(format!("exec.{}", config_key), command_table.unwrap());
            }
        }

        return ExecConfig { commands: HashMap::new() };
    }

    fn get_commands_from_table(&self, key: String, table: &Table) -> Option<Vec<String>> {
        if table.contains_key("commands") {
            let commands = table.get("commands").unwrap();
            if commands.is_str() {
                return Some(vec![String::from(commands.as_str().unwrap())]);
            }

            if commands.is_array() {
                return Some(commands.as_array()
                    .unwrap()
                    .iter()
                    .map(|x| String::from(x.as_str().unwrap()))
                    .collect());
            }

            error!("{} was supposed to be an array or string, but was {}", key, commands.type_str());
            return None;
        } else {
            return None;
        }
    }

    pub fn get_checkout_configs(&self) -> CheckoutConfig {

        if self.home_config.is_empty() {
            return CheckoutConfig { default: None };
        }

        let config_entry = self.home_config[0].get("checkout");
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