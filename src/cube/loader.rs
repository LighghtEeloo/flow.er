use crate::cube::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use yew::web_sys::console;

pub trait Barge {
    fn from_json(json_str: String) -> Result<Cube>;
    fn export_json(&self) -> String;
    fn load(path: &Path) -> Result<Cube>;
    fn dump(&self, path: &Path) -> Result<()>;
}

impl Barge for Cube {
    fn from_json(json_str: String) -> Result<Self> {
        let json_res: Self = serde_json::from_str(json_str.as_str())?;
        Ok(json_res)
    }
    fn export_json(&self) -> String {
        let obj = serde_json::json!(self);
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        obj.serialize(&mut ser).unwrap();
        let res = String::from_utf8(ser.into_inner()).unwrap();
        res
    }

    // json load
    fn load(path: &Path) -> Result<Cube> {
        unimplemented!();
        match File::open(path) {
            Ok(mut f) => {
                let mut json_str = String::new();
                f.read_to_string(&mut json_str)?;
                Ok(Cube::from_json(json_str)?)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(Cube::new())
                } else {
                    Err(Error::from(e))
                }
            }
        }
    }
    
    // json dump
    fn dump(&self, path: &Path) -> Result<()> {
        unimplemented!();
        // Debug..
        console::log_1(&format!("Doing dump: {:?}", path).into());
        let mut json_file = File::create("a.json")?;
        let json_str = self.export_json();
        match json_file.write_all(json_str.as_ref()) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(Error::from(e))
            }
        }
    }
    
}

