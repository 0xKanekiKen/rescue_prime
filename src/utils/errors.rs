use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldError {
    DeserializationError,
}

impl Display for FieldError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::DeserializationError => write!(f, "Deserialization error due to invalid value"),
        }
    }
}
