extern crate inc_core;
extern crate inc_commands;

use inc_commands::build_fat_main_command;
use inc_commands::mains::sub_command_run;
use std::env::args;
use std::process;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    return sub_command_run(args().collect(), |config, command| { Box::new(build_fat_main_command(config, command)) });
}
