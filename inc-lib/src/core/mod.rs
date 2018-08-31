pub mod config;
pub mod command;
pub mod logging;

#[cfg(test)]
pub(crate) mod config_test;

pub const BASE_APPLICATION_NAME: &'static str = "inc";

pub trait MainCommand {
    fn execute(
        &self,
        args: &Vec<String>,
        config: config::ConfigContainer,
        commands: command::AvaliableCommands,
    ) -> i32;
    fn get_command_name(&self) -> String;
    fn get_command_prefix(&self) -> String;
    fn get_description(&self) -> String;
}
