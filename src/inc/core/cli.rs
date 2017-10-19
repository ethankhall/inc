use std::collections::{HashSet, HashMap};
use std::env::{self, current_exe};
use std::fs::{self, DirEntry, ReadDir};
use core::BASE_APPLICATION_NAME;
use libs::process::SystemBinary;

pub fn find_commands_avalible() -> HashMap<String, SystemBinary> {
    let mut command_map : HashMap<String, SystemBinary> = HashMap::new();
    for command in find_commands().iter() {
        command_map.insert(String::from(command.path.file_stem().unwrap().to_str().unwrap()), command.clone());
    }
    return command_map;
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
    use std::os::unix::fs::PermissionsExt;

    let permissions = dir_entry.metadata().unwrap().permissions();
    return permissions.mode() & 0o111 != 0;
}
