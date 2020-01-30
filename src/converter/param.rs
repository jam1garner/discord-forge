use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{Converter, Convert};

pub struct ParamConverter;

impl Converter for ParamConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "xml" => Convert::To,
            "prc" | "stprm" | "stdat" => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("xml");
        let out = Command::new("dotnet")
            .arg("paramxml/netcoreapp2.1/ParamXML.dll")
            .arg("-l")
            .arg("paramxml/netcoreapp2.1/ParamLabels.csv")
            .arg("-d")
            .arg(path)
            .arg("-o")
            .arg(&outpath)
            .output()?;
        let output = std::str::from_utf8(&out.stdout[..])?;
        if !out.status.success() || output.contains("Trace") || !outpath.exists() {
            Err(ConvertError::param(output))
        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }

    fn convert_to(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("prc");
        let out = Command::new("dotnet")
            .arg("paramxml/netcoreapp2.1/ParamXML.dll")
            .arg("-l")
            .arg("paramxml/netcoreapp2.1/ParamLabels.csv")
            .arg("-a")
            .arg(path)
            .arg("-o")
            .arg(&outpath)
            .output()?;
        let output = std::str::from_utf8(&out.stdout[..])?;
        if !out.status.success() || output.contains("Trace") || !outpath.exists() {
            Err(ConvertError::param(output))
        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }
}

