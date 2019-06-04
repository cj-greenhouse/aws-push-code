use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Cred, Error, FetchOptions, Oid, RemoteCallbacks};
use serde_json::Value;
use std::path::Path;
use crate::effect::secret::{Secrets, SecretsTypes};

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
    <T as GitTypes>::Error: From<::git2::Error>,
    <T as GitTypes>::Error: From<<T as SecretsTypes>::Error>,
    <T as SecretsTypes>::Error: std::fmt::Debug,
{
    fn clone_repo(&self, url: &str, dir: &Path, target: &str) -> Result<(), Self::Error> {
        let mut builder = RepoBuilder::new();
        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();

        callbacks.credentials(|_, _, _| { self.credentials() });
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
        Ok(())
    }
}

impl<T> GitCredentials for T
    where T: Secrets + InIO,
    git2::Error: From<<T as SecretsTypes>::Error>,
    <T as SecretsTypes>::Error: std::fmt::Debug,
     {
         fn credentials(&self) -> Result<Cred, git2::Error> {
            let secrets = self.secrets().map_err(|e| {git2::Error::from_str(&format!("credentials: {:?}", e))})?;
            let credentials = match secrets {
                Value::Object(m) => match m.get("deploy_key") {
                    Some(Value::String(s)) => Ok(s.to_owned()),
                    _ =>  Err(git2::Error::from_str("credentials: secrets object does not contain string value for 'deploy_key'"))
                },
                _ => Err(git2::Error::from_str("credentials: secrets are not an object"))
            }?;
            Cred::ssh_key_from_memory("git", None, &credentials, None)
         }
}

