use crate::effect::file::ToFile;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

pub trait ZipTypes {
    type Error;
    type File: ToFile;
}

pub trait Zip: ZipTypes {
    fn zip_directory(&self, _from: &Path, _to: &Self::File) -> Result<(), Self::Error> {
        unimplemented!();
    }
}

pub trait InIO {}

impl<T> Zip for T
where
    T: ZipTypes + InIO,
    <T as ZipTypes>::Error: From<ZipError>,
    <T as ZipTypes>::Error: From<std::io::Error>,
    <T as ZipTypes>::Error: From<<<T as ZipTypes>::File as ToFile>::Error>,
{
    fn zip_directory(&self, dir: &Path, arch: &Self::File) -> Result<(), Self::Error> {
        if !dir.is_dir() {
            return Err(Self::Error::from(ZipError::FileNotFound));
        }

        let mut arch = arch.to_file()?;

        arch.seek(SeekFrom::Start(0))?;
        arch.set_len(0)?;

        let walk = WalkDir::new(dir).into_iter();
        zipd(
            &mut arch,
            dir,
            &mut walk.filter_map(|e| match e {
                Ok(entry) => {
                    let path = entry.path();
                    let name = path.strip_prefix(Path::new(dir)).unwrap();
                    let name = name.to_str()?;
                    if name != ".git" && (name.len() < 5 || &name[0..5] != ".git/") {
                        Some(entry)
                    } else {
                        None
                    }
                }
                _ => None,
            }),
        )?;
        arch.sync_data()?;
        Ok(())
    }
}

fn zipd<T>(
    writer: &mut T,
    prefix: &Path,
    it: &mut Iterator<Item = DirEntry>,
) -> Result<(), ZipError>
where
    T: Write + Seek,
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        if path.is_file() {
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory_from_path(name, options)?;
        }
    }
    // placeholder for more useful build info. probably this will
    // replace the "commit-id" file generated in the clone effect
    zip.start_file_from_path(Path::new("cjbuildinfo.json"), options)?;
    let buildinfo = format!("{{\"commit\":\"{}\"}}", "abc12349024");
    zip.write_all(buildinfo.as_bytes())?;
    zip.finish()?;
    Ok(())
}
