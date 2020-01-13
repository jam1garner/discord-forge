use std::process::Command;
use super::error::ConvertError;
use super::Convert;
use std::path::{Path, PathBuf};

pub struct MscsbConverter;

impl super::Converter for MscsbConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "mscsb" => Convert::From,
            "c" => Convert::To,
            _ => Convert::None
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
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
            .output()?;
        if !out.status.success() {
            Err(ConvertError::msc(
                &(String::from(
                    std::str::from_utf8(&out.stdout[..])?) + "\n" +
                    std::str::from_utf8(&out.stderr[..])?
            )))
        }
        else {
            Ok(PathBuf::from(outpath))
        }
    }

    fn convert_to(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path.clone());
        outpath.set_extension("mscsb");
        let out = Command::new("python3")
            .arg("msclang/msclang.py")
            .arg("-x")
            .arg("msclang/mscinfo.xml")
            .arg(path)
            .arg("-o")
            .arg(&outpath)
            .output()?;
        if !out.status.success() {
            Err(ConvertError::msc(
                &(String::from(
                    std::str::from_utf8(&out.stdout[..])?) + "\n" +
                    std::str::from_utf8(&out.stderr[..])?
            )))
        }
        else {
            Ok(PathBuf::from(outpath))
        }
    }
}
