use chrono::NaiveDate;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(NaiveDate);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DateError {
    #[error("session date must not be empty")]
    Empty,
    #[error("session date must use format YYYY-MM-DD")]
    InvalidFormat,
}

impl Date {
    pub fn parse(value: &str) -> Result<Self, DateError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DateError::Empty);
        }

        NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
            .map(Self)
            .map_err(|_| DateError::InvalidFormat)
    }

    pub fn new(value: NaiveDate) -> Self {
        Self(value)
    }

    pub fn as_naive_date(&self) -> NaiveDate {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{Date, DateError};

    #[test]
    fn valid_date_is_accepted() {
        let date = Date::parse("2026-04-14").unwrap();
        assert_eq!(date.as_naive_date(), NaiveDate::from_ymd_opt(2026, 4, 14).unwrap());
    }

    #[test]
    fn empty_date_is_rejected() {
        assert_eq!(Date::parse(""), Err(DateError::Empty));
    }

    #[test]
    fn invalid_date_format_is_rejected() {
        assert_eq!(Date::parse("14-04-2026"), Err(DateError::InvalidFormat));
    }
}
