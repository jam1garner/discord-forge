mod msc;
mod error;
mod param;
mod nus3audio_convert;
use error::ConvertError;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub fn extension<'a>(path: &'a Path) -> &'a str {
    path.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap()
}

pub fn convert<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let return_path = match extension(path) {
        "prc" | "stprm" | "stdat" => {
            param::convert(path)
        }
        "xml" => {
            param::convert_back(path)
        }
        "wav" => {
            nus3audio_convert::convert(path)
        }
        "nus3audio" => {
            nus3audio_convert::convert_back(path)
        }
        "mscsb" => {
            msc::convert(path)
        }
        "c" => {
            msc::convert_back(path)
        }
        _ => {
            Err(ConvertError::bad_extension())
        }
    };

    std::fs::remove_file(path)?;
    
    return_path
}
