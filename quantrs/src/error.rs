use std::fmt;

#[derive(Debug)]
pub enum QuantrsError {
    InvalidPeriod,
    EmptyData,
}

impl std::error::Error for QuantrsError {}

impl fmt::Display for QuantrsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantrsError::InvalidPeriod => write!(f, "Period must be greater than 0"),
            QuantrsError::EmptyData => write!(f, "Data cannot be empty"),
        }
    }
}

pub type Result<T> = std::result::Result<T, QuantrsError>;
