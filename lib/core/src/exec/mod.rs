use std::vec::Vec;

pub trait Execution<T> {
    fn execute(&self, args: &Vec<String>) -> Result<T, String>;
}

pub mod system;
pub mod executor;