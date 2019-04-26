use std::path::Path;

pub trait GitError {}

pub trait Git<E>
where E: GitError {
    fn clone_repo(&self, _from: &str, _to: &Path) -> Result<(), E> {unimplemented!();}
}



