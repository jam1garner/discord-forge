use std::fs;
use super::error::ConvertError;
use std::path::{Path, PathBuf};
use nutexb::{ddsfile::Dds, DdsExt};

use super::{Converter, Convert};

pub struct NutexbConverter;

impl Converter for NutexbConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "dds" => Convert::To,
            //"" => Convert::From,
            _ => Convert::None
        }
    }

    fn convert_to(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError> {
        let dds = Dds::read(&mut fs::File::open(path)?).unwrap();
        let mut out_path = PathBuf::from(path);
        out_path.set_extension("nutexb");
        dds.write_nutexb_to_file(&out_path, message)?;
        Ok(out_path)
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        todo!()
    }
}

