use super::error::ConvertError;
use std::path::{Path, PathBuf};
use super::{Converter, Convert};

pub struct SqbConverter;

impl Converter for SqbConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "yaml" => Convert::To,
            "sqb" => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("yaml");
        std::fs::write(&outpath, serde_yaml::to_string(&sqb::open(path)?)?)?;
        Ok(outpath)
    }

    fn convert_to(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("sqb");
        sqb::save(&outpath, &serde_yaml::from_str(&std::fs::read_to_string(path)?)?)?;
        Ok(outpath)
    }
}
