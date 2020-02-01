use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ops::Range;
use std::io::Read;

use hound::{WavReader, WavWriter};
use samplerate::ConverterType::SincBestQuality;
use nus3audio::{AudioFile, Nus3audioFile};

use super::error::ConvertError;
use super::{Converter, Convert};

pub struct Nus3audioConverter;

const FORMAT_ERROR: &str = "Bad message format. Use either start-end or start,end";

pub fn message_to_range(message: &str) -> Result<Range<usize>, ConvertError> {
    let bounds = message
        .split(|c| c == ',' || c == '-')
        .map(|s| Ok(usize::from_str_radix(s.trim(), 10)?))
        .collect::<Result<Vec<usize>, ConvertError>>()
        .map_err(|_| ConvertError::message_format(FORMAT_ERROR))?;
    if let &[start, end] = &bounds[..] {
        Ok(start..end)
    } else {
        Err(ConvertError::message_format(FORMAT_ERROR))
    }
}

const I16_MAX: f32 = std::i16::MAX as f32;

fn max_from_bits(bits: u16) -> f32 {
    match bits {
        32 => std::i32::MAX as f32,
        24 => 0x7fffff as f32,
        16 => std::i16::MAX as f32,
        8 => std::i8::MAX as f32,
        _ => panic!("Bad bits per sample")
    }
}

fn samples_to_float(samples: Vec<i16>, bits: u16) -> Vec<f32> {
    samples.into_iter().map(|sample| sample as f32).collect()
}

fn samples_to_i16(samples: Vec<f32>) -> Vec<i16> {
    samples.into_iter().map(|sample| sample as i16).collect()
}

fn resample_wav<R: Read>(path: &Path, wav: WavReader<R>, hz: u32) -> Result<(), ConvertError> {
    let old_hz = wav.spec().sample_rate;
    let old_bits = wav.spec().bits_per_sample;

    let samples: Vec<f32> = match wav.spec().sample_format {
        hound::SampleFormat::Float => {
            return Err(ConvertError::nus3audio("f32 wavs not supported"))
        }
        hound::SampleFormat::Int => {
            samples_to_float(wav.into_samples().collect::<Result<_, _>>()?, old_bits)
        }
    };
    let samples = samples_to_i16(samplerate::convert(old_hz, hz, 1, SincBestQuality, &samples)?);

    let samples: Vec<_> = samples.into_iter()
        .enumerate()
        .filter_map(|(i, sample)|{
            if i % 2 == 0 {
                Some(sample)
            } else {
                None
            }
        })
        .collect();
    
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: hz,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };

    let mut writer = WavWriter::create(path, spec)?;

    for sample in samples {
        writer.write_sample(sample)?;
    }

    Ok(())
}

fn check_wav_samples(path: &Path, hz: u32) -> Result<(), ConvertError> {
    let wav = WavReader::new(fs::File::open(path)?)?;
    if wav.spec().sample_rate == hz {
        Ok(())
    } else {
        resample_wav(path, wav, hz)
    }
}

fn check_wav_sample_count(path: &Path, count: usize) -> Result<(), ConvertError> {
    let wav = WavReader::new(fs::File::open(path)?)?;
    let num_samples = wav.len() as usize;
    if count <= num_samples {
        Ok(())
    } else {
        Err(ConvertError::nus3audio(&format!(
            "Bad loop points. There are only {} samples", num_samples
        )))
    }
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
            check_wav_samples(path, 48000)?;

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
                let range = message_to_range(message)?;
                check_wav_sample_count(path, range.end)?;
                command
                    .arg("-l")
                    .arg(format!("{}-{}", range.start, range.end));
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

