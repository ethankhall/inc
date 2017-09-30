use slog::Logger;
use scm::CheckoutError;
use std::collections::LinkedList;
use std::process::Command;
use std::path::Path;
use scm::ScmUrl;
use regex::RegexSet;

#[derive(Debug)]
pub struct GitScm  {
    logger: Logger,
    url: ScmUrl
}

pub fn get_git_checkout(logger: Logger, url: ScmUrl) -> Option<GitScm> {
    return Some(GitScm { logger, url });
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn will_accept_ssh_url() {
        assert!(is_git_url(ScmUrl::from("ssh://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz:111/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz:111/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz:111/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz:111/path/to/repo.git")));

        assert!(is_git_url(ScmUrl::from("ssh://host.xz/~ethall/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz/~ethall/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz/~ethall/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz/~ethall/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz:111/~ethall/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://host.xz:111/~ethall/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz:111/~ethall/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ssh://ethall@host.xz:111/~ethall/path/to/repo.git")));
    }

    #[test]
    fn will_accept_git_url() {
        assert!(is_git_url(ScmUrl::from("git://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("git://host.xz/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("git://host.xz:111/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("git://host.xz:111/path/to/repo.git")));
    }

    #[test]
    fn will_accept_http_url() {
        assert!(is_git_url(ScmUrl::from("http://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("https://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("http://host.xz:222/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("https://host.xz:222/path/to/repo.git/")));
    }

    #[test]
    fn will_accept_ftp_url() {
        assert!(is_git_url(ScmUrl::from("ftp://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ftps://host.xz/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ftp://host.xz:222/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("ftps://host.xz:222/path/to/repo.git/")));
    }

    #[test]
    fn will_accept_file_path() {
        assert!(is_git_url(ScmUrl::from("/path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("/path/to/repo.git/")));
        assert!(is_git_url(ScmUrl::from("file:///path/to/repo.git")));
        assert!(is_git_url(ScmUrl::from("file:///path/to/repo.git/")));
    }
}

fn is_git_url(url: ScmUrl) -> bool {
    let regex_set = RegexSet::new(&[
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
    ]).unwrap();
    return regex_set.matches(url.as_str()).matched_any();
}

impl GitScm {
    pub fn do_checkout(&self, destination: &Path) -> Result<i32, CheckoutError>  {

        let mut args: LinkedList<String> = LinkedList::new();
        args.push_back(String::from("clone"));
        args.push_back(self.url.as_str().to_string());
        args.push_back((*destination).to_str().unwrap().to_string());

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