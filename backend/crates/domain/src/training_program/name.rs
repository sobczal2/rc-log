use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrainingProgramName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrainingProgramNameError {
    Empty,
    TooLong,
}

impl fmt::Display for TrainingProgramNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrainingProgramNameError::Empty => write!(f, "training program name must not be empty"),
            TrainingProgramNameError::TooLong => {
                write!(f, "training program name must not exceed 255 characters")
            }
        }
    }
}

impl std::error::Error for TrainingProgramNameError {}

impl TrainingProgramName {
    pub fn new(value: String) -> Result<Self, TrainingProgramNameError> {
        if value.trim().is_empty() {
            return Err(TrainingProgramNameError::Empty);
        }

        if value.len() > 255 {
            return Err(TrainingProgramNameError::TooLong);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
