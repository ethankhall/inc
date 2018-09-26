#[cfg(test)]
pub mod test_data {

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
        "ssh://ethall@host.xz:111/~ethall/path/to/repo.git",
    ];

    pub const GIT_URL_ARRAY: &'static [&'static str] = &[
        "git://host.xz/path/to/repo.git/",
        "git://host.xz/path/to/repo.git",
        "git://host.xz:111/path/to/repo.git/",
        "git://host.xz:111/path/to/repo.git",
        "git://host.xz:to/repo.git",
    ];

    pub const HTTP_URL_ARRAY: &'static [&'static str] = &[
        "http://host.xz/path/to/repo.git/",
        "http://host.xz/path/to/repo.git",
        "http://host.xz:222/path/to/repo.git/",
        "http://host.xz:222/path/to/repo.git",
        "https://host.xz/path/to/repo.git/",
        "https://host.xz/path/to/repo.git",
        "https://host.xz:222/path/to/repo.git/",
        "https://host.xz:222/path/to/repo.git",
    ];

    pub const FTP_URL_ARRAY: &'static [&'static str] = &[
        "ftp://host.xz/path/to/repo.git/",
        "ftps://host.xz/path/to/repo.git/",
        "ftp://host.xz:222/path/to/repo.git/",
        "ftps://host.xz:222/path/to/repo.git/",
    ];

    pub const FILE_URL_ARRAY: &'static [&'static str] = &[
        "/path/to/repo.git",
        "/path/to/repo.git/",
        "file:///path/to/repo.git",
        "file:///path/to/repo.git/",
    ];

    pub const CUSTOM_URLS_ARRAY: &'static [&'static str] = &[
        "git@github.com:ethankhall/etrain.git",
        "https://github.com/ethankhall/etrain.git",
    ];
}

#[cfg(test)]
pub mod test {
    use super::test_data::*;
    use libs::scm::provider::git::*;
    use libs::scm::{ScmProvier, ScmUrl};

    macro_rules! is_git_url {
        ($($name:ident: $arguments:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let git = GitScm { };
                for arg in $arguments.iter() {
                    assert!(git.handles_url(&ScmUrl::from(*arg)));
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

    macro_rules! checkout_destination_from_url {
        ($($name:ident: $arguments:expr,)*) => {
        $(
            #[test]
            fn $name() {
                for arg in $arguments.iter() {
                    let expected = String::from("repo");
                    let url = ScmUrl::from(*arg);

                    let git = GitScm { };
                    let name = git.sugested_checkout_name(&url);
                    assert!(name.is_some(), "url didn't get parsed: {}", url.clone());
                    assert_eq!(name.unwrap(), expected, "parse url: {}", url.clone());
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
