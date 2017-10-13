use std::collections::HashMap;
use libs::process::SystemCommand;
use core::config::ConfigContainer;
use core::cli::find_commands_avalible;

#[derive(Debug, Clone)]
pub struct CommandContainer {
    pub commands: HashMap<String, SystemCommand>,
}

impl CommandContainer {
    pub fn new() -> Self {
        CommandContainer { commands: find_commands_avalible() }
    }

    pub fn find_sub_commands(&self, command: String) -> Option<SystemCommand> {
        return match self.commands.get(&(command)) {
            Some(value) => Some(value.clone()),
            None => None,
        };
    }
}

pub trait MainCommand {
    fn execute(&self, args: &Vec<String>, config: ConfigContainer, commands: CommandContainer) -> i32;
    fn get_command_name(&self) -> String;
    fn get_command_prefix(&self) -> String;
    fn get_description(&self) -> String;
}
