extern crate assert_cli;
extern crate tempdir;

mod shared;

#[cfg(test)]
mod checkout_integration {
    use shared::utils::*;
    use std::env::var;

    #[test]
    fn checkout_github_repo() {
        with_test_dir(|tmp_dir| {
            create_assert()
                .with_args(&["checkout", "--https-only", "github/choosealicense.com"])
                .current_dir(tmp_dir.clone())
                .succeeds()
                .unwrap();

            let inc_dir = tmp_dir.join("choosealicense");
            assert!(inc_dir.exists());

            let inc_file = inc_dir.join("README.md");
            assert!(inc_file.exists());
        });
    }

    #[test]
    fn checkout_no_args() {
        create_assert()
            .with_args(&["checkout"])
            .fails()
            .and()
            .stderr()
            .contains(
                "error: The following required arguments were not provided:
    <repository>

USAGE:
    inc checkout [FLAGS] [OPTIONS] <repository> [directory]

For more information try --help",
            ).unwrap();
    }

    #[test]
    fn checkout_help() {
        create_assert()
            .with_args(&["checkout", "--help"])
            .succeeds()
            .and()
            .stdout()
            .contains(
                "Checkout from SCM

USAGE:
    inc checkout [FLAGS] [OPTIONS] <repository> [directory]

FLAGS:
    -h, --help             Prints help information
        --https-only       Only do checkouts using http instead of ssh.
        --list-services    List all of the avaliable services.
    -q, --quite            Only error output will be displayed
    -v, --verbose          Increasing verbosity
    -w, --warn             Only display warning messages

OPTIONS:
    -s, --service <service>    Where to checkout from. A lot of cases will be github.

ARGS:
    <repository>    The (possibly remote) repository to clone from.
    <directory>     Clones a repository into a newly created directory.",
            ).unwrap();
    }

    #[test]
    fn checkout_list_internal() {
        create_assert()
            .with_args(&["checkout", "--list-services"])
            .succeeds()
            .and()
            .stdout()
            .contains("Services:\n - bitbucket\n - github\t[default]")
            .unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn checkout_list_with_external() {
        use std::fs;
        use std::io::prelude::*;
        use std::os::unix::fs::OpenOptionsExt;
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc-checkout-service-foobar");
            let file_path = file_path.to_str().unwrap();
            let mut tmp_file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o770)
                .open(file_path)
                .unwrap();
            writeln!(tmp_file, "echo \"github.com/github/choosealicense.com\"")
                .expect("write temp file");

            let new_path = format!("{}:{}", var("PATH").unwrap(), tmp_dir.to_str().unwrap());

            create_assert()
                .with_args(&["-vvv", "checkout", "--list-services"])
                .with_env(&[("PATH", new_path)])
                .succeeds()
                .and()
                .stdout()
                .contains("Services:\n - bitbucket\n - foobar\n - github\t[default]")
                .unwrap();
        });
    }

    #[test]
    fn checkout_from_service_with_param() {
        with_test_dir(|tmp_dir| {
            let checkout_dir = tmp_dir.clone().join("inc-checkout");

            let file_path = tmp_dir.clone().join("inc-checkout-service-foobar");
            copy_resource("inc-checkout-service-foobar", file_path);

            let new_path = format!("{}:{}", var("PATH").unwrap(), tmp_dir.to_str().unwrap());

            create_assert()
                .with_args(&[
                    "-vvv",
                    "checkout",
                    "--service=foobar",
                    "something-random",
                    checkout_dir.to_str().unwrap(),
                ]).with_env(&[("PATH", new_path)])
                .succeeds()
                .unwrap();

            assert!(checkout_dir.exists());
        });
    }

    #[test]
    fn checkout_from_service() {
        with_test_dir(|tmp_dir| {
            let file_path = tmp_dir.clone().join("inc-checkout-service-foobar");
            copy_resource("inc-checkout-service-foobar", file_path);

            let new_path = format!("{}:{}", var("PATH").unwrap(), tmp_dir.to_str().unwrap());

            create_assert()
                .with_args(&["checkout", "--service=foobar", "something-random"])
                .with_env(&[("PATH", new_path)])
                .succeeds()
                .current_dir(tmp_dir.clone())
                .unwrap();

            assert!(tmp_dir.clone().join("choosealicense").exists());
        });
    }
}
