use core::command::MainCommand;
use self::checkout::exe::CheckoutCommand;
use self::main::exe::MainEntryPoint;

pub(crate) mod checkout;
pub(crate) mod main;

pub fn build_checkout_command() -> impl MainCommand {
    return CheckoutCommand{};
}

pub fn build_main_command() -> impl MainCommand {
    return MainEntryPoint { };
}