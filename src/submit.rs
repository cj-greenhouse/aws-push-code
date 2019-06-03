use crate::effect::file::{FileSystem, FileSystemTypes, ToFile, ToPath};
use crate::effect::repo::{Git, GitTypes};
use crate::effect::s3::{S3Types, S3};
use crate::effect::zip::{Zip, ZipTypes};

pub trait SubmitTypes {
    type Error;
}

pub trait Submit: SubmitTypes {
    fn submit_to_pipeline(
        &self,
        _repo_url: &str,
        _s3_bucket: &str,
        _s3_key: &str,
    ) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

impl<T> Submit for T
where
    T: Git + FileSystem + SubmitTypes + Zip + S3,
    <T as SubmitTypes>::Error: From<<T as FileSystemTypes>::Error>
        + From<<T as GitTypes>::Error>
        + From<<T as S3Types>::Error>
        + From<<T as ZipTypes>::Error>
        + From<<<T as FileSystem>::TempDirectory as ToPath>::Error>,
{
    fn submit_to_pipeline(
        &self,
        repo_url: &str,
        s3_bucket: &str,
        s3_key: &str,
    ) -> Result<(), Self::Error> {
        let tempdir = self.mk_temp_dir_n()?;        // should delete dir when scope destroyed
        let path = tempdir.to_path()?;
        let archive = self.mk_temp_file()?;
        self.clone_repo(repo_url, &path, "master")?;
        self.zip_directory(&path, &archive)?;
        self.put_object(&archive, s3_bucket, s3_key)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn happy() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2::test_case(repo, bucket, key, "master");

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Ok(()));
    }

    #[test]
    fn zip_error() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2 {
            zip: ZIP::default(),
            ..R2::test_case(repo, bucket, key, "master")
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn git_error() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2 {
            git: GIT::default(),
            ..R2::test_case(repo, bucket, key, "master")
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn tmpdir_error() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2 {
            tmpdir: None,
            ..R2::test_case(repo, bucket, key, "master")
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn tmpfile_error() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2 {
            tmpfile: None,
            ..R2::test_case(repo, bucket, key, "master")
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    #[test]
    fn put_error() {
        let repo = "git@foo:thingbarnone";
        let bucket = "sourcebucket";
        let key = "sourceobjectname";

        let r = R2 {
            s3: SSS::default(),
            ..R2::test_case(repo, bucket, key, "master")
        };

        let actual = r.submit_to_pipeline(repo, bucket, key);

        assert_eq!(actual, Err(()));
    }

    impl ToFile for String { type Error = (); }
    impl ToPath for String {
        type Error = ();
        fn to_path(&self) -> Result<PathBuf, Self::Error> {
            Ok(Path::new(self).to_owned())
        }
    }

    type FS = (Option<String>, Option<String>);
    impl FileSystemTypes for FS {
        type Error = ();
    }
    impl FileSystem for FS {
        type TempFile = String;
        type TempDirectory = String;
        fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
            match &self.1 {
                Some(p) => Ok(PathBuf::from(p)),
                None => Err(()),
            }
        }
        fn mk_temp_dir_n(&self) -> Result<String, Self::Error> {
            match &self.0 {
                Some(p) => Ok(p.clone()),
                None => Err(()),
            }
        }
        fn mk_temp_file_n(&self) -> Result<String, Self::Error> {
            match &self.1 {
                Some(p) => Ok(p.clone()),
                None => Err(()),
            }
        }
    }

    type GIT = Option<(String, String, String)>;
    impl GitTypes for GIT {
        type Error = ();
    }
    impl Git for GIT {
        fn clone_repo(&self, from: &str, to: &Path, target: &str) -> Result<(), Self::Error> {
            match self {
                Some((f, t, targ)) => {
                    if f == from && t == to.to_str().unwrap() && targ == target {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        }
    }

    type ZIP = Option<(String, String)>;
    impl ZipTypes for ZIP {
        type Error = ();
    }

    impl Zip for ZIP {
        fn zip_directory(&self, from: &Path, to: &Path) -> Result<(), Self::Error> {
            match self {
                Some((f, t)) => {
                    if f == from.to_str().unwrap() && t == to.to_str().unwrap() {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        }
    }

    type SSS = Option<(String, String, String)>;
    impl S3Types for SSS {
        type Error = ();
    }
    impl S3 for SSS {
        fn put_object(&self, from: &Path, bucket: &str, key: &str) -> Result<(), Self::Error> {
            match self {
                Some((f, b, k)) => {
                    if f == from.to_str().unwrap() && b == bucket && k == key {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        }
    }

    struct R2 {
        tmpdir: Option<String>,
        tmpfile: Option<String>,
        git: GIT,
        zip: ZIP,
        s3: SSS,
    }

    impl R2 {
        fn test_case(repo: &str, bucket: &str, key: &str, target: &str) -> R2 {
            let tmpdir = "X29304";
            let tmpfile = "90a90AAC";

            R2 {
                tmpdir: Some(tmpdir.to_owned()),
                tmpfile: Some(tmpfile.to_owned()),
                git: Some((repo.to_owned(), tmpdir.to_owned(), target.to_owned())),
                zip: Some((tmpdir.to_owned(), tmpfile.to_owned())),
                s3: Some((tmpfile.to_owned(), bucket.to_owned(), key.to_owned())),
            }
        }
    }

    impl FileSystemTypes for R2 {
        type Error = <FS as FileSystemTypes>::Error;
    }
    impl FileSystem for R2 {
        type TempFile = <FS as FileSystem>::TempFile;
        type TempDirectory = <FS as FileSystem>::TempDirectory;
        fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
            (self.tmpdir.clone(), self.tmpfile.clone()).mk_temp_file()
        }
        fn mk_temp_dir_n(&self) -> Result<Self::TempDirectory, Self::Error> {
            (self.tmpdir.clone(), self.tmpfile.clone()).mk_temp_dir_n()
        }
    }

    impl GitTypes for R2 {
        type Error = <GIT as GitTypes>::Error;
    }
    impl Git for R2 {
        fn clone_repo(&self, from: &str, to: &Path, target: &str) -> Result<(), Self::Error> {
            self.git.clone_repo(from, to, target)
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

    impl SubmitTypes for R2 {
        type Error = ();
    }

}
