#[macro_use]
extern crate slog;
extern crate etrain_core;

use slog::Logger;
use std::collections::{LinkedList, HashSet};
use std::env::{self, current_exe};
use etrain_core::logging::{logging, get_verbosity_level};
use etrain_core::cli::CliResolver;
use etrain_core::BASE_APPLICATION_NAME;
use std::string::String;
use std::process::{self, Command};
use std::result::Result;


fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

fn run() -> i32 {
    let log_level = get_verbosity_level();
    let app_name = String::from(BASE_APPLICATION_NAME);
    let logger = logging(log_level, &app_name);

    let cli_resolver = CliResolver { logger: logger.new(slog_o!()), prefix: app_name };
    let commands = cli_resolver.find_commands();
    let requested_command = build_sub_command_args(logger.clone());

    slog_trace!(logger, "Found commands: {:?}", commands);
    slog_trace!(logger, "Requested command: {:?}", requested_command);

    if let Err(value) = requested_command {
        slog_warn!(logger, "{}", value);
        print_help(logger, commands);
        return 1;
    }

    let requested_command = requested_command.unwrap();

    if requested_command.command == "help" {
        print_help(logger, commands);
        return 0;
    }

    if !commands.contains(&*requested_command.command) {
        slog_warn!(logger, "Unknown command `{}`", requested_command.command);
        print_help(logger, commands);
        return 1;
    }

    let command_name = format!("etrain-{}", requested_command.command);
    slog_trace!(logger, "Trying to execute {}", command_name);
    let mut child = Command::new(command_name)
        .args(requested_command.arguments)
        .env("ETRAIN_LOG_LEVEL", log_level.as_str())
        .env("PATH", build_path())
        .spawn()
        .expect("failed to execute process");

    let exit_status = child.wait().expect("failed to wait on child");

    return match exit_status.code() {
        Some(code) => code,
        None => 1,
    };
}

fn print_help(logger: Logger, available_commands: HashSet<String>) {
    slog_info!(logger, "usage: etrain [--verbose (-v)] <command> <args>");
    slog_info!(logger, "Available commands:");

    for command in available_commands {
        slog_info!(logger, "\t{}", command)
    }
}

fn build_path() -> String {
    let path = env::var("PATH").unwrap();
    return path + ":" +
        current_exe()
            .unwrap()
            .as_path()
            .parent()
            .unwrap()
            .to_str()
            .unwrap();
}

#[derive(Debug)]
struct SubCommandArguments {
    command: String,
    arguments: LinkedList<String>,
}

fn build_sub_command_args(logger: Logger) -> Result<SubCommandArguments, &'static str> {
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
        Some(p) => Ok(SubCommandArguments {
            command: p,
            arguments: arguments,
        }),
        None => Err("No command specified"),
    };
}
