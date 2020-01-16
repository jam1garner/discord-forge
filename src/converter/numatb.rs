use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{Converter, Convert};

pub struct MaterialConverter;

impl Converter for MaterialConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "xml" => Convert::To,
            "numatb" => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("xml");
        let out = Command::new("dotnet")
            .arg("matlab/MatLab.dll")
            .arg(path)
            .arg(&outpath)
            .output()?;
        if !out.status.success() || !outpath.exists() {
            Err(ConvertError::param(std::str::from_utf8(&out.stderr[..])?))
        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }

    fn convert_to(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("numatb");
        let out = Command::new("dotnet")
            .arg("matlab/MatLab.dll")
            .arg(path)
            .arg(&outpath)
            .output()?;
        if !out.status.success() || !outpath.exists() {
            Err(ConvertError::param(std::str::from_utf8(&out.stderr[..])?))
        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }
}

