use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::ops::Range;
use std::io::Read;

use hound::{WavReader, WavWriter};
use samplerate::ConverterType::SincBestQuality;
use nus3audio::{AudioFile, Nus3audioFile};

use super::error::ConvertError;
use super::{Converter, Convert};

pub struct Nus3audioConverter;

const FORMAT_ERROR: &str = "Bad message format. Use either start-end or start,end";

pub fn message_to_range(message: &str, num_samples: usize) -> Result<Range<usize>, ConvertError> {
    let sep = |c| c == ',' || c == '-';
    let bounds = message
        .trim_end_matches(sep)
        .split(sep)
        .map(|s| Ok(usize::from_str_radix(s.trim(), 10)?))
        .collect::<Result<Vec<usize>, ConvertError>>()
        .map_err(|_| ConvertError::message_format(FORMAT_ERROR))?;

    if let &[start, end] = &bounds[..] {
        if end <= num_samples {
            Ok(start..end)
        } else {
            Err(ConvertError::nus3audio(&format!(
                "Bad loop points. There are only {} samples", num_samples
            )))
        }
    } else if let &[start] = &bounds[..] {
        Ok(start..num_samples)
    } else {
        return Err(ConvertError::message_format(FORMAT_ERROR))
    }
}

fn resample_wav(path: &Path) -> Result<(), ConvertError> {
    let output = Command::new("python3")
        .env("LIBROSA_CACHE_DIR", "/tmp/librosa_cache")
        .env("NUMBA_CACHE_DIR", "/tmp/numba_cache")
        .arg("resample.py")
        .arg(path)
        .output()?;
    if !output.status.success() {
        return Err(ConvertError::nus3audio(
            &(
                String::new() +
                std::str::from_utf8(&output.stdout)? +
                std::str::from_utf8(&output.stderr)?
            )
        ))
    } else {
        Ok(())
    }

}

fn get_wav_sample_count(path: &Path) -> Result<u32, ConvertError> {
    let wav = WavReader::new(fs::File::open(path)?)?;
    Ok(wav.len())
}

impl Converter for Nus3audioConverter {
    fn get_conversion(&self, file_extension: &str, _: &Path) -> Convert {
        match file_extension {
            "wav" | "lopus" => Convert::To,
            "nus3audio" => Convert::From,
            _ => Convert::None
        }
    }

    fn convert_to(&self, path: &Path, message: Option<&str>) -> Result<PathBuf, ConvertError> {
        let mut lopuspath = PathBuf::from(path);
        lopuspath.set_extension("lopus");
        if lopuspath != path {
            let old_samples = get_wav_sample_count(path)?;
            resample_wav(path)?;
            let new_samples = get_wav_sample_count(path)?;
            let conversion_rate = (new_samples as f64) / (old_samples as f64);

            let mut command = 
                Command::new("dotnet");

            command
                .arg("vgaudio/netcoreapp2.0/VGAudioCli.dll")
                .arg("-c")
                .arg(path)
                .arg(&lopuspath)
                .arg("--bitrate")
                .arg("64000")
                .arg("--CBR")
                .arg("--opusheader")
                .arg("namco");


            if let Some(message) = message {
                let Range { start, end, .. } = message_to_range(message, new_samples as _)?;
                let start = ((start as f64 * conversion_rate) + 0.5) as usize;
                let end = ((end as f64 * conversion_rate) + 0.5) as usize;
                command
                    .arg("-l")
                    .arg(format!("{}-{}", start, end));
            }

            let out = command.output()?;

            let failed = !out.status.success() |
                         !lopuspath.exists()   |
                         (fs::metadata(&lopuspath)?.len() == 0);

            if failed {
                return Err(ConvertError::nus3audio(
                    &(String::from(std::str::from_utf8(&out.stderr[..])?)
                     + std::str::from_utf8(&out.stdout[..])?)
                ))
            }
        }

        let mut outpath = PathBuf::from(path);
        outpath.set_extension("nus3audio");

        let lopus_bytes = fs::read(lopuspath)?;
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
        fs::write(&outpath, &file_bytes[..])?;
        Ok(PathBuf::from(outpath))
    }

    fn convert_from(&self, path: &Path, _: Option<&str>) -> Result<PathBuf, ConvertError> {
        let nus3_file = Nus3audioFile::open(path)?;
        let mut audiofile_path = PathBuf::from("/tmp/converter/");
        audiofile_path.push(nus3_file.files[0].filename());
        let mut outpath = audiofile_path.clone();
        outpath.set_extension("wav");
        fs::write(&audiofile_path, &nus3_file.files[0].data[..])?;
        let out = Command::new("dotnet")
            .arg("vgaudio/netcoreapp2.0/VGAudioCli.dll")
            .arg("-c")
            .arg(&audiofile_path)
            .arg(&outpath)
            .output()?;
        
        fs::remove_file(&audiofile_path)?;
        
        if !out.status.success() || !outpath.exists() {
            Err(ConvertError::nus3audio(std::str::from_utf8(&out.stdout[..])?))

        }
        else {
            Ok(PathBuf::from(outpath))
        } 
    }
}

