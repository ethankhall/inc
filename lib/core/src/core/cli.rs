use std::collections::{HashSet, HashMap};
use std::env::{self, current_exe};
use std::fs::{self, DirEntry, ReadDir};
use core::BASE_APPLICATION_NAME;
use libs::process::{SystemBinary, SystemCommand};

pub fn find_commands_avalible() -> HashMap<String, SystemCommand> {
    let commands = find_commands();
    return convert_command_set_to_map(commands);
}

fn convert_command_set_to_map(commands: HashSet<SystemBinary>) -> HashMap<String, SystemCommand> {
    let mut commands: Vec<SystemBinary> = commands.into_iter().collect();
    let mut return_map: HashMap<String, SystemCommand> = HashMap::new();

    //Get the string into length order
    commands.sort_by(|a, b| a.name.len().cmp(&(b.name.len())));

    for command in commands {
        if let Some(key) = find_key_from_command(&command, &return_map) {
            let map_value = return_map.get_mut(key.as_str());
            map_value.unwrap().sub_commands.push(command);
        } else {
            let alias_prefix = format!("{}-", BASE_APPLICATION_NAME);
            let alias = String::from(&command.name[(alias_prefix.len())..]);
            return_map.insert(
                command.clone().name,
                SystemCommand {
                    binary: command,
                    alias: alias,
                    sub_commands: vec![],
                },
            );
        }
    }

    return return_map;
}


fn find_key_from_command(
    command: &SystemBinary,
    map: &HashMap<String, SystemCommand>,
) -> Option<String> {
    let split_command: Vec<String> = (*command)
        .clone()
        .name
        .as_str()
        .split("-")
        .map(|x| String::from(x))
        .collect();
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
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn will_handle_werdly_named_apps() {
        let input: Vec<SystemBinary> = ["a-b", "a-b-c-d", "a-b-c"]
            .iter()
            .map(|x| {
                SystemBinary {
                    path: PathBuf::new(),
                    name: format!("inc-{}", x),
                }
            })
            .collect();
        let input = HashSet::from_iter(input);

        let result = convert_command_set_to_map(input);
        assert!(result.contains_key("inc-a-b"), "contains the 'a-b' command");
        assert_eq!(result.len(), 1, "There should be only one entry");
        let command = result.get("inc-a-b").expect("a-b doesn't exist");
        assert_eq!(
            command.sub_commands.len(),
            2,
            "Should have two elements in it"
        );

        let sub_commands: Vec<String> = command
            .sub_commands
            .iter()
            .map(|x| x.name.clone())
            .collect();
        assert!(sub_commands.contains(&String::from("inc-a-b-c")));
        assert!(sub_commands.contains(&String::from("inc-a-b-c-d")));
        assert!(!sub_commands.contains(&String::from("inc-a-b-c-d-e")));
    }

    #[test]
    fn order_will_not_matter() {
        let input: Vec<SystemBinary> = ["a-b-c-d", "a-b", "a-b-c"]
            .iter()
            .map(|x| {
                SystemBinary {
                    path: PathBuf::new(),
                    name: format!("inc-{}", x),
                }
            })
            .collect();
        let input = HashSet::from_iter(input);

        let result = convert_command_set_to_map(input);
        assert!(result.contains_key("inc-a-b"), "contains the 'a-b' command");
        assert_eq!(result.len(), 1, "There should be only one entry");
        let command = result.get("inc-a-b").expect("a-b doesn't exist");
        assert_eq!(
            command.sub_commands.len(),
            2,
            "Should have two elements in it"
        );

        let sub_commands: Vec<String> = command
            .sub_commands
            .iter()
            .map(|x| x.name.clone())
            .collect();
        assert!(sub_commands.contains(&String::from("inc-a-b-c")));
        assert!(sub_commands.contains(&String::from("inc-a-b-c-d")));
        assert!(!sub_commands.contains(&String::from("inc-a-b-c-d-e")));
    }
}

fn find_commands() -> HashSet<SystemBinary> {
    let mut sub_commands: HashSet<SystemBinary> = HashSet::new();

    if let Ok(path) = env::var("PATH") {
        for split_path in path.split(":") {
            trace!(
                "Processing {} for {} executables",
                split_path,
                BASE_APPLICATION_NAME
            );
            for entry in fs::read_dir(split_path) {
                process_dir_read(&mut sub_commands, entry);
            }
        }
    }

    if let Ok(path) = current_exe() {
        let mut path = path.canonicalize().unwrap();
        path.pop();
        let path = path.as_path();
        debug!(
            "Processing {:?} for {:?} executables",
            BASE_APPLICATION_NAME,
            path
        );
        for entry in path.read_dir() {
            process_dir_read(&mut sub_commands, entry);
        }
    }

    return sub_commands;
}

fn process_dir_read(sub_commands: &mut HashSet<SystemBinary>, dir_read: ReadDir) {
    for entry in dir_read {
        match entry {
            Ok(ent) => process_dir_entry(sub_commands, ent),
            Err(_) => debug!("Unable to read dir"),
        }
    }
}

fn process_dir_entry(
    sub_commands: &mut HashSet<SystemBinary>,
    dir_entry: DirEntry,
) {
    let file_name = dir_entry.file_name().into_string();

    if file_name.is_err() {
        debug!("Unable to process entry");
        return;
    }

    if let Ok(file_type) = dir_entry.file_type() {
        if !file_type.is_file() {
            return;
        }
    }

    // let path = dir_entry.path();
    let file_name: String = file_name.unwrap();
    let prefix = format!("{}-", BASE_APPLICATION_NAME);

    if file_name.starts_with(prefix.as_str()) {
        if file_is_executable(&dir_entry) {
            sub_commands.insert(SystemBinary {
                path: dir_entry.path(),
                name: file_name.clone(),
            });
            debug!("Found command {}", file_name);
        }
    }
}

#[cfg(windows)]
fn file_is_executable(dir_entry: &DirEntry) -> bool {
    return true;
}

#[cfg(unix)]
fn file_is_executable(dir_entry: &DirEntry) -> bool {
    use std::os::unix::prelude::*;

    let permissions = dir_entry.metadata().unwrap().permissions();
    return permissions.mode() & 0o111 != 0;
}
