mod error;
mod param;
use error::ConvertError;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

fn extension<'a>(path: &'a Path) -> &'a str {
    path.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap()
}

pub fn convert<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let return_path = match extension(path) {
        "prc" | "stprm" | "stdat" => {
            Ok(param::convert(path))
        }
        "xml" => {
            Ok(param::convert_back(path))
        }
        _ => {
            Err(ConvertError::bad_extension())
        }
    };
    std::fs::remove_file(path);
    return_path?
}
