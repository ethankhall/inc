#![feature(conservative_impl_trait)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate url;
extern crate names;
extern crate docopt;
extern crate slog_term;
extern crate slog_async;
extern crate yaml_rust;

pub mod commands;
pub mod core;
pub mod libs;