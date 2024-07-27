use std::convert::Infallible;

use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;
//TODO: as we keep going on with the app, we smurf this
// pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // externals
    #[from]
    Io(std::io::Error), // as example
    
}

// impl Error {
//     pub fn custom(val: impl std::fmt::Display) -> Self {
//         Self::Custom(val.to_string())
//     }
// }
//
// impl From<&str> for Error {
//     fn from(value: &str) -> Self {
//         Self::Custom(value.to_string())
//     }
// }

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// this video so goated
//https://www.youtube.com/watch?v=j-VQCYP7wyw
