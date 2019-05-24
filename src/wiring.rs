use crate::effect::repo::Git;
use crate::effect::file::FileSystem;
use crate::submit::{SubmitE};

pub struct Runtime;

pub fn wire() -> Runtime {
    Runtime
}


impl Git for Runtime {
    type Error = ();
}

impl FileSystem for Runtime {
    type Error = ();
}

impl SubmitE for Runtime { type Error = ();}

