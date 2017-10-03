extern crate etrain_core;
extern crate etrain_main_entrypoint_lib;

use etrain_main_entrypoint_lib::exe::build_main_command;
use etrain_core::mains::root_main;
use std::process;

fn main() {
    let exit_code = do_main();
    process::exit(exit_code);
}

fn do_main() -> i32 {
    let command = build_main_command();
    return root_main(&command);
}