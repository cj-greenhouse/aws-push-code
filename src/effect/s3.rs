use std::path::Path;
use std::fs::File;
use futures_fs::FsPool;
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{S3 as _S3, S3Client, StreamingBody, PutObjectRequest, PutObjectError};

pub trait S3Types {
    type Error;
}

pub trait S3: S3Types {
    fn put_object(&self, _file: &Path, _bucket: &str, _key: &str) -> Result<(), Self::Error> {unimplemented!();}
}

pub trait InIO {}

impl<T> S3 for T where
    T: S3Types + InIO,
    <T as S3Types>::Error: From<std::io::Error>,
    <T as S3Types>::Error: From<RusotoError<PutObjectError>> {
        fn put_object(&self, file: &Path, bucket: &str, key: &str) -> Result<(), Self::Error> {
            let s3 = S3Client::new(Region::default());

            let meta = std::fs::metadata(file)?;
            let fs = FsPool::default();
            let zipfile = File::open(file)?;
            let read_stream = fs.read_file(zipfile, Default::default());

            let request = PutObjectRequest {
                bucket: bucket.to_owned(),
                key: key.to_owned(),
                content_length: Some(meta.len() as i64),
                body: Some(StreamingBody::new(read_stream)),
                ..Default::default()
            };

            s3.put_object(request).sync()?;
            Ok(())
        }
}

