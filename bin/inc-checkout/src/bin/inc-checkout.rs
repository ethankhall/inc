extern crate inc_core;
extern crate inc_commands;

use inc_commands::build_checkout_command;
use inc_commands::mains::sub_command_run;
use std::process;
use std::env::args;
use std::collections::HashMap;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let command = build_checkout_command();
    return sub_command_run(args().collect(), &command, HashMap::new());
}
