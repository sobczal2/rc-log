use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rating {
    quality: u8,
    comfort: u8,
    repeatability: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingError {
    OutOfRange,
}

impl fmt::Display for RatingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RatingError::OutOfRange => {
                write!(f, "session rating values must be between 1 and 5")
            }
        }
    }
}

impl std::error::Error for RatingError {}

impl Rating {
    pub fn new(quality: u8, comfort: u8, repeatability: u8) -> Result<Self, RatingError> {
        if !Self::is_valid(quality) || !Self::is_valid(comfort) || !Self::is_valid(repeatability) {
            return Err(RatingError::OutOfRange);
        }

        Ok(Self { quality, comfort, repeatability })
    }

    pub fn quality(&self) -> u8 {
        self.quality
    }

    pub fn comfort(&self) -> u8 {
        self.comfort
    }

    pub fn repeatability(&self) -> u8 {
        self.repeatability
    }

    fn is_valid(value: u8) -> bool {
        (1..=5).contains(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Rating, RatingError};

    #[test]
    fn valid_rating_is_accepted() {
        let rating = Rating::new(4, 5, 3).unwrap();
        assert_eq!(rating.quality(), 4);
        assert_eq!(rating.comfort(), 5);
        assert_eq!(rating.repeatability(), 3);
    }

    #[test]
    fn out_of_range_rating_is_rejected() {
        assert_eq!(Rating::new(0, 3, 4), Err(RatingError::OutOfRange));
        assert_eq!(Rating::new(3, 6, 4), Err(RatingError::OutOfRange));
        assert_eq!(Rating::new(3, 4, 9), Err(RatingError::OutOfRange));
    }
}