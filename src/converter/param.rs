use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{Converter, Convert};

pub struct ParamConverter;

impl Converter for ParamConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "xml" => Convert::To,
            "prc" => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path) -> Result<PathBuf, ConvertError> {
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

    fn convert_to(&self, path: &Path) -> Result<PathBuf, ConvertError> {
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
}

