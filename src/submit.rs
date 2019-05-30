use crate::effect::file::{FileSystem};
use crate::effect::repo::{Git};
use crate::effect::s3::{S3, S3Types};
use crate::effect::zip::{Zip, ZipTypes};

pub trait SubmitTypes {
    type Error;
}

pub trait Submit: SubmitTypes {
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str) -> Result<(), Self::Error> {unimplemented!();}
}

impl<T> Submit for T
    where
        T: Git + FileSystem + SubmitTypes + Zip + S3,
        <T as SubmitTypes>::Error:
            From<<T as FileSystem>::Error> +
            From<<T as Git>::Error> +
            From<<T as S3Types>::Error> +
            From<<T as ZipTypes>::Error>
        {
    fn submit_to_pipeline(&self, repo_url: &str, s3_bucket: &str, s3_key: &str)  -> Result<(), Self::Error> {
        let path = self.mk_temp_dir()?;
        let archive = self.mk_temp_file()?;
        self.clone_repo(repo_url, &path )?;
        self.zip_directory(&path, &archive)?;
        self.put_object(&archive, s3_bucket, s3_key)?;
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
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (Some(PathBuf::from(tmpdir)), Some(PathBuf::from(tmpfile))),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: [(tmpdir.to_owned(), tmpfile.to_owned())].iter().cloned().collect(),
            s3: [(tmpfile.to_owned(), bucket.to_owned(), key.to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn zip_error() {
        let tmpdir = "X29304";
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (Some(PathBuf::from(tmpdir)), Some(PathBuf::from(tmpfile))),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: ZIP::default(),
            s3: [(tmpfile.to_owned(), bucket.to_owned(), key.to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn git_error() {
        let tmpdir = "X29304";
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (Some(PathBuf::from(tmpdir)), Some(PathBuf::from(tmpfile))),
            git: GIT::default(),
            zip: [(tmpdir.to_owned(), tmpfile.to_owned())].iter().cloned().collect(),
            s3: [(tmpfile.to_owned(), bucket.to_owned(), key.to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn tmpdir_error() {
        let tmpdir = "X29304";
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (None, Some(PathBuf::from(tmpfile))),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: [(tmpdir.to_owned(), tmpfile.to_owned())].iter().cloned().collect(),
            s3: [(tmpfile.to_owned(), bucket.to_owned(), key.to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn tmpfile_error() {
        let tmpdir = "X29304";
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (Some(PathBuf::from(tmpdir)), None),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: [(tmpdir.to_owned(), tmpfile.to_owned())].iter().cloned().collect(),
            s3: [(tmpfile.to_owned(), bucket.to_owned(), key.to_owned())].iter().cloned().collect(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn put_error() {
        let tmpdir = "X29304";
        let tmpfile = "90a90AAC";
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";


        let r = R2 {
            fs: (Some(PathBuf::from(tmpdir)), Some(PathBuf::from(tmpfile))),
            git: [(repo.to_owned(), tmpdir.to_owned())].iter().cloned().collect(),
            zip: [(tmpdir.to_owned(), tmpfile.to_owned())].iter().cloned().collect(),
            s3: SSS::default(),
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }


    type FS = (Option<PathBuf>, Option<PathBuf>);
    impl FileSystem for FS {
        type Error = ();
        fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {
            match &self.0 {
                Some(p) => Ok(p.clone()),
                None => Err(())
            }
        }
        fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
            match &self.1 {
                Some(p) => Ok(p.clone()),
                None => Err(())
            }
        }
    }

    type GIT = HashSet<(String,String)>;
    impl Git for GIT {
        type Error = ();
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), ()> {
            if ! self.contains(&(from.to_owned(),to.to_str().unwrap().to_owned())) {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    type ZIP = HashSet<(String, String)>;
    impl ZipTypes for ZIP {type Error = ();}

    impl Zip for ZIP {
        fn zip_directory(&self, from: &Path, to: &Path) -> Result<(), Self::Error> {
            if ! self.contains(&(from.to_str().unwrap().to_owned(),to.to_str().unwrap().to_owned())) {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    type SSS = HashSet<(String, String, String)>;
    impl S3Types for SSS {type Error = ();}
    impl S3 for SSS {
        fn put_object(&self, file: &Path, bucket: &str, key: &str) -> Result<(), Self::Error> {
            if self.contains(&(file.to_str().unwrap().to_owned(), bucket.to_owned(), key.to_owned())) {
                Ok (())
            } else {
                Err(())
            }
        }
    }

    struct R2 {
        fs: FS,
        git: GIT,
        zip: ZIP,
        s3: SSS,
    }

    impl FileSystem for R2 {
        type Error = <FS as FileSystem>::Error;
        fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {
            self.fs.mk_temp_dir()
        }
        fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
            self.fs.mk_temp_file()
        }
    }

    impl Git for R2 {
        type Error = <HashSet<(String, String)> as Git>::Error;
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), Self::Error> {
            self.git.clone_repo(from, to)
        }
    }

    impl ZipTypes for R2 {
        type Error = <ZIP as ZipTypes>::Error;
    }

    impl Zip for R2 {
        fn zip_directory(&self, from: &Path, to: &Path) -> Result<(), Self::Error> {
            self.zip.zip_directory(from, to)
        }
    }

    impl S3Types for R2 {
        type Error = <SSS as S3Types>::Error;
    }

    impl S3 for R2 {
        fn put_object(&self, file: &Path, bucket: &str, key: &str) -> Result<(), Self::Error> {
            self.s3.put_object(file, bucket, key)
        }
    }

    impl SubmitTypes for R2 {type Error = ();}

}

