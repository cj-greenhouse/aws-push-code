use crate::effect::repo::Git;
use crate::effect::file::FileSystem;
use crate::submit::Pipeline;

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

impl Pipeline<RuntimeError> for Runtime {}
