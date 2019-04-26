use std::path::PathBuf;

pub trait FileSystemError {}

pub trait FileSystem {
    type Error;
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
}

