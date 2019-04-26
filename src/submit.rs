use crate::effect::repo::*;
use crate::effect::file::*;


pub trait Submit {
    type Error;
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), Self::Error> {unimplemented!();}
}

pub fn submit_to_pipeline<RT, E>(runtime: &RT, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), E>
    where
        RT: Git + FileSystem,
        E: From<<RT as Git>::Error> + From<<RT as FileSystem>::Error> {
    let path = runtime.mk_temp_dir()?;
    let created = runtime.clone_repo(repo_url, &path )?;
    Ok(created)
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

    #[test]
    fn happy() {
        let dir = "X29304";
        let repo = "git@foo:thingbarnone";

        let r = R(PathBuf::from(dir),[(repo, dir)].iter().cloned().collect());

        let actual = submit_to_pipeline::<R, ()>(&r, repo, "", "");

        assert_eq!(actual, Ok(()));
    }
}

