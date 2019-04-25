
use std::error;
use std::fmt;

use crate::effect::repo::{Git};
use crate::effect::file::{FileSystem};


pub trait PipelineError {}

pub trait Pipeline {
    type Error: PipelineError + error::Error + Sized;
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {unimplemented!();}
}



// #[derive(Debug)]
// pub enum PERR<T: FileSystem + Git> {
//     GError(<T as Git>::Error),
//     FSError(<T as FileSystem>::Error),
// }

// impl<T: FileSystem + Git> fmt::Display for PERR<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PERR::GError(e) => write!(f, "Git Error: {}", e),
//             PERR::FSError(e) => write!(f, "FS Error: {}", e),
//         }
//     }
// }

// impl<T: FileSystem + Git> PipelineError for PERR<T> {}
// impl error::Error for PipelineError {}




// impl<T: FileSystem + Git> Pipeline for T
// where
//     // <Self as Git>::Error: Into<<Self as Pipeline>::Error>,
//     // <Self as FileSystem>::Error: Into<<Self as Pipeline>::Error>,
//     <Self as Pipeline>::Error: From<<Self as Git>::Error> + From<<Self as Git>::Error>
// {
//     fn submit_to_pipeline(&self, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), <T as Pipeline>::Error> {
//         let path = self.mk_temp_dir()?;
//         let created = self.clone_repo(repo_url, &path )?;
//         Ok(created)
//     }

// }



// #[cfg(test)]
// mod tests {

//     use super::*;
//     use std::collections::{HashSet};
//     use std::path::{Path, PathBuf};

//     struct R<'a>(PathBuf, HashSet<(&'a str, &'a str)>);
//     impl PipelineError for R<'_> {type PipelineError = String;}

//     impl<'a> FileSystem for R<'a> {
//         type FileSystemError = String;
//         fn mk_temp_dir(&self) -> Result<PathBuf, String> {
//             Ok(self.0.clone())
//         }
//     }

//     impl<'a> Git for R<'a> {
//         type GitError = String;
//         fn clone_public_repo(&self, from: &str, to: &Path) -> GitEffect {
//             if ! self.1.contains(&(from,to.to_str().unwrap())) {
//                 panic!("unexpected clone parameters")
//             }
//             Ok(())
//         }
//     }

//     impl From<Box<dyn GitError>> for String {
//         fn from(e: Box<dyn GitError>) -> String {format!("Git Error {}", e)}
//     }

//     #[test]
//     fn happy() {
//         const REPO: &str = "git@foo:thingbarnone";
//         const DIR: &str = "X29304";
//         let r = R(PathBuf::from(DIR), [(REPO, DIR)].iter().cloned().collect());

//         let actual = r.submit_to_pipeline(REPO, "", "");

//         assert_eq!(actual, Ok(()));
//     }
// }

