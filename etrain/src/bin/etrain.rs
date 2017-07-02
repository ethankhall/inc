extern crate clap;
#[macro_use]
extern crate slog;
extern crate etrain;

use std::collections::HashSet;
use clap::{Arg, App, SubCommand};
use std::env;
use std::fs::{self, DirEntry, ReadDir};
use slog::{Logger, Level};


fn main() {
    let matches = App::new("etrain")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ethan Hall")
        .about("Buy the ticket on the dev box, end up in CI.")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let logger = match matches.occurrences_of("v") {
        0 => etrain::initialization::init_logging(Level::Warning),
        1 => etrain::initialization::init_logging(Level::Info),
        2 => etrain::initialization::init_logging(Level::Debug),
        3 | _ => etrain::initialization::init_logging(Level::Trace),
    };

    list_sub_commands(&logger);
}

fn list_sub_commands(logger: &Logger) -> HashSet<String> {
    let mut sub_commands: HashSet<String> = HashSet::new();

    if let Ok(path) = env::var("PATH") {
        for split_path in path.split(":") {
            debug!(logger, "Processing {} for erail executables", split_path);
            for entry in fs::read_dir(split_path) {

                process_dir_read(logger, &mut sub_commands, entry);
            }
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


fn process_dir_entry(logger: &Logger, sub_commands: &mut HashSet<String>, dir_entry: DirEntry) {
    let file_name = dir_entry.file_name().into_string();

    if file_name.is_err() {
        debug!(logger, "Unable to process entry");
        return;
    }

    let file_name = file_name.unwrap();

    if file_name.starts_with("erail-") {
        if file_is_executable(dir_entry) {
            sub_commands.insert(file_name.split("-").nth(3).unwrap().to_string());
            debug!(logger, "Found command {}", file_name);
        } else {
            debug!(logger, "{} was not a command", file_name);
        };
    }
}

fn process_dir_read(logger: &Logger, sub_commands: &mut HashSet<String>, dir_read: ReadDir) {
    for entry in dir_read {
        match entry {
            Ok(ent) => process_dir_entry(logger, sub_commands, ent),
            Err(_) => {debug!(logger, "Unable to read dir")}
        }
    }
}
