use config::ConfigContainer;
use slog::Logger;
use std::path::PathBuf;

pub struct SubCommandLocation {
    pub path: PathBuf,
    pub name: String
}

pub trait MainCommand {
    fn execute(&self, args: Vec<String>, logger: &Logger, config_container: &ConfigContainer, sub_commands: Vec<String>) -> i32;
    fn name(&self) -> String;
    fn description(&self) -> String;
}