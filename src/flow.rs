use crate::repo::Git;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::io;

// pub struct Request {}

// pub trait Submission
//     where Self : CodeRepository {
//     type SubmissionError;
//     fn submit_to_pipeline(&mut self, request: &Request) -> Result<(), Self::Error>;
// }

pub trait MkTemp {
    type MkTempError;
    fn mk_temp_dir() -> Result<PathBuf, Self::MkTempError> {unimplemented!();}
}

// pub trait Store {
//     type Error;
//     fn store(src: &Path, dest: &str) -> Result<(), Self::Error> {unimplemented!();}
// }

// pub enum SubmitError<T>
// where   T: MkTemp + Git {
//     MTE(T::MkTempError),
//     GE(T::GitError),
// }

// pub trait Submit: MkTemp + Git + Sized {
//     type SubmitError: Sized;
//     fn submit(repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(), Self::SubmitError> {
//         unimplemented!();
//     }
// }




// pub trait Submit<E> {
//     fn stp(&self, repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(),E>;
// }


// impl<T,E> Submit<E> for T
// where   T: MkTemp + Git,
//         E: From<T::MkTempError> + From<T::GitError> {
//     fn stp(&self, repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(),E> {
//         let path = self.mk_temp_dir()
//         unimplemented!();
//     }
// }





pub fn submit_to_pipeline<T, E>(repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(), E>
    where   T: MkTemp + Git,
            E: From<T::MkTempError> + From<T::GitError>
{
    let path = T::mk_temp_dir()?;
    let repo = T::clone(repo_url, &path)?;
    Ok(())
}


// the trait bound `flow::SubmitError<T>: std::convert::From<<T as flow::MkTemp>::MkTempError>` is not satisfied

// the trait `std::convert::From<<T as flow::MkTemp>::MkTempError>` is not implemented for `flow::SubmitError<T>`

// help: consider adding a `where flow::SubmitError<T>: std::convert::From<<T as flow::MkTemp>::MkTempError>` bound
// note: required by `std::convert::From::from`rustc(E0277)





// impl<T> Submit for T
// where   T: MkTemp + Git,
//         Self::SubmitError: From<T::MkTempError> + From<T::GitError> {
//     // fn submit(repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(), Self::SubmitError> {
//     //     let path = T::mk_temp_dir()?;
//     //     let repo = T::clone(repo_url, &path)?;
//     //     Ok(())
//     // }
// }


// pub trait App<W, D> {
//     type Error;
//     fn submi(_repo: &RemoteRepository, _dest: &D) -> Result<(), Self::Error>;
// }



// pub fn submit_to_pipeline<W>(_wiring: &mut W, _repo_url: &str) -> Result<(),String>
//     where W: CodeRepository {
//         // W::pull_repository("thing").unwrap();
//         Ok(())
// }

#[cfg(test)]
mod tests {

    use super::*;

    struct R<'a>(HashMap<&'a str, PathBuf>, HashSet<(&'a str, &'a str)>);
    type E = String;

    impl<'a> MkTemp for R<'a> {
        type MkTempError = String;
        fn mk_temp_dir() -> Result<PathBuf, String> {

            unimplemented!();
        }
    }

    #[test]
    fn happy() {
        const REPO: &str = "git@foo:thingbarnone";
        const DIR: &str = "X29304";

        impl MkTemp for R {type MkTempError = E; fn mk_temp_dir() -> Result<PathBuf, E> {Ok(From::from(DIR.to_owned()))}}
        impl Git for R {type GitError = E; fn clone(_: &str, _: &Path) -> Result<(), E> {Ok(())} }

        let actual = submit_to_pipeline::<R,E>(REPO, "", "");

        assert_eq!(actual, Ok(()));
    }
}

