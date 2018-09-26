use names::Generator;
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub(crate) fn compute_destination(
    destination: Option<String>,
    suggested_name: Option<String>,
) -> PathBuf {
    if destination.is_some() {
        let destination = destination.unwrap();
        return PathBuf::from(destination);
    }

    if let Some(suggestion) = suggested_name {
        return extract_directory(suggestion);
    }

    let project_name = Generator::default().next().unwrap();
    info!(
        "Unable to determine a project name, using {}.",
        project_name
    );

    return extract_directory(project_name);
}

fn extract_directory(last_path_chunk: String) -> PathBuf {
    let working_dir = current_dir();
    if let Ok(_) = working_dir {
        let mut path = working_dir.unwrap();
        path.push(Path::new(last_path_chunk.as_str()).file_stem().unwrap());
        return path;
    }

    return PathBuf::from(last_path_chunk);
}
