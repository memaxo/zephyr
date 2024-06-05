use crate::error_handling::error::Error;

pub type Result<T> = std::result::Result<T, Error>;