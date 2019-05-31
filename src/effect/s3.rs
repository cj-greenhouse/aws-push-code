use futures_fs::FsPool;
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{PutObjectError, PutObjectRequest, S3Client, StreamingBody, S3 as _S3};
use bytes::Bytes;
use std::fs::{File};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use futures::Stream;

pub trait S3Types {
    type Error;
}

pub trait S3: S3Types {
    fn put_object(&self, _file: &Path, _bucket: &str, _key: &str) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn put_object_file(&self, _file: File, _bucket: &str, _key: &str) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn put_object_stream<S: Stream<Error = std::io::Error, Item = Bytes> + Send + 'static>(&self, _stream: S, _length: i64, _bucket: &str, _key: &str) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

pub trait InIO {}

pub trait SizedIOStream:  'static + Stream<Error = std::io::Error, Item = Bytes> + Send{
    type S: 'static + Stream<Error = std::io::Error, Item = Bytes> + Send;
    fn stream() -> Self::S;
}

impl<T> S3 for T
where
    T: S3Types + InIO,
    <T as S3Types>::Error: From<std::io::Error>,
    <T as S3Types>::Error: From<RusotoError<PutObjectError>>,
{
    fn put_object(&self, file: &Path, bucket: &str, key: &str) -> Result<(), Self::Error> {
        let file = File::open(file)?;
        self.put_object_file(file, bucket, key)?;
        Ok(())
    }

    fn put_object_file(&self, mut file: File, bucket: &str, key: &str) -> Result<(), Self::Error> {
        let length = file.seek(SeekFrom::End(0))? as i64;
        file.seek(SeekFrom::Start(0))?;
        let fs = FsPool::default();
        let read_stream = fs.read_file(file, Default::default());
        self.put_object_stream(read_stream, length, bucket, key)?;
        Ok(())
    }

    fn put_object_stream<S: Stream<Error = std::io::Error, Item = Bytes> + Send + 'static>(&self, stream: S, length: i64, bucket: &str, key: &str) -> Result<(), Self::Error>
         {
        let s3 = S3Client::new(Region::default());

        let request = PutObjectRequest {
            bucket: bucket.to_owned(),
            key: key.to_owned(),
            content_length: Some(length),
            body: Some(StreamingBody::new(stream)),
            ..Default::default()
        };

        s3.put_object(request).sync()?;
        Ok(())
    }
}
