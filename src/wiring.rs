use crate::repo::Git;
use crate::flow::{MkTemp};

pub type Runtime = String;
pub type RuntimeError = String;

pub fn wire() -> Runtime {
    unimplemented!();
}


impl Git for Runtime {
    type GitError = String;
}

impl MkTemp for Runtime {
    type MkTempError = String;
}



// impl CodeRepository for Runtime where {
//     type Error = String;
//     type Handle = String;
//     fn pull_repository(_url: &str) -> Result<Self::Handle, Self::Error> {
//         unimplemented!();
//     }
// }
