use std::vec::Vec;
use std::io::Error as IoError;
use std::env::current_dir;
use dirs::home_dir;
use std::path::{PathBuf};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde_yaml;
use serde::de::DeserializeOwned;
use std::fmt;
use serde::de::{self, value, Deserialize, Deserializer, Visitor, SeqAccess};

#[derive(Debug, Clone)]
pub struct ConfigContainer {
    pub(crate) project_config: Vec<ConfigWithPath<ProjectConfig>>,
    pub(crate) home_config: ConfigWithPath<HomeConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HomeConfig {
    pub checkout: CheckoutConfigs
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CheckoutConfigs {
    #[serde(rename = "default-provider")]
    pub default_provider: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Commands {
    CommandAndEnv(CommandAndEnv),
    CommandList(String),
}

impl Commands {
    pub fn to_command_and_envs(self) -> CommandAndEnv {
        return match self {
            Commands::CommandAndEnv(commands) => commands,
            Commands::CommandList(string) => { CommandAndEnv{command: string, command_env: HashMap::new()} }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExecCommandConfig {
    #[serde(default = "default_ignore_failures")]
    pub ignore_failures: bool,
    #[serde(default = "default_description")]
    pub description: String,
    #[serde(rename = "commands")]
    pub commands: Vec<Commands>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub exec: HashMap<String, ExecCommandConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommandAndEnv {
    pub command: String,
    
    #[serde(default)]
    #[serde(rename = "env")]
    pub command_env: HashMap<String, String>
}

#[derive(Debug, Clone)]
pub struct ConfigWithPath<T> {
    pub config: T,
    pub file: Option<PathBuf>
}

impl <T> ConfigWithPath<T> {
    pub fn new(config: T, file: Option<PathBuf>) -> ConfigWithPath<T> {
        return ConfigWithPath { config: config, file: file };
    }

    pub fn no_file(config: T) -> ConfigWithPath<T> {
        return ConfigWithPath { config: config, file: None };
    }
}

// API class, internally
#[derive(Debug)]
pub struct ExecConfig {
    pub commands: HashMap<String, ExecCommandConfig>,
    pub command_defintions: HashMap<String, PathBuf>
}

fn default_description() -> String {
    return s!("No Description Provided");
}

fn default_ignore_failures() -> bool {
    return false;
}

impl ConfigContainer {
    pub fn new() -> Result<Self, String> {
        let project_config: Vec<ConfigWithPath<ProjectConfig>> = match collapse_the_configs::<ProjectConfig>(search_up_for_config_files()) {
            Ok(value) => value,
            Err(s) => return Err(s)
        };
        let home_configs: Vec<ConfigWithPath<HomeConfig>> = match collapse_the_configs::<HomeConfig>(search_for_home_config()) {
            Ok(value) => value,
            Err(s) => return Err(s)
        };
        let home_configs = match home_configs.first() {
            Some(value) => value.clone(),
            None => ConfigWithPath::no_file(HomeConfig { checkout: CheckoutConfigs { default_provider: None } } )
        };

        trace!("Project Configs Found: {:?}", project_config);
        trace!("Home Configs Found: {:?}", home_configs);
        return Ok(ConfigContainer {
            project_config: project_config,
            home_config: home_configs,
        });
    }

    pub fn get_exec_configs(&self) -> ExecConfig {
        let mut command_map: HashMap<String, ExecCommandConfig> = HashMap::new();
        let mut command_defintion_map: HashMap<String, PathBuf> = HashMap::new();

        for project_config in self.project_config.clone().into_iter() {
            
            for (key, value) in project_config.config.exec.into_iter() {
                if !command_map.contains_key(&key) {
                    command_map.insert(key.clone(), value);

                    if let Some(file) = project_config.file.clone() {
                        command_defintion_map.insert(key, file);
                    }
                }
            }
        }

        return ExecConfig {
            commands: command_map,
            command_defintions: command_defintion_map
        };
    }

    pub fn get_home_configs(&self) -> HomeConfig {
        return self.home_config.config.clone();
    }
}

fn collapse_the_configs<T>(config_files: Vec<PathBuf>) -> Result<Vec<ConfigWithPath<T>>, String>
where
    T: DeserializeOwned,
{
    let mut return_configs: Vec<ConfigWithPath<T>> = Vec::new();

    for val in config_files {
        match read_file(&val) {
            Ok(config) => {
                match serde_yaml::from_str::<T>(&config) {
                    Ok(value) => return_configs.push(ConfigWithPath::new(value, Some(val))),
                    Err(err) => return Err(format!("Error trying to parse {:?}: '{}'", val, err))
                };
            }
            Err(err) => {
                error!("Error trying to parse {:?}: '{}'", val, err);
            }
        }
    }

    return Ok(return_configs);
}

fn read_file(path: &PathBuf) -> Result<String, IoError> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    return match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => Err(err)
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
