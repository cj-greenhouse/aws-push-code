use std::path::PathBuf;
use tempfile::{self, NamedTempFile, TempDir};

pub trait InIO {}

pub trait FileSystemTypes {
    type Error;
}

pub trait FileSystem: FileSystemTypes {
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
    fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
}

impl<T> FileSystem for T
    where T: FileSystemTypes + InIO,
        <T as FileSystemTypes>::Error: From<std::io::Error>
{
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {
        let dir =
            tempfile::tempdir()
            .map(TempDir::into_path)?;
        Ok(dir)
    }

    fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
    let file = NamedTempFile::new()?;
    let file = file.path().to_owned();
    Ok(file)
    }
}
