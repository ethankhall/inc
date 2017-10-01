use slog::Logger;
use std::path::{Path,PathBuf};
use git::get_git_checkout;
use url::Url;
use std::env::current_dir;
use names::Generator;

#[derive(Debug)]
pub struct CheckoutError {
    pub error: String,
}

pub type ScmUrl = String;

pub fn create_url(logger: Logger, service: &str, repo: &str) -> Result<String, CheckoutError> {
    slog_debug!(logger, "Origonal input: {}", service);
    let service = service.to_lowercase();
    return if service == "github" {
        Ok(format!("git@github.com:{0}.git", repo))
    } else {
        Err(CheckoutError { error: String::from("Unknown service!") })
    };
}

pub fn do_scm_checkout(
    logger: Logger,
    url: String,
    destination: Option<&str>) -> Result<i32, CheckoutError> {
    slog_trace!(logger, "URL to clone: {}", url);

    let checkout_dir = compute_destination(logger.clone(), url.clone(), destination);
    slog_trace!(logger, "Checkout dir: {:?}", checkout_dir);

    if let Some(git_checkout) = get_git_checkout(logger, url) {
        return git_checkout.do_checkout(checkout_dir.as_path());
    }
    return Ok(1);
}

fn compute_destination(logger: Logger, url: ScmUrl, destination: Option<&str>) -> PathBuf {
    if destination.is_some() {
        let destination = destination.unwrap();
        return PathBuf::from(destination);
    }

    let sanitized_url = if url.ends_with("/") || url.ends_with("\\") {
        String::from(&(url[..(url.len() - 1)]))
    } else {
        url
    };

    let parsed_url = Url::parse(sanitized_url.as_str().clone());
    if let Ok(unwrapped_url) = parsed_url {
        return extract_directory(unwrapped_url.path());
    }

    let index = sanitized_url.as_str().rfind("/");
    if let Some(index) = index {
        return extract_directory(&(sanitized_url[(index)..]));
    }

    let index = sanitized_url.as_str().rfind("\\");
    if let Some(index) = index {
        return extract_directory(&(sanitized_url[(index)..]));
    }
    
    let project_name = Generator::default().next().unwrap();
    slog_info!(logger, "Unable to determine a project name, using {}.", project_name);

    return extract_directory(project_name.as_str());
}

fn extract_directory(last_path_chunk: &str) -> PathBuf {
    let working_dir = current_dir();
    if let Ok(_) = working_dir {
        let mut path = working_dir.unwrap();
        path.push(Path::new(last_path_chunk).file_stem().unwrap());
        return path;
    }

    return PathBuf::from(last_path_chunk);
}

#[cfg(test)]
mod test {
    use super::*;
    use slog::{Discard, Logger};
    use git::test::*;

    macro_rules! checkout_destination_from_url {
        ($($name:ident: $arguments:expr,)*) => {
        $(
            #[test]
            fn $name() {
                for arg in $arguments.iter() {
                    let expected = PathBuf::from("repo");
                    let root = Logger::root(Discard, o!());
                    let url = ScmUrl::from(*arg);
                    // println!("test string: {}", String::from(*arg));
                    let dest = compute_destination(root, url.clone(), None);
                    let file_name = dest.file_name();
                    assert!(file_name.is_some(), "url didn't get parsed: {}", url.clone());
                    assert_eq!(file_name.unwrap(), expected, "parse url: {}", url.clone());
                }
            }
        )*
        }
    }

    checkout_destination_from_url! {
        git_ssh_urls: SSH_URL_ARRAY,
        git_urls: GIT_URL_ARRAY,
        git_http_urls: HTTP_URL_ARRAY,
        git_ftp_urls: FTP_URL_ARRAY,
        git_file_urls: FILE_URL_ARRAY,
    }
}