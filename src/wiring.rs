use crate::repo::Git;
use crate::file::{FileSystem};

pub type Runtime = String;
pub type RuntimeError = String;

pub fn wire() -> Runtime {
    "".to_owned()
}


impl Git for Runtime {
    type GitError = String;
}

impl FileSystem for Runtime {
    type FileSystemError = String;
}
