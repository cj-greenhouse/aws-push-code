use std::path::Path;

pub trait ZipTypes {
    type Error;
}

pub trait Zip : ZipTypes {
    fn zip_directory(&self, _from: &Path, _to: &Path) -> Result<(), Self::Error> {unimplemented!();}
}


