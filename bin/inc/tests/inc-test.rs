extern crate assert_cli;
extern crate tempdir;

#[cfg(test)]
mod root_integration {
    use assert_cli;

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
}