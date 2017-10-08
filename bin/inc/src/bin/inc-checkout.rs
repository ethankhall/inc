extern crate inc_core;
extern crate inc_commands;

use inc_commands::build_checkout_command;
use inc_commands::mains::sub_command_run;
use std::process;
use std::env::args;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 { 
    return sub_command_run(args().collect(), |config, command| { Box::new(build_checkout_command(config, command)) });
}
