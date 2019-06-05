use crate::effect::secret::{Secrets, SecretsTypes};
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Cred, Error, FetchOptions, Oid, RemoteCallbacks};
use std::path::Path;
use std::fs;

pub trait GitTypes {
    type Error;
}

pub trait Git: GitTypes {
    fn clone_repo(&self, _from: &str, _to: &Path, _target: &str) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

pub trait InIO {}

pub trait GitCredentials {
    fn credentials(&self) -> Result<git2::Cred, git2::Error> {
        unimplemented!();
    }
}

impl<T> Git for T
where
    T: GitTypes + InIO + Secrets + GitCredentials,
    <T as GitTypes>::Error: From<::git2::Error> + From<<T as SecretsTypes>::Error> + From<std::io::Error>,
{
    fn clone_repo(&self, url: &str, dir: &Path, target: &str) -> Result<(), Self::Error> {
        let mut builder = RepoBuilder::new();
        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();

        callbacks.credentials(|_, _, _| self.credentials());
        fetch_options.remote_callbacks(callbacks);
        builder.fetch_options(fetch_options);
        let repo = builder.clone(url, dir)?;

        let mut co = CheckoutBuilder::new();
        co.force();

        let commit = if let Ok(oid) = Oid::from_str(target) {
            repo.find_commit(oid)
        } else if let Ok(branch) =
            repo.find_reference(&format!("refs/remotes/origin/{}", target)[..])
        {
            branch.peel_to_commit()
        } else if let Ok(tag) = repo.find_reference(&format!("refs/tags/{}", target)[..]) {
            tag.peel_to_commit()
        } else {
            Err(Error::from_str(
                "this is not what we really want to do for error",
            ))
        }?;

        let obj = commit.as_object();
        repo.checkout_tree(&obj, Some(&mut co))?;
        fs::write(dir.join("commit-id"), commit.id().to_string())?;
        Ok(())
    }
}

fn into_git_result<V, E, F: Fn(E) -> String>(r: Result<V, E>, cb: F) -> Result<V, git2::Error> {
    r.map_err(|e| git2::Error::from_str(&(cb(e))))
}

trait Foo {}

impl<T> GitCredentials for T
where
    T: Secrets + InIO,
    <T as SecretsTypes>::Error: std::fmt::Debug,
{
    fn credentials(&self) -> Result<Cred, git2::Error> {
        /* this is not great--probably the whole configuration architecture
         * needs to be revisited. what I'm imagining is an automatic parsing
         * of secrets and merging the call results with other configuration
         * data from wiring. that could even lead to a tool for updating
         * secrets and could also be used in a secrets rotation lambda
         * function.
         */
        let secret_id_key = "CJ_PUSHCODE_GIT_CREDENTIALS_ID";
        let secret_id = std::env::var(secret_id_key);
        let secret_id = into_git_result(secret_id, |e| {
            format!(
                "credentials: cannot get secret id: {} {:?}",
                secret_id_key, e
            )
        })?;
        let credentials = self.secrets(&secret_id);
        let credentials = into_git_result(credentials, |e| {
            format!("credentials: cannot get secret: {} {:?}", secret_id, e)
        })?;
        Cred::ssh_key_from_memory("git", None, &credentials, None)
    }
}
