use std::path::{PathBuf};

use crate::effect::repo::*;
use crate::effect::file::*;
use crate::effect::zip::{Zip, ZipTypes};

pub trait SubmitTypes {
    type Error;
}

pub trait Submit: SubmitTypes {
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {unimplemented!();}
}

impl<T> Submit for T
    where
        T: Git + FileSystem + SubmitTypes + Zip,
        <T as SubmitTypes>::Error:
            From<<T as Git>::Error> +
            From<<T as FileSystem>::Error> +
            From<<T as ZipTypes>::Error>
        {
    fn submit_to_pipeline(&self, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {
    let path = self.mk_temp_dir()?;
    self.clone_repo(repo_url, &path )?;
    self.zip_directory(&path, &PathBuf::from("master.zip"))?;
    Ok(())
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
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: [(tmpdir.to_owned(), "master.zip".to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, "", "");

        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn zip_error() {
        let tmpdir = "X29304";
        let repo = "git@foo:thingbarnone";


        let r = R2 {
            fs: PathBuf::from(tmpdir),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: HashSet::new(),
        };

        let actual = r.submit_to_pipeline(repo, "", "");

        assert_eq!(actual, Err(()));
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

    impl ZipTypes for HashSet<(String, String)> {type Error = ();}

    impl Zip for HashSet<(String, String)> {
        fn zip_directory(&self, from: &Path, to: &Path) -> Result<(), Self::Error> {
            if ! self.contains(&(from.to_str().unwrap().to_owned(),to.to_str().unwrap().to_owned())) {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    struct R2 {
        fs: PathBuf,
        git: HashSet<(String, String)>,
        zip: HashSet<(String, String)>,
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

    impl ZipTypes for R2 {
        type Error = <HashSet<(String, String)> as ZipTypes>::Error;
    }

    impl Zip for R2 {
        fn zip_directory(&self, from: &Path, to: &Path) -> Result<(), Self::Error> {
            self.zip.zip_directory(from, to)
        }
    }


    impl SubmitTypes for R2 {type Error = ();}

}

