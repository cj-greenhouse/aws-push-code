use crate::effect::file::{self, FileSystem, FileSystemTypes};
use crate::effect::repo::{self, GitTypes};
use crate::effect::s3::{self, S3Types};
use crate::effect::secret::{self, SecretsTypes};
use crate::effect::zip::{self, ZipTypes};
use crate::submit::SubmitTypes;

pub struct Runtime;

impl Default for Runtime {
    fn default() -> Self {
        Runtime
    }
}

impl GitTypes for Runtime {
    type Error = RuntimeError;
}
impl repo::InIO for Runtime {}

impl FileSystemTypes for Runtime {
    type Error = RuntimeError;
}
impl file::InIO for Runtime {}

impl ZipTypes for Runtime {
    type Error = RuntimeError;
    type File = <Runtime as FileSystem>::TempFile;
}
impl zip::InIO for Runtime {}

impl S3Types for Runtime {
    type Error = RuntimeError;
    type File = <Runtime as FileSystem>::TempFile;
}
impl s3::InIO for Runtime {}

impl SecretsTypes for Runtime {
    type Error = RuntimeError;
}

impl secret::InAWS for Runtime {}

impl SubmitTypes for Runtime {
    type Error = RuntimeError;
}

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

impl From<secret::SecretsAWSError> for RuntimeError {
    fn from(error: secret::SecretsAWSError) -> Self {
        RuntimeError(format!("SECRET AWS: {}", error))
    }
}

impl From<String> for RuntimeError {
    fn from(error: String) -> Self {
        RuntimeError(format!("ERR: {}", error))
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::PutObjectError>> for RuntimeError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::PutObjectError>) -> Self {
        RuntimeError(format!("Rusoto: {}", error))
    }
}
