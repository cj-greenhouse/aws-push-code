use std::path::PathBuf;
use tempfile::{self, NamedTempFile};

pub trait FileSystemError {}

pub trait FileSystem {
    type Error;
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
    fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
}

pub fn mk_temp_dir() -> Result<PathBuf,std::io::Error> {
    Ok(
        tempfile::tempdir()?.into_path()
    )
}

pub fn mk_temp_file() -> Result<PathBuf,std::io::Error> {
    let file = NamedTempFile::new()?;
    let file = file.path().to_owned();
    Ok(file)
}
