#[macro_use]
extern crate slog;
extern crate etrain_core;
extern crate etrain_checkout_lib;

use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::config::{ConfigParser, ConfigContainer, ConfigSource, ConfigValue};
use etrain_core::cli::CliResolver;
use etrain_checkout_lib::command::build_checkout_command;
use etrain_core::command::MainCommand;
use etrain_core::BASE_APPLICATION_NAME;
use std::process;
use std::env::args;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let command = build_checkout_command();

    let name = command.name();
    let logger = logging(get_verbosity_level(), &name);
    let cli_resolver = CliResolver { logger: logger.clone(), prefix: format!("{}-{}", BASE_APPLICATION_NAME, name)};
    let commands = cli_resolver.find_commands();
    let config_container = ConfigContainer::new();

    let args = args().collect();
    return command.execute(args, &logger, &config_container, commands.into_iter().collect());
}
