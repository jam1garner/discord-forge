use super::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct BymlConverter;

fn check_if_byml(path: &Path) -> std::io::Result<bool> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 4];
    f.read(&mut magic)?;
    Ok(&magic[..2] == b"BY" || &magic[..2] == b"YB" || &magic == b"Yaz0")
}

pub const EXTENSIONS: &[&str] = &[
    "baischedule", "baniminfo", "bgdata", "bgsvdata", "bquestpack", "bquestpack", "byml", "mubin"
];

impl Converter for BymlConverter {
    fn get_conversion(&self, file_extension: &str, path: &Path) -> Convert {
        match file_extension {
            "yml" => Convert::To,
            _ if check_if_byml(path).unwrap_or(false) => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("yml");
        let out = Command::new("byml_to_yml")
            .arg(path)
            .arg(&outpath)
            .output()?;
        if out.status.success() {
            Ok(outpath)
        } else {
            Err(ConvertError::byml(
                format!("{}{}",
                    std::str::from_utf8(&out.stdout)?,
                    std::str::from_utf8(&out.stderr)?,
                )
            ))
        }
    }

    fn convert_to(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError> {
        let little_endian = match message {
            Some(s) if s.contains("wiiu") | s.contains("wii u") | s.contains("Wii U")
                        | s.contains("wii U") | s.contains("big") => false,
            _ => true
        };

        let (compress, ext) = if let Some(s) = message {
            let comp_ext = EXTENSIONS.iter()
                .map(|ext| String::from("s") + ext)
                .find(|ext| s.contains(&*ext));
            let uncomp_ext = EXTENSIONS.iter()
                                .find(|ext| s.contains(*ext))
                                .map(|s| String::from(*s));
            (comp_ext.is_some(), comp_ext.or(uncomp_ext))
        } else {
            (false, None)
        };
        
        let mut outpath = PathBuf::from(path);
        let ext = ext.unwrap_or("byml".into());
        outpath.set_extension(&ext);
        let out = if little_endian {
            Command::new("yml_to_byml")
                .arg(path)
                .arg(&outpath)
                .output()?
        } else {
            Command::new("yml_to_byml")
                .arg(path)
                .arg(&outpath)
                .arg("-b")
                .output()?
        };
        
        if compress && !ext.starts_with("s") {
            compress_file(&outpath)?;
        }

        if out.status.success() {
            Ok(outpath)
        } else {
            Err(ConvertError::byml(
                format!("{}{}",
                    std::str::from_utf8(&out.stdout)?,
                    std::str::from_utf8(&out.stderr)?,
                )
            ))
        }
    }
}

fn compress_file<P: AsRef<Path>>(path: P) -> Result<(), ConvertError> {
    let path = path.as_ref();
    let data = std::fs::read(path)?;
    Ok(
        yaz0::Yaz0Writer::new(&mut File::create(path)?)
            .compress_and_write(
                &data,
                yaz0::deflate::CompressionLevel::Lookahead { quality: 10 }
            )?
    )
}
