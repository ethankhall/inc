use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemBinary {
    pub path: PathBuf,
    pub name: String,
}
