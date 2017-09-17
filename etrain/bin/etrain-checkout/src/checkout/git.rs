use slog::Logger;
use scm::CheckoutError;
use std::collections::LinkedList;
use std::process::Command;
use url::Url;

#[derive(Debug)]
pub struct GitCheckout {
    logger: Logger,
    url: String,
    destination: Option<String>,
}

impl GitCheckout {
    pub fn new(logger: Logger, url: String, destination: Option<&str>) -> GitCheckout {
        let destination = destination.map(|n| String::from(n));
        GitCheckout { logger, url, destination }
    }

    pub fn do_checkout(&self) -> Result<i32, CheckoutError> {

        let mut args: LinkedList<String> = LinkedList::new();
        args.push_back(String::from("clone"));
        args.push_back(self.url.as_str().to_string());

        if let Some(dest) = self.destination.clone() {
            args.push_back(dest);
        }

        slog_trace!(self.logger, "About to execute {:?}", args);
        let mut git_command = Command::new("git")
            .args(args)
            .spawn()
            .expect("failed to execute process");

        let exit_status = git_command.wait().expect("failed to wait on child");

        return match exit_status.code() {
            Some(code) => Ok(code),
            None => Err( CheckoutError { error: String::from("Unknown Error") })
        };
    }
}
