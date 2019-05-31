use crate::effect::repo::{self, Git, GitTypes};
use crate::effect::file::{self, FileSystem};
use crate::effect::zip::{self, Zip, ZipTypes};
use crate::effect::s3::{self, S3, S3Types};
use crate::submit::{SubmitTypes};

use std::path::{PathBuf, Path};

pub struct Runtime;

impl Runtime {
    pub fn new() -> Runtime {Runtime}
}

impl GitTypes for Runtime { type Error = RuntimeError; }

impl FileSystem for Runtime {
    type Error = std::io::Error;
    fn mk_temp_dir(&self) -> Result<PathBuf, Self::Error> {
        file::mk_temp_dir()
    }
    fn mk_temp_file(&self) -> Result<PathBuf, Self::Error> {
        file::mk_temp_file()
    }
}

impl ZipTypes for Runtime { type Error = RuntimeError; }
impl zip::InIO for Runtime {}

impl S3Types for Runtime  {type Error = RuntimeError; }
impl s3::InIO for Runtime {}

#[derive(Debug)]
pub struct RuntimeError(String);

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError(format!("IO: {}", error))
    }
}

impl From<git2::Error> for RuntimeError {
    fn from(error: git2::Error) -> Self {
        RuntimeError(format!("GIT: {}", error))
    }
}

impl From<::zip::result::ZipError> for RuntimeError {
    fn from(error: ::zip::result::ZipError) -> Self {
        RuntimeError(format!("ZIP: {}", error))
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::PutObjectError>> for RuntimeError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::PutObjectError>) -> Self {
        RuntimeError(format!("Rusoto: {}", error))
    }
}

impl SubmitTypes for Runtime { type Error = RuntimeError;}

