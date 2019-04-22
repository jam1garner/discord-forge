use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn convert<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let outpath = String::from(path.to_str().unwrap()) + ".xml";
    let out = Command::new("dotnet")
        .arg("paramxml/netcoreapp2.1/ParamXML.dll")
        .arg("-l")
        .arg("paramxml/netcoreapp2.1/ParamLabels.csv")
        .arg("-d")
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
    let outpath = String::from(path.to_str().unwrap()) + ".prc";
    let out = Command::new("dotnet")
        .arg("paramxml/netcoreapp2.1/ParamXML.dll")
        .arg("-l")
        .arg("paramxml/netcoreapp2.1/ParamLabels.csv")
        .arg("-a")
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
