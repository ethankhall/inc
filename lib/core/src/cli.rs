extern crate slog;

use std::collections::{HashSet, HashMap};
use std::env::{self, current_exe};
use std::fs::{self, DirEntry, ReadDir};
use slog::Logger;
use BASE_APPLICATION_NAME;

pub fn find_commands_avalible(logger: &Logger) -> HashMap<String, Vec<String>> {
    let commands = find_commands(logger);
    return convert_command_set_to_map(commands);
}

fn convert_command_set_to_map(commands: HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut commands: Vec<String> = commands.into_iter().collect();
    let mut return_map: HashMap<String, Vec<String>> = HashMap::new();

    //Get the string into length order
    commands.sort_by(|a, b| a.len().cmp(&(b.len())));

    for command in commands {
        if let Some(key) = find_key_from_command(&command, &return_map) {
            let map_value = return_map.get_mut(key.as_str());
            map_value.unwrap().push(command);
        } else {
            return_map.insert(command, vec![]);
        }
    }

    return return_map;
}


fn find_key_from_command(command: &String, map: &HashMap<String, Vec<String>>) -> Option<String> {
    let split_command: Vec<String> = (*command).clone().as_str().split("-").map(|x| String::from(x)).collect();
    for i in 0..(split_command.len()) {
        let joint = split_command[0..(i)].join("-");
        if map.contains_key(joint.as_str()) {
            return Some(joint);
        }
    }

    return None;
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;
    use super::*;

    #[test]
    fn will_handle_werdly_named_apps() {
        let input: Vec<String> = ["a-b", "a-b-c-d", "a-b-c"].iter().map(|x| String::from(*x)).collect();
        let input = HashSet::from_iter(input);

        let result = convert_command_set_to_map(input);
        assert!(result.contains_key("a-b"), "contains the 'a-b' command");
        assert_eq!(result.len(), 1, "There should be only one entry");
        assert_eq!(result.get("a-b").unwrap().len(), 2, "Should have two elements in it");
        assert!(result.get("a-b").unwrap().contains(&String::from("a-b-c")));
        assert!(result.get("a-b").unwrap().contains(&String::from("a-b-c-d")));
        assert!(!result.get("a-b").unwrap().contains(&String::from("a-b-c-d-e")));
    }

    #[test]
    fn order_will_not_matter() {
        let input: Vec<String> = ["a-b-c-d", "a-b", "a-b-c"].iter().map(|x| String::from(*x)).collect();
        let input = HashSet::from_iter(input);

        let result = convert_command_set_to_map(input);
        assert!(result.contains_key("a-b"), "contains the 'a-b' command");
        assert_eq!(result.len(), 1, "There should be only one entry");
        assert_eq!(result.get("a-b").unwrap().len(), 2, "Should have two elements in it");
        assert!(result.get("a-b").unwrap().contains(&String::from("a-b-c")));
        assert!(result.get("a-b").unwrap().contains(&String::from("a-b-c-d")));
        assert!(!result.get("a-b").unwrap().contains(&String::from("a-b-c-d-e")));
    }
}

fn find_commands(logger: &Logger) -> HashSet<String> {
    let mut sub_commands: HashSet<String> = HashSet::new();

    if let Ok(path) = env::var("PATH") {
        for split_path in path.split(":") {
            debug!(
                logger,
                "Processing {} for {} executables",
                BASE_APPLICATION_NAME,
                split_path
            );
            for entry in fs::read_dir(split_path) {
                process_dir_read(logger, &mut sub_commands, entry);
            }
        }
    }

    if let Ok(path) = current_exe() {
        let mut path = path.canonicalize().unwrap();
        path.pop();
        let path = path.as_path();
        debug!(logger, "Processing {:?} for {:?} executables", BASE_APPLICATION_NAME, path);
        for entry in path.read_dir() {
            process_dir_read(logger, &mut sub_commands, entry);
        }
    }

    return sub_commands;
}

fn process_dir_read(logger: &Logger, sub_commands: &mut HashSet<String>, dir_read: ReadDir) {
    for entry in dir_read {
        match entry {
            Ok(ent) => process_dir_entry(logger, sub_commands, ent),
            Err(_) => debug!(logger, "Unable to read dir"),
        }
    }
}

fn process_dir_entry(logger: &Logger, sub_commands: &mut HashSet<String>, dir_entry: DirEntry) {
    let file_name = dir_entry.file_name().into_string();

    if file_name.is_err() {
        debug!(logger, "Unable to process entry");
        return;
    }

    // let path = dir_entry.path();
    let file_name: String = file_name.unwrap();

    if file_name.starts_with(format!("{}-", BASE_APPLICATION_NAME).as_str()) {
        if file_is_executable(dir_entry) {
            sub_commands.insert(file_name[7..].to_string());
            debug!(logger, "Found command {}", file_name);
        }
    }
}

#[cfg(windows)]
fn file_is_executable(dir_entry: DirEntry) -> bool {
    return true;
}

#[cfg(unix)]
fn file_is_executable(dir_entry: DirEntry) -> bool {
    use std::os::unix::prelude::*;

    let permissions = dir_entry.metadata().unwrap().permissions();
    return permissions.mode() & 0o111 != 0;
}