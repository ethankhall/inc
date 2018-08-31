use clap::{App, ArgMatches, SubCommand};
use inc_lib::core::config::ConfigContainer;
use inc_lib::core::command::AvaliableCommands;
use inc_lib::exec::executor::CliResult;
use inc_lib::core::BASE_APPLICATION_NAME;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    return SubCommand::with_name("list").about("List the known element for Inc.");
}

pub fn execute(
    _args: &ArgMatches,
    avaliable_commands: AvaliableCommands,
    _config: ConfigContainer,
) -> CliResult {
    let mut commands: Vec<String> = vec![s!("checkout"), s!("exec"), s!("list")];
    avaliable_commands
        .find_commands_with_parent(BASE_APPLICATION_NAME)
        .into_iter()
        .for_each(|command| {
            commands.push(format!(
                "{} - External command from {:?}",
                command.name(),
                command.binary().path
            ))
        });

    let commands: Vec<String> = commands.iter().map(|x| format!("  - {}", x)).collect();
    info!("avaliable-commands:\n{}", commands.join("\n"));
    return Ok(0);
}
