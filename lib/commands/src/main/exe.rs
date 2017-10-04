use std::env::{current_exe, var};
use slog::{Logger, Level};
use main::args::build_sub_command_args;
use std::process::{Command};
use std::collections::{HashMap};
use std::vec::Vec;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::core::command::{LoggingContainer, MainCommand, CommandContainer};
use inc_core::core::config::ConfigContainer;
use inc_core::libs::process::SystemCommand;
use std::fmt::Write;

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
        sub_commands: &HashMap<String, SystemCommand>) -> i32 {
    
    let requested_command = build_sub_command_args(logger, args);
    let commands: Vec<&SystemCommand> = sub_commands.values().collect();

    slog_trace!(logger, "Found commands: {:?}", commands);
    slog_trace!(logger, "Requested command: {:?}", requested_command);

    let help_message = build_help(&commands);

    if let Err(value) = requested_command {
        slog_warn!(logger, "{}", value);
        slog_info!(logger, "{}", help_message);
        return 1;
    }

    let requested_command = requested_command.unwrap();

    if requested_command.command == "help" {
        slog_info!(logger, "{}", help_message);
        return 0;
    }

    let avaliable_command = commands.iter().find(|x| x.alias == requested_command.command);
    if avaliable_command.is_none() {
        slog_warn!(logger, "Unknown command `{}`", requested_command.command);
        slog_info!(logger, "{}", help_message);
        return 1;
    }

    let avaliable_command = avaliable_command.unwrap();

    let command_path = &avaliable_command.binary.path;
    slog_trace!(logger, "Trying to execute {:?}", command_path);
    let mut child = Command::new(command_path)
        .args(requested_command.arguments)
        .env("INC_LOG_LEVEL", log_level.as_str())
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

fn build_help(available_commands: &Vec<&SystemCommand>) -> String {
    let mut help = String::new();
    write!(&mut help, "usage: inc [--verbose (-v)] <command> <args>\n");
    write!(&mut help, "Available commands:\n");

    for command in available_commands.iter() {
        write!(&mut help, "\t{}", command.alias);
    }

    return help
}
