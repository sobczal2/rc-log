use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Quality {
    pub fn from_i16(value: i16) -> Result<Self, RatingError> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            other => Err(RatingError::InvalidQuality(other)),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Comfort {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Comfort {
    pub fn from_i16(value: i16) -> Result<Self, RatingError> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            other => Err(RatingError::InvalidComfort(other)),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Repeatability {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Repeatability {
    pub fn from_i16(value: i16) -> Result<Self, RatingError> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            other => Err(RatingError::InvalidRepeatability(other)),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rating {
    quality: Quality,
    comfort: Comfort,
    repeatability: Repeatability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingError {
    InvalidQuality(i16),
    InvalidComfort(i16),
    InvalidRepeatability(i16),
}

impl fmt::Display for RatingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RatingError::InvalidQuality(value) => write!(f, "invalid quality rating: {value}"),
            RatingError::InvalidComfort(value) => write!(f, "invalid comfort rating: {value}"),
            RatingError::InvalidRepeatability(value) => {
                write!(f, "invalid repeatability rating: {value}")
            }
        }
    }
}

impl std::error::Error for RatingError {}

impl Rating {
    pub fn new(quality: Quality, comfort: Comfort, repeatability: Repeatability) -> Self {
        Self { quality, comfort, repeatability }
    }

    pub fn quality(&self) -> Quality {
        self.quality
    }

    pub fn comfort(&self) -> Comfort {
        self.comfort
    }

    pub fn repeatability(&self) -> Repeatability {
        self.repeatability
    }
}

#[cfg(test)]
mod tests {
    use super::{Comfort, Quality, Rating, RatingError, Repeatability};

    #[test]
    fn valid_rating_is_accepted() {
        let rating = Rating::new(Quality::Four, Comfort::Five, Repeatability::Three);
        assert_eq!(rating.quality(), Quality::Four);
        assert_eq!(rating.comfort(), Comfort::Five);
        assert_eq!(rating.repeatability(), Repeatability::Three);
    }

    #[test]
    fn invalid_numeric_rating_is_rejected() {
        assert_eq!(Quality::from_i16(0), Err(RatingError::InvalidQuality(0)));
        assert_eq!(Comfort::from_i16(6), Err(RatingError::InvalidComfort(6)));
        assert_eq!(Repeatability::from_i16(9), Err(RatingError::InvalidRepeatability(9)));
    }
}
