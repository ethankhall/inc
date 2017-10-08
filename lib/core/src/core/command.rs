use std::collections::HashMap;
use libs::process::SystemCommand;

#[derive(Debug, Clone)]
pub struct CommandContainer {
    pub commands: HashMap<String, SystemCommand>,
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
    fn execute(&self, args: &Vec<String>) -> i32;
    fn get_command_name(&self) -> String;
    fn get_command_prefix(&self) -> String;
    fn get_description(&self) -> String;
}
