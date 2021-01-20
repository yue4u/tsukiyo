use std::fmt;
use validator::ValidationError;
#[derive(Debug)]
pub struct MessageError {
    message: String,
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl MessageError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::error::Error for MessageError {}

pub trait OrMessageError<T> {
    fn or_error(self, message: &str) -> Result<T, MessageError>;
}

impl<T> OrMessageError<T> for Option<T> {
    fn or_error(self, message: &str) -> Result<T, MessageError> {
        match self {
            Some(data) => Ok(data),
            _ => Err(MessageError::new(message).into()),
        }
    }
}

pub fn not_empty(input: &str) -> Result<(), ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::new("should not be empty"));
    }
    Ok(())
}
