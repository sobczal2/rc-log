pub mod difficulty;
pub mod tag;
pub mod repository;

use std::collections::BTreeSet;

use uuid::Uuid;

use crate::{
    maneuver::{difficulty::Difficulty, tag::Tag},
    shared::{markdown_text::MarkdownText, vehicle_type::VehicleType, video_path::VideoPath},
};

#[derive(Debug, Clone)]
pub struct Maneuver {
    id: Uuid,
    vehicle_type: VehicleType,
    name: String,
    tags: BTreeSet<Tag>,
    description: MarkdownText,
    difficulty: Difficulty,
    video_path: Option<VideoPath>,
}

