extern crate assert_cli;
extern crate tempdir;

#[cfg(test)]
#[cfg(unix)]
mod root_integration {
    use assert_cli;

    #[test]
    fn calling_inc_without_args() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr()
            .contains(
                "USAGE:
    inc [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -q, --quite      Only error output will be displayed
    -V, --version    Prints version information
    -v, --verbose    Increasing verbosity
    -w, --warn       Only display warning messages

SUBCOMMANDS:
    checkout    Checkout from SCM
    exec        Execute commands from the project.
    help        Prints this message or the help of the given subcommand(s)
    list        List the known element for Inc.",
            ).unwrap();
    }

    #[test]
    fn calling_inc_with_list() {
        assert_cli::Assert::main_binary()
            .with_args(&["list"])
            .succeeds()
            .and()
            .stdout()
            .contains(
                "avaliable-commands:
  - checkout
  - exec
  - list",
            ).unwrap();
    }

    #[test]
    fn calling_inc_with_list_and_verbose() {
        assert_cli::Assert::main_binary()
            .with_args(&["-vvv", "list"])
            .succeeds()
            .and()
            .stdout()
            .contains(
                "[INFO] avaliable-commands:
  - checkout
  - exec
  - list",
            ).unwrap();
    }
}
