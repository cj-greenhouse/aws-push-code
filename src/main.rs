use git2::{RemoteCallbacks, FetchOptions, Cred, Error, Oid};
use git2::build::{RepoBuilder, CheckoutBuilder};
use std::path::Path;

fn main() {
    let dir = "./deleteme-repo";
    // let oid = "8cec085269b276ef6a077381a644b39529b81099";
    // let target = "8cec085269b276ef6a077381a644b39529b81099";
    // let target = "thebranch";
    let target = "thetag";
    pull(dir, "git@gitlab.cj.com:gwiley/cj.git", target).unwrap();
}




fn pull(dir: &str, url: &str, target: &str) -> Result<(), Error> {
    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

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


    // let target = repo.find_reference(target)?;

    // // let target = repo.find_reference(target)?;
    // let commit = target.peel_to_commit()?;


    // let branch_name = "__SUBMISSION";
    // let branch = repo.branch(branch_name, &commit, false)?;
    // let branch_ref = branch.get();
    // let branch_tree = branch_ref.peel_to_tree()?;
    // let branch_obj = branch_tree.as_object();
    // repo.checkout_tree(&branch_obj, Some(&mut co))?;
    // repo.set_head(branch_ref.name().unwrap())?;

    // repo.set_head_detached(oid)

    // Ok(())
}

fn greg() -> Result<Cred, Error> {
    Cred::ssh_key(
        "git",
        None,
        Path::new("/Users/gwiley/.ssh/id_rsa"),
        None
    )
}



