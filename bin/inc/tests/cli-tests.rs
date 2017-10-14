extern crate assert_cli;
extern crate tempdir;

#[cfg(test)]
mod integration {
    use assert_cli;
    use tempdir::TempDir;
    use std::path::PathBuf;

    #[test]
    fn calling_inc_without_args() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr().contains("No options provided. See `inc --help` for more options.")
            .unwrap();
    }

    #[test]
    fn calling_inc_with_list() {
        assert_cli::Assert::main_binary()
            .with_args(&["--list"])
            .succeeds()
            .and()
            .stdout().contains("I don't know how to do that.... Yet!")
            .unwrap();
    }

    #[test]
    fn calling_inc_with_list_and_verbose() {
        assert_cli::Assert::main_binary()
            .with_args(&["-vvv", "--list"])
            .succeeds()
            .and()
            .stdout().contains("I don't know how to do that.... Yet!")
            .unwrap();
    }

    #[test]
    fn calling_inc_with_help() {
        assert_cli::Assert::main_binary()
            .with_args(&["help"])
            .fails()
            .and()
            .stdout().contains("Help! Help! I need an adult!")
            .unwrap();
    }

    #[test]
    fn calling_inc_with_checkout() {
        with_test_dir(|tmp_dir| {
            assert_cli::Assert::command(&[build_exec().as_str(), "checkout", "ethankhall/inc"])
            .current_dir(tmp_dir.clone())
            .succeeds()
            .unwrap();

            let inc_dir = tmp_dir.join("inc");
            assert!(inc_dir.exists());

            let inc_file = inc_dir.join("inc.toml");
            assert!(inc_file.exists());
        });
    }

    fn with_test_dir<F: Fn(PathBuf) -> ()>(exec: F) {
        let tmp_dir = TempDir::new("checkout-dir-tmp").expect("temp dir should be created");

        exec(tmp_dir.path().to_owned());

        tmp_dir.close().expect("temp dir should be closed");
    }

    fn build_exec() -> String {
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
}