use std::error;
use std::fmt;
use std::path::PathBuf;

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub enum FileErrorType {
//     General,
// }

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct FileError<E: Sized> (FileErrorType, E);

// impl<E: error::Error> error::Error for FileError<E> {}

// impl<E: fmt::Display> fmt::Display for FileError<E> {
//     fn fmt(&FileError(t, e): &Self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "File Error {} : {}", t, e)
//     }
// }

pub trait FileSystemError {}

pub trait FileSystem {
    type Error: FileSystemError + error::Error + Sized;
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {unimplemented!();}
}

