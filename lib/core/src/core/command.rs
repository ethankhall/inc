use core::config::ConfigContainer;
use slog::{Logger, Level};
use std::collections::HashMap;
use libs::process::SystemCommand;
use exec::Execution;

#[derive(Debug)]
pub struct CommandContainer {
    pub commands: HashMap<String, SystemCommand>,
}

#[derive(Debug)]
pub struct LoggingContainer<'a> {
    pub logger: &'a Logger,
    pub level: &'a Level,
}

impl CommandContainer {
    pub fn find_sub_commands(&self, command: String) -> Option<SystemCommand> {
        return match self.commands.get(&(command)) {
            Some(value) => Some(value.clone()),
            None => None,
        };
    }
}

pub trait MainCommand {
    fn execute(
        &self,
        args: Vec<String>,
        logging_container: &LoggingContainer,
        config_container: &ConfigContainer,
        command_container: &CommandContainer,
        buildin_commands: &HashMap<String, Box<Execution<i32>>>,
    ) -> i32;
    fn get_command_name(&self) -> String;
    fn get_command_prefix(&self) -> String;
    fn get_description(&self) -> String;
}
