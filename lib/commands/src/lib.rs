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

use inc_core::core::command::MainCommand;
use checkout::exe::CheckoutCommand;
use root::exe::MainEntryPoint;
use std::collections::HashMap;

pub(crate) mod checkout;
pub(crate) mod root;
pub mod logging;
pub mod mains;

pub fn build_checkout_command() -> impl MainCommand {
    return CheckoutCommand {};
}

pub fn build_main_command() -> impl MainCommand {
    return MainEntryPoint { internal_commands: HashMap::new() };
}
