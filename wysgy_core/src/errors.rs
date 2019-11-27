use std::error;
pub type WysgyResult<T> = std::result::Result<T, Box<dyn error::Error>>;
