use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{Converter, Convert};

pub struct LuaConverter;

impl Converter for LuaConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "lua" => Convert::To,
            "lc" => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("lua");
        let out = Command::new("dotnet")
            .arg("luadec/DSLuaDecompiler.dll")
            .arg("-o")
            .arg(&outpath)
            .arg(path)
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
        outpath.set_extension("lc");
        let out = Command::new("luac")
            .arg("-s")
            .arg("-o")
            .arg(&outpath)
            .arg(path)
            .output()?;
        if !out.status.success() || !outpath.exists() {
            Err(ConvertError::param(std::str::from_utf8(&out.stderr[..])?))
        }
        else {
            Ok(PathBuf::from(outpath))
        }
    }
}

