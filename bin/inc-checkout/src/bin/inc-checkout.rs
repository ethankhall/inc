extern crate inc_core;

use inc_core::commands::build_checkout_command;
use inc_core::core::mains::sub_command_run;
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
