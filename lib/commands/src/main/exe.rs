use main::args::build_sub_command_args;
use std::collections::HashMap;
use std::vec::Vec;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::core::command::{LoggingContainer, MainCommand, CommandContainer};
use inc_core::core::config::ConfigContainer;
use inc_core::libs::process::SystemCommand;
use std::fmt::Write;
use inc_core::exec::system::SystemExecution;
use inc_core::exec::executor::Executor;
use inc_core::exec::Execution;

pub struct MainEntryPoint {
    pub internal_commands: HashMap<String, Box<Execution<i32>>>
}

impl MainCommand for MainEntryPoint {
    fn execute(
        &self,
        args: Vec<String>,
        logging_container: &LoggingContainer,
        _config_container: &ConfigContainer,
        command_container: &CommandContainer,
    ) -> i32 {
        let logger = logging_container.logger;
        let requested_command = build_sub_command_args(logger, args);
        let commands: Vec<&SystemCommand> = command_container.commands.values().collect();
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

        let avaliable_command = commands.iter().find(
            |x| x.alias == requested_command.command,
        );
        if avaliable_command.is_none() {
            slog_warn!(logger, "Unknown command `{}`", requested_command.command);
            slog_info!(logger, "{}", help_message);
            return 1;
        }

        let avaliable_command = avaliable_command.unwrap();

        let executor = Executor { logger: logger.clone() };
        let execution = SystemExecution { 
            command: avaliable_command.binary.clone().path, 
            log_level: logging_container.level.clone(), 
            logger: logger.clone() 
        };
        
        let result = executor.execute(&execution, &(requested_command.arguments));

        return match result {
            Ok(expr) => expr,
            Err(_) => -1
        };
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

fn build_help(available_commands: &Vec<&SystemCommand>) -> String {
    let mut help = String::new();
    write!(&mut help, "usage: inc [--verbose (-v)] <command> <args>\n").unwrap();
    write!(&mut help, "Available commands:\n").unwrap();

    for command in available_commands.iter() {
        write!(&mut help, "\t{}\n", command.alias).unwrap();
    }

    return help;
}
