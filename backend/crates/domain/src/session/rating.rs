use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rating {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Rating {
    pub fn from_i16(value: i16) -> Result<Self, RatingError> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            other => Err(RatingError::InvalidValue(other)),
        }
    }

    pub fn as_i16(&self) -> i16 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingError {
    InvalidValue(i16),
}

impl fmt::Display for RatingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RatingError::InvalidValue(value) => write!(f, "invalid rating: {value}"),
        }
    }
}

impl std::error::Error for RatingError {}

#[cfg(test)]
mod tests {
    use super::{Rating, RatingError};

    #[test]
    fn valid_rating_is_accepted() {
        assert_eq!(Rating::from_i16(1).unwrap(), Rating::One);
        assert_eq!(Rating::from_i16(5).unwrap(), Rating::Five);
        assert_eq!(Rating::Three.as_i16(), 3);
    }

    #[test]
    fn invalid_numeric_rating_is_rejected() {
        assert_eq!(Rating::from_i16(0), Err(RatingError::InvalidValue(0)));
        assert_eq!(Rating::from_i16(6), Err(RatingError::InvalidValue(6)));
        assert_eq!(Rating::from_i16(9), Err(RatingError::InvalidValue(9)));
    }
}
