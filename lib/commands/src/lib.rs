#![feature(conservative_impl_trait)]

#[macro_use]
extern crate slog;
extern crate inc_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;

use inc_core::core::command::MainCommand;
use checkout::exe::CheckoutCommand;
use main::exe::MainEntryPoint;

pub(crate) mod checkout;
pub(crate) mod main;

pub fn build_checkout_command() -> impl MainCommand {
    return CheckoutCommand{};
}

pub fn build_main_command() -> impl MainCommand {
    return MainEntryPoint { };
}