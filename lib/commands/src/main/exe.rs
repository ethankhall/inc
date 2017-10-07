use std::collections::HashMap;
use std::vec::Vec;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::core::command::{LoggingContainer, MainCommand, CommandContainer};
use inc_core::core::config::ConfigContainer;
use inc_core::libs::process::SystemCommand;
use inc_core::exec::system::SystemExecution;
use inc_core::exec::executor::Executor;
use inc_core::exec::Execution;
use main::help::{HelpCommand, HelpArgs};
use docopt::Docopt;

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
        buildin_commands: &HashMap<String, Box<Execution<i32>>>
    ) -> i32 {
        let logger = logging_container.logger;
        let commands: Vec<&SystemCommand> = command_container.commands.values().collect();
        let help_command = HelpCommand::new(logger, &commands);

        let doc_opts: HelpArgs = Docopt::new(help_command.build_help_message())
            .and_then(|d| d.argv(args.into_iter()).parse())
            .and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let executor = Executor::new(logger);

        if doc_opts.arg_command == "help" {
            slog_info!(logger, "{}", help_command.build_help_message());
            return 0
        }

        let command_search = commands.iter().find(
            |x| x.alias == doc_opts.arg_command,
        );

        let args = doc_opts.arg_args.unwrap_or_else(|| Vec::new());
        if let Some(system_command) = command_search {
            let command = SystemExecution { 
                command: system_command.binary.clone().path, 
                log_level: logging_container.level.clone(), 
                logger: logger.clone() 
            };

            let result = executor.execute(&command, &args);

            return match result {
                Ok(expr) => expr,
                Err(_) => -1
            };
        }

        match buildin_commands.get(doc_opts.arg_command.as_str()) {
            Some(execution) => {
                let result = executor.execute(execution.as_ref(), &args);

                return match result {
                    Ok(expr) => expr,
                    Err(_) => -1
                };
            }
            None => {
                slog_warn!(logger, "Unknown command `{}`", doc_opts.arg_command);
                slog_warn!(logger, "Run `{} help` for a list of commands", BASE_APPLICATION_NAME);
                return 1;
            }
        }
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