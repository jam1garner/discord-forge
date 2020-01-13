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

pub static SUPPORTED_TYPES: &str = "prc, xml, wav, nus3audio, mscsb, c, yaml, motion_list.bin";

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

#[derive(Debug)]
pub enum ConvertErrorKind {
    BadExtension,
    Param,
    Nus3audio,
    Msc,
    File,
    HandleNone,
    YamlError,
}
