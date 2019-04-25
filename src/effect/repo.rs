use std::fmt;
use std::error;
use std::path::Path;


// #[derive(Debug, PartialEq, Eq, Hash)]
// pub enum GitErrorType {
//     General,
// }

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct GitError<E: Sized> (GitErrorType, E);

// impl<E: error::Error> error::Error for GitError<E> {}

// impl<E: fmt::Display> fmt::Display for GitError<E> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let GitError(l,e) = self;
//         write!(f, "Git Error {} : {}", l, e)
//     }
// }


pub trait GitError {}

pub trait Git {
    type Error: GitError + error::Error + Sized;
    fn clone_repo(&self, _from: &str, _to: &Path) -> Result<(), Self::Error> {unimplemented!();}
}


