use std::io::{Write, Seek, Read};
use std::path::Path;
use std::fs::File;
use walkdir::{WalkDir, DirEntry};
use zip::{ZipWriter, CompressionMethod};
use zip::result::ZipError;
use zip::write::FileOptions;

pub trait ZipTypes {
    type Error;
}

pub trait Zip : ZipTypes {
    fn zip_directory(&self, _from: &Path, _to: &Path) -> Result<(), Self::Error> {unimplemented!();}
}


pub fn zip(dir: &Path, arch: &str) -> Result<(), ZipError> {

    if !dir.is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let arch = Path::new(arch);
    let arch = File::create(&arch)?;

    let walk = WalkDir::new(dir).into_iter();
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
            },
            _ => None
        }
    }))

}


fn zipd<T>(writer: T, prefix: &Path, it: &mut Iterator<Item=DirEntry>) -> Result<(), ZipError>
    where T: Write+Seek {

    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated);

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
        } else if name.as_os_str().len() != 0 {
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.start_file_from_path(Path::new("cjbuildinfo.json"), options)?;
    let buildinfo = format!("{{\"commit\":\"{}\"}}", "abc12349024");
    zip.write_all(buildinfo.as_bytes())?;
    zip.finish()?;
    Ok(())
}
