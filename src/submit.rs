use crate::effect::repo::*;
use crate::effect::file::*;


pub trait SubmitTypes {
    type Error;
}

pub trait Submit: SubmitTypes {
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {unimplemented!();}
}

impl<T> Submit for T
    where
        T: Git + FileSystem + SubmitTypes,
        <T as SubmitTypes>::Error: From<<T as Git>::Error> + From<<T as FileSystem>::Error> {
    fn submit_to_pipeline(&self, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {
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


    #[test]
    fn happy() {
        let tmpdir = "X29304";
        let repo = "git@foo:thingbarnone";


        let r = R2 {
            fs: PathBuf::from(tmpdir),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect()
        };

        let actual = r.submit_to_pipeline(repo, "", "");

        assert_eq!(actual, Ok(()));
    }

    impl FileSystem for PathBuf {
        type Error = ();
        fn mk_temp_dir(&self) -> Result<PathBuf, ()> {
            Ok(self.clone())
        }
    }

    impl Git for HashSet<(String, String)> {
        type Error = ();
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), ()> {
            if ! self.contains(&(from.to_owned(),to.to_str().unwrap().to_owned())) {
                panic!("unexpected clone parameters")
            }
            Ok(())
        }
    }

    struct R2 {
        fs: PathBuf,
        git: HashSet<(String, String)>,
    }

    impl FileSystem for R2 {
        type Error = <PathBuf as FileSystem>::Error;
        fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {
            self.fs.mk_temp_dir()
        }
    }

    impl Git for R2 {
        type Error = <HashSet<(String, String)> as Git>::Error;
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), Self::Error> {
            self.git.clone_repo(from, to)
        }
    }

    impl SubmitTypes for R2 {type Error = ();}

}

