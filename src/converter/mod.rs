mod msc;
mod sqb;
mod error;
mod param;
mod numatb;
mod motion_list;
mod nus3audio_convert;
mod nutexb;
mod sarc_converter;
use error::ConvertError;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

static CONVERTERS: &[&dyn Converter] = &[
    &msc::MscsbConverter,
    &nus3audio_convert::Nus3audioConverter,
    &param::ParamConverter,
    &motion_list::MotionListConverter,
    &sqb::SqbConverter,
    &numatb::MaterialConverter,
    &nutexb::NutexbConverter,
    &sarc_converter::SarcConverter,
];

pub use error::SUPPORTED_TYPES;

pub fn extension<'a>(path: &'a Path) -> &'a str {
    path.extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap()
}

fn as_non_empty_string(string: &str) -> Option<&str> {
    match string {
        "" => None,
        a => Some(a)
    }
}

pub fn convert<P: AsRef<Path>>(path: P, message: &str) -> Result<PathBuf, ConvertError> {
    let path = path.as_ref();
    let ext = extension(path);
    let message = as_non_empty_string(message);
    let return_path = 'ret_path: {
        let mut last_err = None;
        for converter in CONVERTERS {
            match match converter.get_conversion(ext, path) {
                Convert::To => converter.convert_to(path, message),
                Convert::From => converter.convert_from(path, message),
                Convert::None => continue
            } {
                return_path @ Ok(_) => break 'ret_path return_path,
                err @ Err(_) => last_err = Some(err),
            }
        }
        
        last_err.unwrap_or_else(|| Err(ConvertError::bad_extension()))
    }?;

    std::fs::remove_file(path)?;
    
    if return_path.exists() {
        Ok(return_path)
    } else {
        Err(ConvertError::file("Returned file not found"))
    }
}

enum Convert {
    To,
    From,
    None
}

trait Converter: Sync {
    fn get_conversion(&self, file_extension: &str, path: &Path) -> Convert;
    fn convert_to(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError>;
    fn convert_from(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError>;
}
