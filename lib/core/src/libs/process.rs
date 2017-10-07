use std::vec::Vec;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemCommand {
    pub binary: SystemBinary,
    pub alias: String,
    pub sub_commands: Vec<SystemBinary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemBinary {
    pub path: PathBuf,
    pub name: String,
}
