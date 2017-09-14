#[macro_use]
extern crate slog;
extern crate etrain;

use slog::Logger;
use std::collections::{LinkedList, HashSet};
use std::env::{self, current_exe};
use etrain::initialization::{logging, log_level};
use etrain::cli::CliResolver;
use std::string::String;
use std::process::{Command,self};


fn main() {
    let log_level_int = get_verbosity_level();
    let logger = logging(log_level(log_level_int), "etrain");

    let cli_resolver = CliResolver { logger: logger.new(slog_o!()) };
    let commands = cli_resolver.find_commands();

    let requested_command = build_sub_command_args(logger);

    if requested_command.command == "help" {
        print_help(commands);
        return;
    }

    if commands.contains(&*requested_command.command) {
        println!("Unknown command `{}`", requested_command.command);
        print_help(commands);
        process::exit(1);
    }

    let mut child = Command::new(requested_command.command)
        .args(requested_command.arguments)
        .env("ETRAIN_LOG_LEVEL", log_level_int.to_string())
        .env("PATH", build_path())
        .spawn()
        .expect("failed to execute process");

    let exit_status = child.wait()
        .expect("failed to wait on child");


    let exit_code = match exit_status.code() {
        Some(code) => code,
        None       => 1
    };

    process::exit(exit_code);
}

fn print_help(available_commands: HashSet<String>) {
    println!("usage: etrain [--verbose (-v)] <command> <args>");
    println!();
    println!("Available commands:");

    for command in available_commands {
        println!("\t{}", command)
    }
}

fn build_path() -> String {
    let path = env::var("PATH").unwrap();
    return path + ":" + current_exe().unwrap().as_path().parent().unwrap().to_str().unwrap()
}

struct SubCommandArguments {
    command: String,
    arguments: LinkedList<String>
}

fn build_sub_command_args(logger: Logger) -> SubCommandArguments {
    let mut arguments: LinkedList<String> = LinkedList::new();
    let mut command: Option<String> = None;

    let raw_args = env::args();
    let mut in_sub_command = false;

    for argument in raw_args.skip(1) {
        let argument = &argument;
        slog_debug!(logger, "parse argument: {}", *argument);
        if !in_sub_command && !argument.starts_with("-") {
            in_sub_command = true;
            command = Some(argument.clone());
            slog_debug!(logger, "Setting command to execute: {}", *argument);
            continue;
        };

        if in_sub_command {
            arguments.push_back(argument.clone());
        }
    }

    return match command {
        Some(p) => SubCommandArguments { command: p, arguments: arguments },
        None => SubCommandArguments { command: String::from("help"), arguments: LinkedList::new() }
    };
}

fn get_verbosity_level() -> u64 {
    let mut verbose_level = 1;
    for argument in env::args() {
        if argument == "--verbose" {
            verbose_level = verbose_level + 1;
        }

        if argument == "-v" {
            verbose_level = verbose_level + 1;
        }

        if argument.starts_with("-v=") {
            if let Some(count) = argument.get(3..) {
                if let Ok(value) = count.parse() {
                    verbose_level = value;
                }
            }
        }
    }

    return verbose_level;
}