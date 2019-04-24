use crate::effect::repo::Git;
use crate::effect::file::{FileSystem};

use std::collections::{HashSet};
use std::path::{Path, PathBuf};



pub fn submit_to_pipeline<T, E>(runtime: &T, repo_url: &str, s3_bucket: &str, s3_key: &str) -> Result<(), E>
    where   T: FileSystem + Git,
            E: From<T::FileSystemError> + From<T::GitError>
{
    let path = runtime.mk_temp_dir()?;
    runtime.clone_repo(repo_url, &path)?;
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::*;

    struct R<'a>(PathBuf, HashSet<(&'a str, &'a str)>);
    type E = String;

    impl<'a> FileSystem for R<'a> {
        type FileSystemError = String;
        fn mk_temp_dir(&self) -> Result<PathBuf, String> {
            Ok(self.0.clone())
        }
    }

    impl<'a> Git for R<'a> {
        type GitError = String;
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), String> {
            if ! self.1.contains(&(from,to.to_str().unwrap())) {
                panic!("unexpected clone parameters")
            }
            Ok(())
        }
    }

    #[test]
    fn happy() {
        const REPO: &str = "git@foo:thingbarnone";
        const DIR: &str = "X29304";
        let r = R(PathBuf::from(DIR), [(REPO, DIR)].iter().cloned().collect());


        let actual = submit_to_pipeline::<R, E>(&r, REPO, "", "");

        assert_eq!(actual, Ok(()));
    }
}

