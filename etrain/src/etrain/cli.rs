extern crate slog;
extern crate yaml_rust;

use std::path::Path;
use std::collections::HashSet;
use std::env::{self, current_exe};
use std::fs::{self, DirEntry, ReadDir};
use slog::Logger;

pub struct CliResolver {
    pub logger: Logger
}

impl CliResolver {
    pub fn find_command(&self, command_name: String) -> Option<String> {
        let prefix_command = &*format!("etrain-{}", command_name);
        let commands = self.find_commands();
        let commands = commands.iter()
            .map(|it| *&(Path::new(it).as_os_str().to_str().unwrap()))
            .filter(|it| it.starts_with(prefix_command)).collect::<Vec<_>>();

        if commands.is_empty() {
            return None;
        }

        return Some(String::from(commands[0]));
    }

    pub fn find_commands(&self) -> HashSet<String> {
        let mut sub_commands: HashSet<String> = HashSet::new();

        if let Ok(path) = env::var("PATH") {
            for split_path in path.split(":") {
                debug!(self.logger, "Processing {} for erail executables", split_path);
                for entry in fs::read_dir(split_path) {
                    self.process_dir_read(&mut sub_commands, entry);
                }
            }
        }

        if let Ok(path) = current_exe() {
            let mut path = path.canonicalize().unwrap();
            path.pop();
            let path = path.as_path();
            debug!(self.logger, "Processing {:?} for erail executables", path);
            for entry in path.read_dir() {
                self.process_dir_read(&mut sub_commands, entry);
            }
        }

        return sub_commands;
    }

    #[cfg(windows)]
    fn file_is_executable(&self, dir_entry: DirEntry) -> bool {
        return true;
    }

    #[cfg(unix)]
    fn file_is_executable(&self, dir_entry: DirEntry) -> bool {
        use std::os::unix::prelude::*;

        let permissions = dir_entry.metadata().unwrap().permissions();
        return permissions.mode() & 0o111 != 0;
    }


    fn process_dir_entry(&self, sub_commands: &mut HashSet<String>, dir_entry: DirEntry) {
        let file_name = dir_entry.file_name().into_string();

        if file_name.is_err() {
            debug!(self.logger, "Unable to process entry");
            return;
        }

        let path = dir_entry.path();
        let file_name: String = file_name.unwrap();
        let prefix = "etrain-";

//        debug!(self.logger, "element: {}", file_name);

        if file_name.starts_with(&*prefix) {
            if self.file_is_executable(dir_entry) {
                sub_commands.insert(file_name[7..].to_string());
                debug!(self.logger, "Found command {}", file_name);
            } else {
                trace!(self.logger, "{} was not a command", file_name);
            };
        }
    }

    fn process_dir_read(&self, sub_commands: &mut HashSet<String>, dir_read: ReadDir) {
        for entry in dir_read {
            match entry {
                Ok(ent) => self.process_dir_entry(sub_commands, ent),
                Err(_) => { debug!(self.logger, "Unable to read dir") }
            }
        }
    }
}