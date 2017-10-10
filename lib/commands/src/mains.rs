use inc_core::core::command::{MainCommand, CommandContainer};
use inc_core::core::cli::find_commands_avalible;
use logging::{parse_from_args, configure_logging};
use inc_core::core::config::ConfigContainer;
use std::vec::Vec;
use std::env::args;

pub fn sub_command_run<F>(
    break_on_command: bool,
    args: Vec<String>,
    generator: F
) -> i32 
    where F: Fn(ConfigContainer, CommandContainer) -> Box<MainCommand> {

    let level = parse_from_args(&args, break_on_command);
    configure_logging(Some(level));

    let config_container = ConfigContainer::new();
    let commands = find_commands_avalible();

    let command_conatiner = CommandContainer { commands: commands };

    let main = generator(config_container, command_conatiner);
    debug!("Starting {}", main.get_command_name());

    return main.execute(&args);
}

pub fn root_main<F>(generator: F) -> i32 
    where F: Fn(ConfigContainer, CommandContainer) -> Box<MainCommand> {
    return sub_command_run(true, args().collect(), generator);
}
