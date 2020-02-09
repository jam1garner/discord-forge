#![allow(dead_code)]
use std::fmt;

pub struct ConvertError {
    pub message: String,
    pub kind: ConvertErrorKind,
}

impl fmt::Debug for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConvertError '{:?}', message: \"{}\"", self.kind, self.message)
    }
}

pub static SUPPORTED_TYPES: &str = "prc, xml, wav, lopus, nus3audio, dds, mscsb, c, sqb, yaml, numatb, stprm, stdat, motion_list.bin, zip, sarc, szs, pack, bactorpack, bmodelsh, beventpack, stera, stats, arc, bars, blarc, bgenv, genvb" ;

impl ConvertError {
    pub fn bad_extension() -> ConvertError {
        ConvertError {
            message: format!("Unsupported Filetype. Supported types: {}", SUPPORTED_TYPES),
            kind: ConvertErrorKind::BadExtension,
        }
    }

    pub fn param(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::Param
        }
    }

    pub fn nus3audio(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::Nus3audio
        }
    }

    pub fn file(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::File
        }
    }

    pub fn msc(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::Msc
        }
    }

    pub fn message_format(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::MessageFormat
        }
    }
}

impl std::convert::From<std::io::Error> for ConvertError {
    fn from(err: std::io::Error) -> Self {
        ConvertError {
            message: format!("{:?}", err),
            kind: ConvertErrorKind::File,
        }
    }
}

impl std::convert::From<std::option::NoneError> for ConvertError {
    fn from(err: std::option::NoneError) -> Self {
        ConvertError {
            message: format!("{:?}", err),
            kind: ConvertErrorKind::HandleNone,
        }
    }
}

impl std::convert::From<serde_yaml::Error> for ConvertError {
    fn from(err: serde_yaml::Error) -> Self {
        ConvertError {
            message: format!("{:?}", err),
            kind: ConvertErrorKind::YamlError,
        }
    }
}

impl std::convert::From<std::str::Utf8Error> for ConvertError {
    fn from(err: std::str::Utf8Error) -> Self {
        ConvertError {
            message: format!("{:?}", err),
            kind: ConvertErrorKind::Utf8Error,
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for ConvertError {
    fn from(err: std::num::ParseIntError) -> Self {
        ConvertError {
            message: format!("{:?}", err),
            kind: ConvertErrorKind::ParseIntError,
        }
    }
}

impl std::convert::From<hound::Error> for ConvertError {
    fn from(err: hound::Error) -> Self {
        ConvertError {
            message: format!("HoundError: {:?}", err),
            kind: ConvertErrorKind::Nus3audio,
        }
    }
}

impl std::convert::From<samplerate::Error> for ConvertError {
    fn from(err: samplerate::Error) -> Self {
        ConvertError {
            message: format!("SampleRateError: {:?}", err),
            kind: ConvertErrorKind::Nus3audio,
        }
    }
}

impl std::convert::From<sarc::parser::Error> for ConvertError {
    fn from(err: sarc::parser::Error) -> Self {
        ConvertError {
            message: format!("SarcParseError: {:?}", err),
            kind: ConvertErrorKind::SarcError
        }
    }
}

impl std::convert::From<sarc::writer::Error> for ConvertError {
    fn from(err: sarc::writer::Error) -> Self {
        ConvertError {
            message: format!("SarcWriteError: {:?}", err),
            kind: ConvertErrorKind::SarcError
        }
    }
}

impl std::convert::From<zip::result::ZipError> for ConvertError {
    fn from(err: zip::result::ZipError) -> Self {
        ConvertError {
            message: format!("ZipError: {:?}", err),
            kind: ConvertErrorKind::ZipError
        }
    }
}

#[derive(Debug)]
pub enum ConvertErrorKind {
    BadExtension,
    Param,
    Nus3audio,
    Msc,
    File,
    HandleNone,
    YamlError,
    Utf8Error,
    ParseIntError,
    MessageFormat,
    WaveError,
    SarcError,
    ZipError
}
