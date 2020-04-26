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
    Ok(contents.trim().to_string())
}
