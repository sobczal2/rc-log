use rc_log_domain::session::rating::Rating;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RatingDto {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub fn rating_to_dto(rating: Rating) -> RatingDto {
    match rating {
        Rating::One => RatingDto::One,
        Rating::Two => RatingDto::Two,
        Rating::Three => RatingDto::Three,
        Rating::Four => RatingDto::Four,
        Rating::Five => RatingDto::Five,
    }
}

pub fn rating_from_dto(rating: RatingDto) -> Rating {
    match rating {
        RatingDto::One => Rating::One,
        RatingDto::Two => Rating::Two,
        RatingDto::Three => Rating::Three,
        RatingDto::Four => Rating::Four,
        RatingDto::Five => Rating::Five,
    }
}
