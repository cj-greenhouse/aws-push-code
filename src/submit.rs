

use crate::effect::repo::*;
use crate::effect::file::*;


pub trait PipelineError {}

pub trait Pipeline<E>
    where E: PipelineError {
    fn submit_to_pipeline(&self, _repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), E> {unimplemented!();}
}


pub struct PipelineT<GE, G, FSE, FS, E> {
    git: G,
    fs: FS,
    git_error: std::marker::PhantomData<GE>,
    fs_error: std::marker::PhantomData<FSE>,
    error: std::marker::PhantomData<E>,
}

impl<GE, G, FSE, FS, E> PipelineT<GE, G, FSE, FS, E> {
    pub fn new(git: G, fs: FS) -> Self {
        PipelineT {
            git, fs,
            git_error: std::marker::PhantomData,
            fs_error: std::marker::PhantomData,
            error: std::marker::PhantomData,
        }
    }
}

impl<GE, G, FSE, FS, E> Pipeline<E> for PipelineT<GE, G, FSE, FS, E>
where
        G: Git<GE>,
        GE: GitError,
        FS: FileSystem<FSE>,
        FSE: FileSystemError,
        E:
            From<GE> +
            From<FSE> +
            PipelineError
    {
    fn submit_to_pipeline(&self, repo_url: &str, _s3_bucket: &str, _s3_key: &str)  -> Result<(), E> {
        let path = self.fs.mk_temp_dir()?;
        let created = self.git.clone_repo(repo_url, &path )?;
        Ok(created)
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::{HashSet};
    use std::path::{Path, PathBuf};
    use crate::effect::repo::GitError;
    use crate::effect::file::FileSystemError;

    struct FS(PathBuf);
    impl FileSystemError for String {}
    impl FileSystem<String> for FS {
        fn mk_temp_dir(&self) -> Result<PathBuf, String> {
            Ok(self.0.clone())
        }
    }

    struct G(HashSet<(String, String)>);
    impl GitError for String {}
    impl Git<String> for G {
        fn clone_repo(&self, from: &str, to: &Path) -> Result<(), String> {
            if ! self.0.contains(&(from.to_owned(),to.to_str().unwrap().to_owned())) {
                panic!("unexpected clone parameters")
            }
            Ok(())
        }
    }

    impl PipelineError for String {}

    #[test]
    fn happy() {
        let repo = "git@foo:thingbarnone";
        let dir = "X29304";
        let fs = FS(PathBuf::from(dir));
        let git = G([(repo.to_owned(), dir.to_owned())].iter().cloned().collect());
        let r: PipelineT<String, G, String, FS, String> = PipelineT::new(git, fs);

        let actual = r.submit_to_pipeline(repo, "", "");

        assert_eq!(actual, Ok(()));
    }
}

