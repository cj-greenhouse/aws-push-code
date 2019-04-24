use crate::effect::repo::Git;
use crate::effect::file::FileSystem;
use crate::submit::Pipeline;

pub type Runtime = ();

pub fn wire() -> Runtime {
    ()
}


impl Git for Runtime {
    type GitError = String;
}

impl FileSystem for Runtime {
    type FileSystemError = String;
}

impl Pipeline<String> for Runtime {}
