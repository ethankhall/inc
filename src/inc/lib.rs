#![feature(conservative_impl_trait)]

extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;
extern crate names;
extern crate regex;
extern crate toml;
extern crate url;

#[macro_export]
macro_rules! s {
    ($x:expr) => ( $x.to_string() );
}

pub mod core;
pub mod libs;
pub mod exec;
