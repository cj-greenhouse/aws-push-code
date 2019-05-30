use std::path::Path;
use git2::{RemoteCallbacks, FetchOptions, Cred, Error, Oid};
use git2::build::{RepoBuilder, CheckoutBuilder};

pub trait Git {
    type Error;
    fn clone_repo(&self, _from: &str, _to: &Path) -> Result<(), Self::Error> {unimplemented!();}
}

pub fn pull_git_repo(dir: &str, url: &str, target: &str) -> Result<(), Error> {
    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

    // callbacks.credentials(|_, _, _| deploykey());
    callbacks.credentials(|_, _, _| greg());
    fetch_options.remote_callbacks(callbacks);
    builder.fetch_options(fetch_options);
    let repo = builder.clone(url, Path::new(dir))?;

    let mut co = CheckoutBuilder::new();
    co.force();

    let commit = if let Ok(oid) = Oid::from_str(target) {
        repo.find_commit(oid)
    } else if let Ok(branch) = repo.find_reference(&format!("refs/remotes/origin/{}", target)[..]) {
        branch.peel_to_commit()
    } else if let Ok(tag) = repo.find_reference(&format!("refs/tags/{}", target)[..]) {
        tag.peel_to_commit()
    } else {
        Err(Error::from_str("this is not what we really want to do for error"))
    }?;

    let obj = commit.as_object();
    repo.checkout_tree(&obj, Some(&mut co))

}

fn greg() -> Result<Cred, Error> {
    Cred::ssh_key(
        "git",
        None,
        Path::new("/Users/gwiley/.ssh/id_rsa"),
        None
    )
}
