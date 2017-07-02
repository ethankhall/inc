#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog;
extern crate etrain;

use std::vec::Vec;
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};
use std::env::{self, current_exe};
use std::fs::{self, DirEntry, ReadDir};
use slog::{Logger, Level};
use std::process::{exit, Command};
use etrain::initialization::{logging, log_level};


fn main() {
    let sub_commands: HashMap<String, String> = sub_commands();
    let mut sub_command_apps: Vec<App> = Vec::new();
    for sub_command in sub_commands.keys() {
        sub_command_apps.push(SubCommand::with_name(&*sub_command));
    }

    let matches = App::new("etrain")
        .version(crate_version!())
        .author("Ethan Hall")
        .about("Buy the ticket on the dev box, end up in CI.")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommands(sub_command_apps)
        .get_matches();

    let logger = logging(log_level(matches.occurrences_of("v")));
    let sub_command = matches.subcommand();

    let name = String::from(sub_command.0);
    let executable = sub_commands.get(&name).unwrap();
    debug!(logger, "Mapping {} to {}", name, executable);

    match Command::new(executable).status() {
        Ok(it) => exit(it.code().unwrap_or(1)),
        Err(e) => {
            error!(logger, "Unable to start process! {}", e);
            exit(1);
        }
    }
}

fn sub_commands() -> HashMap<String, String> {
    let level = match env::var("LOG_LEVEL") {
        Ok(value) => if "DEBUG" == value { Level::Debug } else { Level::Info },
        Err(_) => Level::Warning
    };

    let logger = &logging(level);
    let mut sub_commands: HashMap<String, String> = HashMap::new();

    if let Ok(path) = env::var("PATH") {
        for split_path in path.split(":") {
            debug!(logger, "Processing {} for erail executables", split_path);
            for entry in fs::read_dir(split_path) {
                process_dir_read(logger, &mut sub_commands, entry);
            }
        }
    }

    if let Ok(path) = current_exe() {
        let mut path = path.canonicalize().unwrap();
        path.pop();
        let path = path.as_path();
        debug!(logger, "Processing {:?} for erail executables", path);
        for entry in path.read_dir() {
            process_dir_read(logger, &mut sub_commands, entry);
        }
    }

    return sub_commands;
}

#[cfg(windows)]
fn file_is_executable(dir_entry: DirEntry) -> bool {
    return true;
}

#[cfg(unix)]
pub fn file_is_executable(dir_entry: DirEntry) -> bool {
    use std::os::unix::prelude::*;

    let permissions = dir_entry.metadata().unwrap().permissions();
    return permissions.mode() & 0o111 != 0
}


fn process_dir_entry(logger: &Logger, sub_commands: &mut HashMap<String, String>, dir_entry: DirEntry) {
    let file_name = dir_entry.file_name().into_string();

    if file_name.is_err() {
        debug!(logger, "Unable to process entry");
        return;
    }

    let path = dir_entry.path();
    let file_name = file_name.unwrap();

    if file_name.starts_with("etrain-") {
        if file_is_executable(dir_entry) {
            let command_name = file_name.split("-").nth(1).unwrap().to_string();
            sub_commands.insert(command_name, path.canonicalize().unwrap().as_path().to_str().unwrap().to_string());
            debug!(logger, "Found command {}", file_name);
        } else {
            trace!(logger, "{} was not a command", file_name);
        };
    }
}

fn process_dir_read(logger: &Logger, sub_commands: &mut HashMap<String, String>, dir_read: ReadDir) {
    for entry in dir_read {
        match entry {
            Ok(ent) => process_dir_entry(logger, sub_commands, ent),
            Err(_) => { debug!(logger, "Unable to read dir") }
        }
    }
}
