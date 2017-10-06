use std::collections::LinkedList;
use slog::Logger;

#[derive(Debug)]
pub(crate) struct SubCommandArguments {
    pub(crate) command: String,
    pub(crate) arguments: LinkedList<String>,
}

pub(crate) fn build_sub_command_args(
    logger: &Logger,
    args: Vec<String>,
) -> Result<SubCommandArguments, &'static str> {
    let mut arguments: LinkedList<String> = LinkedList::new();
    let mut command: Option<String> = None;

    let mut in_sub_command = false;

    for argument in args.iter().skip(1) {
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
