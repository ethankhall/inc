extern crate assert_cli;
extern crate tempdir;

mod shared;

#[cfg(test)]
#[cfg(unix)]
mod exec_integration {
    use shared::utils::*;

    #[test]
    fn with_no_args() {
        create_assert()
            .with_args(&["exec"])
            .fails()
            .and()
            .stderr()
            .contains(
                "
USAGE:
    inc exec [FLAGS] <command>

FLAGS:
    -h, --help             Prints help information
        --list-commands    List all of the avaliable commands.
    -q, --quite            Only error output will be displayed
    -v, --verbose          Increasing verbosity
    -w, --warn             Only display warning messages

ARGS:
    <command>    Name of the command to execute.",
            ).unwrap();
    }

    #[test]
    fn list_with_build_and_run() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.yaml");
            copy_resource("sample1.yaml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "--list-commands"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains(
                    "Avaliable Commands:
 - name: build
   description: Build the project
   commands:
     - command: echo \"Hello World\"
       env: {}
 - name: run
   description: No Description Provided
   commands:
     - command: echo \"Goodbye World!\"
       env: {}",
                ).unwrap();
        });
    }

    #[test]
    fn exec_build() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.yaml");
            copy_resource("sample1.yaml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "build"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains("Hello World")
                .unwrap();

            create_assert()
                .with_args(&["-vvv", "exec", "run"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains("Goodbye World!")
                .unwrap();
        });
    }

    #[test]
    fn list_with_multiple_commands() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.yaml");
            copy_resource("sample2.yaml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "--list-commands"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains(
                    "Avaliable Commands:
 - name: build
   description: Build the project
   commands:
     - command: echo \"Hello World\"
       env: {}
     - command: echo \"Goodbye World!\"
       env: {}
",
                ).unwrap();

            create_assert()
                .with_args(&["exec", "build"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains(
                    "** Executing `echo \"Hello World\"`
Hello World
** Executing `echo \"Goodbye World!\"`
Goodbye World!
",
                ).unwrap();
        });
    }

    #[test]
    fn when_command_failes_it_will_stop() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc.yaml");
            copy_resource("sample3.yaml", file_path);

            create_assert()
                .with_args(&["-vvv", "exec", "--list-commands"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .and()
                .stderr()
                .is("")
                .stdout()
                .contains(
                    "Avaliable Commands:
 - name: build
   description: This should fail, due to the false.
   commands:
     - command: echo \"Hello World\"
       env: {}
     - command: false
       env: {}
     - command: echo \"Goodbye World!\"
       env: {}",
                ).unwrap();

            create_assert()
                .with_args(&["exec", "build"])
                .current_dir(tmp_dir.clone())
                .fails()
                .and()
                .stderr()
                .is("Command: `false` returned 1")
                .stdout()
                .contains("Hello World\n")
                .unwrap();
        });
    }
}
