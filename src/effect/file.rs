use std::fs::File;
use std::path::PathBuf;
use tempfile::{self, NamedTempFile, TempDir};

pub trait InIO {}

pub trait ToFile {
    type Error;
    fn to_file(&self) -> Result<File, Self::Error> {
        unimplemented!();
    }
}

pub trait ToPath {
    type Error;
    fn to_path(&self) -> Result<PathBuf, Self::Error> {
        unimplemented!();
    }
}

pub trait FileSystemTypes {
    type Error;
}

pub trait FileSystem: FileSystemTypes {
    type TempFile: ToFile;
    type TempDirectory: ToPath;

    fn mk_temp_dir(&self) -> Result<Self::TempDirectory, Self::Error> {
        unimplemented!();
    }

    fn mk_temp_file(&self) -> Result<Self::TempFile, Self::Error> {
        unimplemented!();
    }
}

impl ToFile for NamedTempFile {
    type Error = std::io::Error;
    fn to_file(&self) -> Result<File, Self::Error> {
        self.reopen()
    }
}

impl ToPath for TempDir {
    type Error = std::io::Error;
    fn to_path(&self) -> Result<PathBuf, Self::Error> {
        Ok(self.path().to_owned())
    }
}

impl<T> FileSystem for T
where
    T: FileSystemTypes + InIO,
    <T as FileSystemTypes>::Error: From<std::io::Error>,
{
    type TempFile = NamedTempFile;
    type TempDirectory = TempDir;

    fn mk_temp_dir(&self) -> Result<Self::TempDirectory, Self::Error> {
        Ok(TempDir::new()?)
    }

    fn mk_temp_file(&self) -> Result<Self::TempFile, Self::Error> {
        Ok(NamedTempFile::new()?)
    }
}
