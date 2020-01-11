mod msc;
mod error;
mod param;
mod nus3audio_convert;
use error::ConvertError;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

static CONVERTERS: &[&dyn Converter] = &[
    &msc::MscsbConverter,
    &nus3audio_convert::Nus3audioConverter,
    &param::ParamConverter
];

pub fn extension<'a>(path: &'a Path) -> &'a str {
    path.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap()
}

pub fn convert<P: AsRef<Path>>(path: P) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let ext = extension(path);
    let return_path = 'ret_path: {
        let mut last_err = None;
        for converter in CONVERTERS {
            match match converter.get_conversion(ext, path) {
                Convert::To => converter.convert_to(path),
                Convert::From => converter.convert_from(path),
                Convert::None => continue
            } {
                return_path @ Ok(_) => break 'ret_path return_path,
                err @ Err(_) => last_err = Some(err),
            }
        }
        
        last_err.unwrap_or_else(|| Err(ConvertError::bad_extension()))
    };

    std::fs::remove_file(path)?;
    
    return_path
}

enum Convert {
    To,
    From,
    None
}

trait Converter: Sync {
    fn get_conversion(&self, file_extension: &str, path: &Path) -> Convert;
    fn convert_to(&self, path: &Path) -> Result<PathBuf, ConvertError>;
    fn convert_from(&self, path: &Path) -> Result<PathBuf, ConvertError>;
}
