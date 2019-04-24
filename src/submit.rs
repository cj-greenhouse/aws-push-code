use crate::effect::repo::Git;
use crate::effect::file::{FileSystem};


pub trait PipelineError {
    type PipelineError;
}

pub trait Pipeline<E>
{
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), E> {unimplemented!();}
}

impl<T> Pipeline<T::PipelineError> for T
where   T: FileSystem + Git + PipelineError,
        T::PipelineError: From<T::GitError> + From<T::FileSystemError> {
    fn submit_to_pipeline(&self, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), <T as PipelineError>::PipelineError> {
        let path = self.mk_temp_dir()?;
        let created = self.clone_repo(repo_url, &path )?;
        Ok(created)
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::{HashSet};
    use std::path::{Path, PathBuf};

    struct R<'a>(PathBuf, HashSet<(&'a str, &'a str)>);
    impl PipelineError for R<'_> {type PipelineError = String;}

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


        let actual = r.submit_to_pipeline(REPO, "", "");
        // let actual = submit_to_pipeline::<R, E>(&r, REPO, "", "");

        assert_eq!(actual, Ok(()));
    }
}

