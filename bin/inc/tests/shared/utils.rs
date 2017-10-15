use std::path::PathBuf;
use tempdir::TempDir;
use assert_cli;
use std::fs;

pub fn with_test_dir<F: Fn(PathBuf) -> ()>(exec: F) {
    let tmp_dir = TempDir::new("checkout-dir-tmp").expect("temp dir should be created");

    exec(tmp_dir.path().to_owned());

    tmp_dir.close().expect("temp dir should be closed");
}

pub fn build_exec() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("target");
    path.push("debug");
    path.push("inc");

    let path = String::from(path.to_str().unwrap());

    println!("path: {}", path);
    path
}

pub fn create_assert() -> assert_cli::Assert {
    assert_cli::Assert::command(&[build_exec().as_str()])
}

pub fn copy_resource<T: Into<String>>(source_name: T, dest: PathBuf) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("resources");
    path.push(source_name.into());

    fs::copy(path, &dest).expect("copying file from test dir");
}