use std::error::Error;

pub mod data;
pub mod solution;

pub type SimpleResult<T> = std::result::Result<T, Box<dyn Error>>;
