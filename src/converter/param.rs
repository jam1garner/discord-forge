use super::error::ConvertError;
use std::io::Seek;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{Converter, Convert};
use prc::xml;

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
        let mut writer = std::io::BufWriter::new(std::fs::File::create(outpath)?);
        xml::write_xml(&prc::open(path)?, &mut writer)
            .or_else(|e| Err(ConvertError::param(format!("{:?}", e).as_ref())));
        Ok(outpath)
    }

    fn convert_to(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("prc");
        let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
        match xml::read_xml(&mut reader) {
            Ok(p) => prc::save(outpath, &p)?,
            Err(e) => {
                reader.seek(std::io::SeekFrom::Start(0))?;
                return Err(ConvertError::param(
                    format!(
                        "{}\n{:?}",
                        xml::get_xml_error(&mut reader, e.start, e.end)?,
                        e.error
                    ).as_ref()
                ))
            }
        }
        Ok(outpath)
    }
}

