use crate::repo::{CodeRepository};

type Runtime = String;

pub fn wire() -> Runtime {
    unimplemented!();
}


impl CodeRepository for Runtime where {
    type Error = String;
    type Handle = String;
    fn pull_repository(_url: &str) -> Result<Self::Handle, Self::Error> {
        unimplemented!();
    }
}
