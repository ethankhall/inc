use slog::Logger;
use std::collections::LinkedList;
use std::process::Command;
use std::path::Path;
use libs::scm::{ScmUrl, CheckoutError, ScmProvier};
use regex::RegexSet;
use url::Url;

#[derive(Debug, Clone)]
pub struct GitScm<'a> {
    pub logger: &'a Logger,
}

pub(crate) const GIT_URL_REGEX: &'static [&'static str] =
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
    ];

impl<'a> ScmProvier for GitScm<'a> {
    fn sugested_checkout_name(&self, url: &ScmUrl) -> Option<String> {
        return if self.handles_url(url) {
            compute_destination(url.clone())
        } else {
            None
        };
    }

    fn do_checkout(&self, url: &ScmUrl, destination: &Path) -> Result<i32, CheckoutError> {
        let mut args: LinkedList<String> = LinkedList::new();
        args.push_back(String::from("clone"));
        args.push_back(url.clone());
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

    fn handles_url(&self, url: &ScmUrl) -> bool {
        let regex_set = RegexSet::new(GIT_URL_REGEX).unwrap();
        let matches = regex_set.matches(url.as_str()).matched_any();
        slog_debug!(self.logger, "GIT: `{}` is a git url? => {}", url, matches);
        return matches;
    }
}

fn compute_destination(url: ScmUrl) -> Option<String> {
    let sanitized_url = if url.ends_with("/") || url.ends_with("\\") {
        String::from(&(url[..(url.len() - 1)]))
    } else {
        url
    };

    let parsed_url = Url::parse(sanitized_url.as_str().clone());
    if let Ok(unwrapped_url) = parsed_url {
        return Some(extract_directory(unwrapped_url.path()));
    }

    let index = sanitized_url.as_str().rfind("/");
    if let Some(index) = index {
        return Some(extract_directory(&(sanitized_url[(index)..])));
    }

    let index = sanitized_url.as_str().rfind("\\");
    if let Some(index) = index {
        return Some(extract_directory(&(sanitized_url[(index)..])));
    }

    return None;
}

fn extract_directory(last_path_chunk: &str) -> String {
    return String::from(
        Path::new(last_path_chunk)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
}
