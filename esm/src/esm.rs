use crate::dev::*;

pub mod mapped;
pub mod raw;
pub mod full;

// ====================================================================================================


pub trait ESMUtils {
    fn load_file(file_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn load_dir(dir_path: &str) -> Result<Self, ESMError> where Self: Sized;
    fn append<T>(&mut self, other: &T);
    fn parse(i: &[u8]) -> Result<Self, ESMError> where Self: Sized;
}

// ====================================================================================================

#[derive(Debug)]
pub enum ESMError {
    IO(std::io::Error),
    Nom(nom::Err<nom::error::Error<&'static [u8]>>),
    InvalidFile,
    InvalidHeader,
    InvalidRecord,
    InvalidField,
    InvalidGroup,
    InvalidVersionControl,
    InvalidData,
    NotEnoughBytes(String),
    StringConversionError(String),
    GameSetting(String)
}

impl From<std::io::Error> for ESMError {
    fn from(err: std::io::Error) -> Self {
        ESMError::IO(err)
    }
}

impl From<nom::Err<nom::error::Error<&'static[u8]>>> for ESMError {
    fn from(value: nom::Err<nom::error::Error<&'static[u8]>>) -> Self {
        ESMError::Nom(value)
    }
}

// ================================================================================

#[derive(Debug, PartialEq)]
pub enum ESMParseMode {
    Full,
    DataOnly,
    ReferenceOnly,
}