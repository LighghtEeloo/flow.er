use crate::cube::{Collection, Result, Error};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

// json load
pub fn load(path: &Path) -> Result<Collection> {
    match File::open(path) {
        Ok(mut f) => {
            let mut json_str = String::new();
            f.read_to_string(&mut json_str)?;
            Ok(Collection::from_json(json_str)?)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(Collection::new())
            } else {
                Err(Error::from(e))
            }
        }
    }
}

// json dump
pub fn dump(tr: &Collection, path: &Path) -> Result<()> {
    let mut json_file = File::create(path)?;
    let json_str = tr.export_json();
    match json_file.write_all(json_str.as_ref()) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(Error::from(e))
        }
    }
}
