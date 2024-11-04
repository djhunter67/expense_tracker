use std::{
    error::Error,
    fmt::{self, Display},
    io,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum ModelError {
    InvalidInput,
    NotFound,
    InternalServerError,
}

impl Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModelError::InvalidInput => write!(f, "Invalid input"),
            ModelError::NotFound => write!(f, "Not found"),
            ModelError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl Error for ModelError {}

impl From<io::Error> for ModelError {
    fn from(_error: io::Error) -> Self {
        ModelError::InternalServerError
    }
}

impl From<ParseIntError> for ModelError {
    fn from(_error: ParseIntError) -> Self {
        ModelError::InvalidInput
    }
}

impl From<ParseFloatError> for ModelError {
    fn from(_error: ParseFloatError) -> Self {
        ModelError::InvalidInput
    }
}

impl From<mongodb::error::Error> for ModelError {
    fn from(_error: mongodb::error::Error) -> Self {
        ModelError::InternalServerError
    }
}
