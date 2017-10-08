use inc_core::core::command::{MainCommand, CommandContainer};
use inc_core::core::cli::find_commands_avalible;
use logging::{parse_from_args, configure_logging};
use inc_core::core::config::ConfigContainer;
use std::vec::Vec;
use std::env::args;
use std::collections::HashMap;
use inc_core::exec::Execution;

pub fn sub_command_run<T: MainCommand>(
    args: Vec<String>,
    command: &T,
    baked_commands: HashMap<String, Box<Execution<i32>>>,
) -> i32 {
    let level = parse_from_args(&args);
    configure_logging(Some(level));
    let config_container = ConfigContainer::new();
    let commands = find_commands_avalible();

    let command_conatiner = CommandContainer { commands: commands };

    return command.execute(
        args,
        &config_container,
        &command_conatiner,
        &baked_commands,
    );
}

pub fn root_main<T: MainCommand>(command: &T) -> i32 {
    return sub_command_run(args().collect(), command, HashMap::new());
}
