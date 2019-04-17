use git2::{Repository, RemoteCallbacks, FetchOptions, Cred, Error};
use git2::build::{RepoBuilder};
use std::path::Path;

fn main() {

    let repo = clone("git@gitlab.cj.com:gwiley/cj.git", "./deleteme-repo").unwrap();
}





fn clone(url: &str, dir: &str) -> Result<Repository, Error> {
    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

    callbacks.credentials(|_,_,_| greg());
    fetch_options.remote_callbacks(callbacks);
    builder.fetch_options(fetch_options);

    builder.clone(url, Path::new(dir))
}

fn greg() -> Result<Cred, Error> {
    Cred::ssh_key(
        "git",
        None,
        Path::new("/Users/gwiley/.ssh/id_rsa"),
        None
    )
}

// fn pull(url: &str, oid: &str) -> Result<(), Error> {



//     let repo = Repository::clone(url, "./deleteme-repo")?;
//     let oid = Oid::from_str(oid)?;
//     let obj = repo.find_object(oid, None)?;
//     repo.checkout_tree(&obj, None)

// }

// fn push() {

// }




