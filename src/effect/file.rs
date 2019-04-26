use std::path::PathBuf;

pub trait FileSystemError {}

pub trait FileSystem<E>
where E: FileSystemError {
    fn mk_temp_dir(&self) -> Result<PathBuf, E> {unimplemented!();}
}

