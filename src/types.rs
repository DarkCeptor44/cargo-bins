use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::DirEntry};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Binary {
    pub(crate) name: String,
    pub(crate) path: String,
}

impl From<DirEntry> for Binary {
    fn from(value: DirEntry) -> Self {
        let name = value.file_name().to_string_lossy().to_string();
        let path = value.path().display().to_string();

        Binary { name, path }
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
