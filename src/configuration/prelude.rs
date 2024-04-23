pub use crate::configuration::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
