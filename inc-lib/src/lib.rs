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
extern crate serde_yaml;

#[macro_export]
macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

pub mod core;
pub mod exec;
pub mod libs;
