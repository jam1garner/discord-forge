use super::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use sarc::{SarcFile, Endian, SarcEntry};
use zip::{CompressionMethod, ZipArchive, ZipWriter, result::ZipError, write::FileOptions};

pub struct SarcConverter;

fn check_if_sarc(path: &Path) -> std::io::Result<bool> {
    let mut f = File::open(path)?;
    let mut magic = [0u8; 4];
    f.read(&mut magic)?;
    Ok(&magic == b"Yaz0" || &magic == b"SARC")
}

impl super::Converter for SarcConverter {
    fn get_conversion(&self, file_extension: &str, path: &Path) -> Convert {
        match file_extension {
            "zip" => Convert::To,
            _ if check_if_sarc(path).unwrap_or(false) => Convert::From,
            _ => Convert::None,
        }
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let sarc = SarcFile::read_from_file(path)?;
        let mut outpath = PathBuf::from(path);
        outpath.set_extension("zip");
        let mut zip = ZipWriter::new(File::create(&outpath)?);

        let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
        for (i, file) in sarc.files.into_iter().enumerate() {
            zip.start_file(file.name.unwrap_or_else(|| format!("{}.bin", i)), options)?;
            zip.write(&file.data)?;
        }

        Ok(outpath)
    }

    fn convert_to(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut zip = ZipArchive::new(File::open(path)?)?;

        let byte_order = match message {
            Some(s) if s.contains("3ds") | s.contains("switch") => Endian::Little,
            Some(s) if s.contains("wiiu") => Endian::Big,
            _ => Endian::Little
        };

        const COMPRESSED_EXTS: &[&str] = &[
            "ssarc", "pack", "bactorpack", "bmodelsh", "beventpack", "stera", "stats", "szs"
        ];

        const UNCOMPRESSED_EXTS: &[&str] = &[
            "sarc", "arc", "bars", "blarc", "bgenv", "genvb"
        ];

        let (file_ext, is_compressed) = match message {
            Some(s) if s.contains("uncompressed") => ("sarc", false),
            Some(s) if s.contains("yaz0") | s.contains("compressed") => ("szs", true),
            // if message contains a compressed extension
            Some(s) if COMPRESSED_EXTS.iter().any(|ext| s.contains(ext)) =>
                (*COMPRESSED_EXTS.iter().find(|ext| s.contains(*ext)).unwrap(), true),
            // if 
            Some(s) if UNCOMPRESSED_EXTS.iter().any(|ext| s.contains(ext)) =>
                (*UNCOMPRESSED_EXTS.iter().find(|ext| s.contains(*ext)).unwrap(), false),
            _ => ("szs", true),
        };

        let mut outpath = PathBuf::from(path);
        outpath.set_extension(file_ext);

        let files = (0..zip.len())
            .map(|i| -> Result<_, ConvertError> {
                let file = zip.by_index(i)?;
                let name = Some(file.name().to_owned());
                let data = file.bytes().collect::<Result<_, _>>()?;
                Ok(SarcEntry {
                    name, data
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let sarc = SarcFile {
            byte_order, files,
        };

        if is_compressed {
            sarc.write_to_compressed_file(&outpath)?;
        } else {
            sarc.write_to_file(&outpath)?;
        }

        Ok(outpath)
    }
}
