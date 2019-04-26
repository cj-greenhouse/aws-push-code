use crate::effect::repo::Git;
use crate::effect::file::FileSystem;

pub type Runtime = ();

pub fn wire() -> Runtime {
    ()
}


impl Git for Runtime {
    type Error = ();
}

impl FileSystem for Runtime {
    type Error = ();
}

