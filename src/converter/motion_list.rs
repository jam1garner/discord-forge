use super::error::ConvertError;
use std::path::{Path, PathBuf};
use super::{Converter, Convert};
use hash40::*;
use byteorder::{LittleEndian};

pub struct MotionListConverter;

fn check_if_motion_bin(path: &Path) -> bool {
    std::fs::File::open(path).unwrap().read_hash40::<LittleEndian>().unwrap() == hash40!("motion")
}

impl Converter for MotionListConverter {
    fn get_conversion(&self, file_extension: &str, path: &Path) -> Convert {
        match file_extension {
            "yaml" => Convert::To,
            "bin" if check_if_motion_bin(path) => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("yaml");
        std::fs::write(&outpath, serde_yaml::to_string(&motion_lib::open(path)?)?)?;
        Ok(outpath)
    }

    fn convert_to(&self, path: &Path) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("bin");
        motion_lib::save(&outpath, &serde_yaml::from_str(&std::fs::read_to_string(path)?)?)?;
        Ok(outpath)
    }
}
