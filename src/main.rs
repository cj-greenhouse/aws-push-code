use git2::Repository;


fn main() {
    pull();
    push();
}



fn pull() {
    let url = "https://github.com/alexcrichton/git2-rs";
    match Repository::clone(url, "./deleteme-repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

}

fn push() {

}




