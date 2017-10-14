extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn calling_inc_without_args() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr().contains("Invalid arguments.

Usage:
    inc [options] <command> [--] [<args>...]
    inc --list
    inc --version
    inc --help
")
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
}