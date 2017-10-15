extern crate assert_cli;
extern crate tempdir;

mod shared;

#[cfg(test)]
mod exec_integration {
    use shared::utils::*;

    #[test]
    fn with_no_args() {
        create_assert()
            .with_args(&["exec"])
            .fails()
            .and()
            .stderr().contains("Option or command must be passed! Run inc exec --help for options.")
            .unwrap();
    }

    #[test]
    fn list_with_build_and_run() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.toml");
            copy_resource("sample1.toml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "--list"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr().is("")
                .stdout().contains("Avaliable Commands:
 - name: build
   description: Build the project
   commands:
     - echo \"Hello World\"
 - name: run
   description: No Description Provided
   commands:
     - echo \"Hello World\"")
                .unwrap();
        });
    }

    #[test]
    fn exec_build() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.toml");
            copy_resource("sample1.toml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "build"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr().is("")
                .stdout().contains("Hello World")
                .unwrap();
        });
    }
}