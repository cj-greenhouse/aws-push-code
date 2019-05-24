use crate::effect::repo::*;
use crate::effect::file::*;


pub trait SubmitE {
    type Error;
}

pub trait Submit: SubmitE {
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {unimplemented!();}
}

impl<T> Submit for T
    where
        T: Git + FileSystem + SubmitE,
        <T as SubmitE>::Error: From<<T as Git>::Error> + From<<T as FileSystem>::Error> {
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

    struct R<'a>(PathBuf, HashSet<(&'a str, &'a str)>);

    impl FileSystem for R<'_> {
        type Error = ();
        fn mk_temp_dir(&self) -> Result<PathBuf, ()> {
            Ok(self.0.clone())
        }
    }

    impl Git for R<'_> {
        type Error = ();
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), ()> {
            if ! self.1.contains(&(from,to.to_str().unwrap())) {
                panic!("unexpected clone parameters")
            }
            Ok(())
        }
    }

    impl SubmitE for R<'_> {type Error = ();}

    #[test]
    fn happy() {
        let dir = "X29304";
        let repo = "git@foo:thingbarnone";

        let r = R(PathBuf::from(dir),[(repo, dir)].iter().cloned().collect());

        let actual = r.submit_to_pipeline(repo, "", "");

        assert_eq!(actual, Ok(()));
    }
}

