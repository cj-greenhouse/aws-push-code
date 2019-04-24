
use std::path::Path;


pub trait Git {
    type GitError;
    fn clone_repo(&self, from: &str, to: &Path) -> Result<(), Self::GitError> {unimplemented!();}
}

