use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

pub fn write(path: &str, data: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn read(path: &str) -> Result<String, Error> {
    let path = Path::new(path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.to_string())
}

pub fn basename(filename: &str) -> String {
    Path::new(filename)
        .file_stem()
        .map(|s| s.to_os_string()
            .into_string()
            .unwrap_or_else(|_| filename.to_string()))
        .unwrap_or_else(|| filename.to_string())
}

pub fn extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .map(|s| s.to_os_string().into_string().ok())
        .flatten()
}
