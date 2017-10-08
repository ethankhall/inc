#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
extern crate regex;
extern crate url;
extern crate names;
extern crate docopt;
extern crate yaml_rust;

pub mod core;
pub mod libs;
pub mod exec;