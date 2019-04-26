use crate::effect::repo::Git;
use crate::effect::file::FileSystem;
use crate::submit::{self,Submit};

pub struct Runtime;

pub fn wire() -> Runtime {
    Runtime
}


impl Git for Runtime {
    type Error = ();
}

impl FileSystem for Runtime {
    type Error = ();
}

impl Submit for Runtime {
    type Error = ();
    fn submit_to_pipeline(&self, repo_url: &str, s3_bucket: &str, s3_key: &str)  -> Result<(), Self::Error> {
        submit::submit_to_pipeline::<Runtime, ()>(self, repo_url, s3_bucket, s3_key)
    }
}
