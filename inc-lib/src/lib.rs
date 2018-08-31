extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;
extern crate names;
extern crate regex;
extern crate url;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate dirs;

#[macro_export]
macro_rules! s {
    ($x:expr) => ( $x.to_string() );
}

pub mod core;
pub mod libs;
pub mod exec;
