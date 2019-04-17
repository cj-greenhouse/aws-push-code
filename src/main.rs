use git2::{Repository, RemoteCallbacks, FetchOptions, Cred, Error, Oid};
use git2::build::{RepoBuilder, CheckoutBuilder};
use std::path::Path;

fn main() {
    let dir = "./deleteme-repo";
    let oid = "8cec085269b276ef6a077381a644b39529b81099";
    pull(dir, "git@gitlab.cj.com:gwiley/cj.git", oid).unwrap();
}




fn pull(dir: &str, url: &str, oid: &str) -> Result<(), Error> {
    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

    callbacks.credentials(|_,_,_| greg());
    fetch_options.remote_callbacks(callbacks);
    builder.fetch_options(fetch_options);

    let repo = builder.clone(url, Path::new(dir))?;

    let oid = Oid::from_str(oid)?;
    let obj = repo.find_object(oid, None)?;
    println!("{:?}", obj);
    repo.checkout_tree(&obj, None)
}

fn greg() -> Result<Cred, Error> {
    Cred::ssh_key(
        "git",
        None,
        Path::new("/Users/gwiley/.ssh/id_rsa"),
        None
    )
}



