use slog::Logger;
use std::collections::LinkedList;
use std::process::Command;
use std::path::Path;
use scm::core::{ScmUrl, CheckoutError};
use regex::RegexSet;

#[derive(Debug)]
pub struct GitScm {
    logger: Logger,
    url: ScmUrl,
}

pub fn get_git_checkout(logger: Logger, url: ScmUrl) -> Option<GitScm> {
    return if is_git_url(url.clone()) {
        Some(GitScm { logger, url })
    } else {
        None
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub const SSH_URL_ARRAY: &'static [&'static str] = &[
        "ssh://host.xz/path/to/repo.git/",
        "ssh://host.xz/path/to/repo.git",
        "ssh://ethall@host.xz/path/to/repo.git/",
        "ssh://ethall@host.xz/path/to/repo.git",
        "ssh://host.xz:111/path/to/repo.git/",
        "ssh://host.xz:111/path/to/repo.git",
        "ssh://ethall@host.xz:111/path/to/repo.git/",
        "ssh://ethall@host.xz:111/path/to/repo.git",
        "ssh://host.xz/~ethall/path/to/repo.git/",
        "ssh://host.xz/~ethall/path/to/repo.git",
        "ssh://ethall@host.xz/~ethall/path/to/repo.git/",
        "ssh://ethall@host.xz/~ethall/path/to/repo.git",
        "ssh://ethall@host.xz:111/~ethall/path/to/repo.git/",
        "ssh://ethall@host.xz:111/~ethall/path/to/repo.git"];

    pub const GIT_URL_ARRAY: &'static [&'static str] = &[
        "git://host.xz/path/to/repo.git/",
        "git://host.xz/path/to/repo.git",
        "git://host.xz:111/path/to/repo.git/",
        "git://host.xz:111/path/to/repo.git",
        "git://host.xz:to/repo.git",];

    pub const HTTP_URL_ARRAY: &'static [&'static str] = &[
        "http://host.xz/path/to/repo.git/",
        "http://host.xz/path/to/repo.git",
        "http://host.xz:222/path/to/repo.git/",
        "http://host.xz:222/path/to/repo.git",
        "https://host.xz/path/to/repo.git/",
        "https://host.xz/path/to/repo.git",
        "https://host.xz:222/path/to/repo.git/",
        "https://host.xz:222/path/to/repo.git",];

    pub const FTP_URL_ARRAY: &'static [&'static str] = &[
        "ftp://host.xz/path/to/repo.git/",
        "ftps://host.xz/path/to/repo.git/",
        "ftp://host.xz:222/path/to/repo.git/",
        "ftps://host.xz:222/path/to/repo.git/",];

    pub const FILE_URL_ARRAY: &'static [&'static str] = &[
        "/path/to/repo.git",
        "/path/to/repo.git/",
        "file:///path/to/repo.git",
        "file:///path/to/repo.git/",];

    pub const CUSTOM_URLS_ARRAY: &'static [&'static str] = &[
        "git@github.com:ethankhall/etrain.git",
        "https://github.com/ethankhall/etrain.git"];

    macro_rules! is_git_url {
        ($($name:ident: $arguments:expr,)*) => {
        $(
            #[test]
            fn $name() {
                for arg in $arguments.iter() {
                    assert!(is_git_url(ScmUrl::from(*arg)));
                }
            }
        )*
        }
    }

    is_git_url! {
        will_accept_ssh_url: SSH_URL_ARRAY,
        will_accept_git_url: GIT_URL_ARRAY,
        will_accept_http_url: HTTP_URL_ARRAY,
        will_accept_ftp_url: FTP_URL_ARRAY,
        will_accept_file_path: FILE_URL_ARRAY,
        will_accept_custom_urls: CUSTOM_URLS_ARRAY,
    }
}

fn is_git_url(url: ScmUrl) -> bool {
    let regex_set = RegexSet::new(
        &[
            // => ssh://[user@]host.xz[:port]/path/to/repo.git/
            r"ssh://((.*@)?)([a-zA-Z0-9\\-\\.]+)((:[0-9]+)?)/(.+?)(\.git(/)?)",
            // => git://host.xz[:port]/path/to/repo.git/
            r"git://([a-zA-Z0-9\\-\\.]+)((:[0-9]+)?)/(.+?)(\.git(/)?)",
            // => http[s]://host.xz[:port]/path/to/repo.git/
            r"http(s?)://([a-zA-Z0-9\\-\\.]+)((:[0-9]+)?)/(.+?)(\.git(/)?)",
            // => ftp[s]://host.xz[:port]/path/to/repo.git/
            r"ftp(s?)://([a-zA-Z0-9\\-\\.]+)((:[0-9]+)?)/(.+?)(\.git(/)?)",
            // => /path/to/repo.git/
            r"/(.+?)(\.git(/)?)",
            // => file:///path/to/repo.git/
            r"file:///(.+?)(\.git(/)?)",
        ],
    ).unwrap();
    return regex_set.matches(url.as_str()).matched_any();
}

impl GitScm {
    pub fn do_checkout(&self, destination: &Path) -> Result<i32, CheckoutError> {

        let mut args: LinkedList<String> = LinkedList::new();
        args.push_back(String::from("clone"));
        args.push_back(self.url.as_str().to_string());
        args.push_back((*destination).to_str().unwrap().to_string());

        slog_trace!(self.logger, "About to execute {:?}", args);
        let mut git_command = Command::new("git").args(args).spawn().expect(
            "failed to execute process",
        );

        let exit_status = git_command.wait().expect("failed to wait on child");

        return match exit_status.code() {
            Some(code) => Ok(code),
            None => Err(CheckoutError { error: String::from("Unknown Error") }),
        };
    }
}
