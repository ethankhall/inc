extern crate inc_impl;

use inc_impl::commands::build_checkout_command;
use inc_impl::core::mains::sub_command_run;
use std::process;
use std::env::args;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let command = build_checkout_command();
    return sub_command_run(args().collect(), &command);
}
