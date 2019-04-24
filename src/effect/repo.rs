
use std::path::Path;


pub trait Git {
    type GitError;
    fn clone_repo(&self, _from: &str, _to: &Path) -> Result<(), Self::GitError> {unimplemented!();}
}

