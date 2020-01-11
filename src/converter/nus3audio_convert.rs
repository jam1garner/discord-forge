use std::fs;
use std::io::prelude::*;
use super::error::ConvertError;
use std::path::{Path, PathBuf};
use std::process::Command;
use nus3audio::{AudioFile, Nus3audioFile};

use super::{Converter, Convert};

pub struct Nus3audioConverter;

impl Converter for Nus3audioConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "wav" => Convert::To,
            "nus3audio" => Convert::From,
            _ => Convert::None
        }
    }

    fn convert_to(&self, path: &Path) -> Result<PathBuf, ConvertError> {
        let mut lopuspath = PathBuf::from(path);
        lopuspath.set_extension("lopus");
        let out = Command::new("dotnet")
            .arg("vgaudio/netcoreapp2.0/VGAudioCli.dll")
            .arg("-c")
            .arg(path)
            .arg(&lopuspath)
            .arg("--bitrate")
            .arg("64000")
            .arg("--CBR")
            .arg("--opusheader")
            .arg("namco")
            .output()
            .unwrap();
        
        if !out.status.success() {
            Err(ConvertError::nus3audio(std::str::from_utf8(&out.stdout[..]).unwrap()))
        }
        else {
            let mut outpath = PathBuf::from(path);
            outpath.set_extension("nus3audio");
            
            let mut lopus_bytes = vec![];
            fs::File::open(lopuspath)?
                .read_to_end(&mut lopus_bytes).unwrap();
            let nus3_file = Nus3audioFile {
                files: vec![
                    AudioFile {
                        name: String::from(path.file_stem()?.to_str()?),
                        id: 0,
                        data: lopus_bytes
                    }
                ]
            };
            let mut file_bytes = Vec::with_capacity(nus3_file.calc_size());
            nus3_file.write(&mut file_bytes);
            fs::File::create(&outpath)?
                .write_all(&file_bytes[..])?;
            Ok(PathBuf::from(outpath))
        } 
    }

    fn convert_from(&self, path: &Path) -> Result<PathBuf, ConvertError> {
        let nus3_file = Nus3audioFile::open(path).unwrap();
        let mut audiofile_path = PathBuf::from("/tmp/converter/");
        audiofile_path.push(nus3_file.files[0].filename());
        let mut outpath = audiofile_path.clone();
        outpath.set_extension("wav");
        fs::File::create(&audiofile_path).unwrap()
            .write_all(&nus3_file.files[0].data[..]).unwrap();
        let out = Command::new("dotnet")
            .arg("vgaudio/netcoreapp2.0/VGAudioCli.dll")
            .arg("-c")
            .arg(&audiofile_path)
            .arg(&outpath)
            .output()
            .unwrap();
        
        fs::remove_file(&audiofile_path)?;
        
        if !out.status.success() || !outpath.exists() {
            Err(ConvertError::nus3audio(std::str::from_utf8(&out.stdout[..]).unwrap()))

        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }
}

