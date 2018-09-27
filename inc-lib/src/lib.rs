extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;
extern crate names;
extern crate regex;
extern crate serde;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
#[cfg(unix)]
extern crate libc;
extern crate serde_yaml;
extern crate signal_hook;

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

pub mod core;
pub mod exec;
pub mod libs;
