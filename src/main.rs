use git2::{RemoteCallbacks, FetchOptions, Cred, Error, Oid};
use git2::build::{RepoBuilder, CheckoutBuilder};
use rusoto_core::{*};
use rusoto_secretsmanager::{GetSecretValueRequest, *};
use std::fs::{File};
use std::io::prelude::*;
use std::io::{Write, Seek};
use std::iter::Iterator;
use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use zip::{ZipWriter, CompressionMethod};
use zip::result::ZipError;
use zip::write::FileOptions;

fn main() {
    let dir = "./deleteme-repo";
    // let oid = "8cec085269b276ef6a077381a644b39529b81099";
    // let target = "8cec085269b276ef6a077381a644b39529b81099";
    // let target = "thebranch";
    let target = "thetag";
    pull(dir, "git@gitlab.cj.com:gwiley/cj.git", target).unwrap();
    zip(dir, "sources.zip").unwrap();
}

fn secret() -> Option<String> {
    std::env::var("GLPK").ok()
}


fn secret_aws() -> Option<String> {

    let secrets = SecretsManagerClient::new(Region::default());
    let request = GetSecretValueRequest {secret_id: "cj-deploy-key".to_string(), ..Default::default()};

    let response = secrets.get_secret_value(request).sync().ok()?;
    println!("{:?}", response);
    response.secret_string

}


fn pull(dir: &str, url: &str, target: &str) -> Result<(), Error> {
    let mut builder = RepoBuilder::new();
    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

    callbacks.credentials(|_, _, _| deploykey());
    // callbacks.credentials(|_, _, _| greg());
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

fn deploykey() -> Result<Cred, Error> {
    let pk = secret_aws().unwrap();
    Cred::ssh_key_from_memory("git", None, &pk[..], None)
}


fn zipd<T>(writer: T, prefix: &str, it: &mut Iterator<Item=DirEntry>) -> zip::result::ZipResult<()>
    where T: Write+Seek {

    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

fn zip(dir: &str, arch: &str) -> zip::result::ZipResult<()> {

    if !Path::new(dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let arch = Path::new(arch);
    let arch = File::create(&arch)?;

    let walk = WalkDir::new(dir.to_string()).into_iter();
    zipd(arch, dir, &mut walk.filter_map(|e| {
        match e {
            Ok(entry) => {
                let path = entry.path();
                let name = path.strip_prefix(Path::new(dir)).unwrap();
                let name = name.to_str()?;
                if name != ".git" && (name.len() < 5 || &name[0..5] != ".git/") {
                    Some(entry)
                } else {
                    None
                }
                // println!("{:?}", &name[0..0]);
                // // if entry.path().to_string().mat != ".git" {
                //     Some(entry)
                // // } else {
                // //     None
                // // }
            },
            _ => None
        }
    }))

}
