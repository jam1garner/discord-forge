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

impl ConvertError {
    pub fn bad_extension() -> ConvertError {
        ConvertError {
            message: String::from("Bad extension, cannot convert"),
            kind: ConvertErrorKind::BadExtension,
        }
    }

    pub fn param(message: &str) -> ConvertError {
        ConvertError {
            message: message.to_string(),
            kind: ConvertErrorKind::Param
        }
    }
}

#[derive(Debug)]
pub enum ConvertErrorKind {
    BadExtension,
    Param,
}
