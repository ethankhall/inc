#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
extern crate inc_core;
extern crate fern;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate chrono;

use inc_core::core::command::CommandContainer;
use checkout::exe::CheckoutCommand;
use root::exe::MainEntryPoint;
use std::collections::HashMap;
use inc_core::core::config::ConfigContainer;
use inc_core::exec::Execution;

pub(crate) mod checkout;
pub(crate) mod root;
pub mod logging;
pub mod mains;

pub fn build_checkout_command(config: ConfigContainer, command: CommandContainer) -> CheckoutCommand {
    return CheckoutCommand {
        config_container: config,
        command_container: command
    };
}

pub fn build_main_command(command: CommandContainer) -> MainEntryPoint {
    return MainEntryPoint { 
        internal_commands: HashMap::new(),
        command_container: command
    };
}

pub fn build_fat_main_command(config: ConfigContainer, command: CommandContainer) -> MainEntryPoint {
    let mut sub_commands: HashMap<String, Box<Execution<i32>>> = HashMap::new();
    sub_commands.insert(String::from("checkout"), Box::new(build_checkout_command(config, command.clone())));

    return MainEntryPoint { 
        internal_commands: sub_commands,
        command_container: command
    };
}