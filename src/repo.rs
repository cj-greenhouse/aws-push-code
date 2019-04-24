
use std::path::Path;


pub trait Git {
    type GitError;
    fn clone(from: &str, to: &Path) -> Result<(), Self::GitError> {unimplemented!();}
}

