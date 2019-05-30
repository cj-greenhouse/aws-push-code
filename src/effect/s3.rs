use std::path::Path;

pub trait S3Types {
    type Error;
}

pub trait S3: S3Types {
    fn put_object(&self, _file: &Path, _bucket: &str, _key: &str) -> Result<(), Self::Error> {unimplemented!();}
}

