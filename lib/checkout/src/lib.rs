#![feature(conservative_impl_trait)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate url;
extern crate names;
extern crate etrain_core;
extern crate docopt;

const DEFAULT_CHECKOUT_SOURCE: &'static str = "github";
const PRE_DEFINED_CHECKOUT_SOURCES: &'static [&'static str] = &["github"];

pub mod exe;
mod scm;