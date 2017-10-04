use std::env::{current_exe, var};
use slog::{Logger, Level};
use commands::main::args::build_sub_command_args;
use std::process::{Command};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use core::BASE_APPLICATION_NAME;
use core::command::{LoggingContainer, MainCommand, CommandContainer};
use core::config::ConfigContainer;

pub struct MainEntryPoint {
}

impl MainCommand for MainEntryPoint {
    fn execute(&self, args: Vec<String>, 
        logging_container: &LoggingContainer, _config_container: &ConfigContainer, 
        command_container: &CommandContainer) -> i32 {
            return entrypoint(args, logging_container.logger, logging_container.level, &command_container.commands);
        }

    fn get_command_name(&self) -> String {
        return String::from(BASE_APPLICATION_NAME);
    }

    fn get_command_prefix(&self) -> String {
        return String::from(BASE_APPLICATION_NAME);
    }

    fn get_description(&self) -> String {
        return String::from("Command that delegates to other sub-commands");
    }
}

fn entrypoint(args: Vec<String>, 
        logger: &Logger,
        log_level: &Level,
        sub_commands: &HashMap<String, Vec<String>>) -> i32 {
    
    let requested_command = build_sub_command_args(logger, args);
    let commands = sub_commands.keys().map(|x| (x.clone())).collect();

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

fn build_path() -> String {
    let path_extension = if let Ok(path) = current_exe() {
        let mut path = path.canonicalize().unwrap();
        path.pop();
        format!(":{}", path.as_os_str().to_str().unwrap())
    } else {
        String::new()
    };

    let path = var("PATH").unwrap();
    return format!("{}{}", path, path_extension);
}

fn print_help(logger: &Logger, available_commands: HashSet<String>) {
    slog_info!(logger, "usage: etrain [--verbose (-v)] <command> <args>");
    slog_info!(logger, "Available commands:");

    for command in available_commands {
        slog_info!(logger, "\t{}", command)
    }
}
