use core::command::{MainCommand, CommandContainer, LoggingContainer};
use core::cli::find_commands_avalible;
use core::logging::{get_verbosity_level, logging};
use core::config::ConfigContainer;
use std::vec::Vec;
use std::env::args;

pub fn sub_command_run<T: MainCommand>(args: Vec<String>, command: &T) -> i32 {
    let name = command.get_command_name();
    let level = get_verbosity_level();
    let logger = logging(level, &name);
    let config_container = ConfigContainer::new();
    let commands = find_commands_avalible(&logger);

    let command_conatiner = CommandContainer { commands: commands };
    let logging_container = LoggingContainer { logger: &logger, level: &level};
    
    return command.execute(args, &logging_container, &config_container, &command_conatiner);
}

pub fn root_main<T: MainCommand>(command: &T) -> i32 {
    return sub_command_run(args().collect(), command);
}