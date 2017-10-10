use std::collections::HashMap;
use std::vec::Vec;
use inc_core::core::BASE_APPLICATION_NAME;
use inc_core::core::command::{MainCommand, CommandContainer};
use inc_core::libs::process::SystemCommand;
use inc_core::exec::system::SystemExecution;
use inc_core::exec::executor::Executor;
use inc_core::exec::Execution;
use root::help::{HelpCommand, HelpArgs};
use docopt::Docopt;

pub struct MainEntryPoint {
    pub internal_commands: HashMap<String, Box<Execution<i32>>>,
    pub command_container: CommandContainer
}

impl MainCommand for MainEntryPoint {
    fn execute(&self, args: &Vec<String>) -> i32 {
        let commands: Vec<&SystemCommand> = self.command_container.commands.values().collect();
        let help_command = HelpCommand::new(&commands, self.internal_commands.keys().collect());

        let doc_opts: HelpArgs = Docopt::new(help_command.build_help_message())
            .and_then(|d| { 
                d.options_first(true).help(false).argv(args.into_iter()).parse()
            }).and_then(|d| d.deserialize())
            .unwrap_or_else(|e| e.exit());

        let executor = Executor::new();

        if doc_opts.arg_command == "help" || doc_opts.flag_help {
            info!("{}", help_command.build_help_message());
            return 0;
        }

        let command_search = commands.iter().find(|x| x.alias == doc_opts.arg_command);

        let args = doc_opts.arg_args.unwrap_or_else(|| Vec::new());
        if let Some(system_command) = command_search {
            let command = SystemExecution {
                command: system_command.binary.clone().path,
            };

            let result = executor.execute(&command, &args);

            return match result {
                Ok(expr) => expr,
                Err(_) => -1,
            };
        }

        match self.internal_commands.get(doc_opts.arg_command.as_str()) {
            Some(execution) => {
                let result = executor.execute(execution.as_ref(), &args);

                return match result {
                    Ok(expr) => expr,
                    Err(_) => -1,
                };
            }
            None => {
                warn!("Unknown command `{}`", doc_opts.arg_command);
                warn!(
                    "Run `{} help` for a list of commands",
                    BASE_APPLICATION_NAME
                );
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
