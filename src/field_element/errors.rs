use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldError {
    InvalidValue,
}

impl Display for FieldError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FieldError::InvalidValue => write!(f, "Invalid value"),
        }
    }
}
