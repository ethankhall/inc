use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemBinary {
    pub path: PathBuf,
    pub name: String,
}

impl SystemBinary {
    pub fn copy(&self) -> Self {
        return SystemBinary {
            path: self.path.to_path_buf(),
            name: self.name.clone(),
        };
    }
}
