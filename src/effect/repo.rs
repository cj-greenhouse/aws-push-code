use std::path::Path;

pub trait GitError {}

pub trait Git {
    type Error;
    fn clone_repo(&self, _from: &str, _to: &Path) -> Result<(), Self::Error> {unimplemented!();}
}



