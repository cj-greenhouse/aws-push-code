use std::path::PathBuf;

pub trait FileSystem {
    type FileSystemError;
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::FileSystemError> {unimplemented!();}
}

