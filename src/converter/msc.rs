use std::process::Command;
use super::error::ConvertError;
use std::path::{Path, PathBuf};

pub fn convert<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let mut outpath = PathBuf::from(path.clone());
    outpath.set_extension("c");
    let out = Command::new("python3")
        .arg("mscdec/mscdec.py")
        .arg("-x")
        .arg("mscdec/mscinfo.xml")
        .arg("-c")
        .arg(path)
        .arg("-o")
        .arg(&outpath)
        .output()
        .unwrap();
    if !out.status.success() {
        Err(ConvertError::param(std::str::from_utf8(&out.stdout[..]).unwrap()))
    }
    else {
        Ok(PathBuf::from(outpath))
    } 
}

pub fn convert_back<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let mut outpath = PathBuf::from(path.clone());
    outpath.set_extension("mscsb");
    let out = Command::new("python3")
        .arg("msclang/msclang.py")
        .arg("-x")
        .arg("msclang/mscinfo.xml")
        .arg(path)
        .arg("-o")
        .arg(&outpath)
        .output()
        .unwrap();
    if !out.status.success() {
        Err(ConvertError::param(std::str::from_utf8(&out.stdout[..]).unwrap()))
    }
    else {
        Ok(PathBuf::from(outpath))
    } 
}
