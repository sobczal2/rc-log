pub mod difficulty;
pub mod id;
pub mod maneuver_resolver;
pub mod tag;
pub mod transaction;
pub mod variation;
pub mod variation_resolver;

use std::collections::BTreeSet;

use crate::{
    maneuver::{
        difficulty::Difficulty,
        id::ManeuverId,
        tag::{Tag, TagId},
        variation::Variation,
    },
    shared::{markdown_text::MarkdownText, vehicle_type::VehicleType},
};

#[derive(Debug, Clone)]
pub struct Maneuver {
    id: ManeuverId,
    vehicle_type: VehicleType,
    name: String,
    tags: BTreeSet<Tag>,
    description: MarkdownText,
    default_variation: Variation,
    other_variations: Vec<Variation>,
}

impl Maneuver {
    pub fn new(
        id: ManeuverId,
        vehicle_type: VehicleType,
        name: String,
        tags: BTreeSet<Tag>,
        description: MarkdownText,
        default_variation: Variation,
        other_variations: Vec<Variation>,
    ) -> Self {
        Self {
            id,
            vehicle_type,
            name,
            tags,
            description,
            default_variation,
            other_variations,
        }
    }

    pub fn id(&self) -> ManeuverId {
        self.id
    }

    pub fn vehicle_type(&self) -> &VehicleType {
        &self.vehicle_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &BTreeSet<Tag> {
        &self.tags
    }

    pub fn description(&self) -> &MarkdownText {
        &self.description
    }

    pub fn min_difficulty(&self) -> Difficulty {
        std::iter::once(self.default_variation.difficulty())
            .chain(self.other_variations.iter().map(|v| v.difficulty()))
            .min()
            .expect("maneuver always has at least one variation")
    }

    pub fn max_difficulty(&self) -> Difficulty {
        std::iter::once(self.default_variation.difficulty())
            .chain(self.other_variations.iter().map(|v| v.difficulty()))
            .max()
            .expect("maneuver always has at least one variation")
    }

    pub fn default_variation(&self) -> &Variation {
        &self.default_variation
    }

    pub fn other_variations(&self) -> &[Variation] {
        &self.other_variations
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag_id: TagId) -> Option<Tag> {
        let found = self.tags.iter().find(|t| t.id() == tag_id).cloned()?;
        self.tags.remove(&found);
        Some(found)
    }

    pub fn update_description(&mut self, description: MarkdownText) {
        self.description = description;
    }
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use crate::asset::name::AssetName;
    use crate::maneuver::difficulty::Difficulty;
    use crate::maneuver::id::ManeuverId;
    use crate::maneuver::tag::{Tag, TagId};
    use crate::maneuver::variation::{Variation, VariationId};
    use crate::shared::markdown_text::MarkdownText;
    use crate::shared::vehicle_type::VehicleType;

    use super::Maneuver;

    fn make_variation(difficulty: Difficulty) -> Variation {
        Variation::new(
            VariationId::new(Uuid::new_v4()),
            ManeuverId::new(Uuid::new_v4()),
            "default".to_string(),
            MarkdownText::new("description".to_string()).unwrap(),
            AssetName::new("asset".to_string()).unwrap(),
            difficulty,
        )
    }

    fn make_maneuver() -> Maneuver {
        Maneuver::new(
            ManeuverId::new(Uuid::new_v4()),
            VehicleType::Helicopter,
            "test maneuver".to_string(),
            BTreeSet::new(),
            MarkdownText::new("some description".to_string()).unwrap(),
            make_variation(Difficulty::Level1),
            vec![],
        )
    }

    #[test]
    fn add_tag_inserts_tag() {
        let mut m = make_maneuver();
        let tag = Tag::new(TagId::new(Uuid::new_v4()), "beginner".to_string());
        m.add_tag(tag.clone());
        assert!(m.tags().contains(&tag));
    }

    #[test]
    fn add_tag_duplicate_is_idempotent() {
        let mut m = make_maneuver();
        let tag = Tag::new(TagId::new(Uuid::new_v4()), "beginner".to_string());
        m.add_tag(tag.clone());
        m.add_tag(tag.clone());
        assert_eq!(m.tags().len(), 1);
    }

    #[test]
    fn remove_tag_returns_tag_and_removes_it() {
        let mut m = make_maneuver();
        let tag_id = TagId::new(Uuid::new_v4());
        let tag = Tag::new(tag_id, "beginner".to_string());
        m.add_tag(tag.clone());
        assert_eq!(m.tags().len(), 1);
        let removed = m.remove_tag(tag_id);
        assert_eq!(removed, Some(tag));
        assert_eq!(m.tags().len(), 0);
    }

    #[test]
    fn remove_tag_with_non_empty_name_works() {
        let mut m = make_maneuver();
        let tag_id = TagId::new(Uuid::new_v4());
        m.add_tag(Tag::new(tag_id, "a-tag-with-a-real-name".to_string()));
        let removed = m.remove_tag(tag_id);
        assert!(removed.is_some(), "remove_tag must find tags by id regardless of name");
        assert_eq!(m.tags().len(), 0);
    }

    #[test]
    fn remove_tag_nonexistent_returns_none() {
        let mut m = make_maneuver();
        assert_eq!(m.remove_tag(TagId::new(Uuid::new_v4())), None);
    }

    #[test]
    fn update_description_changes_content() {
        let mut m = make_maneuver();
        let new_desc = MarkdownText::new("updated description".to_string()).unwrap();
        m.update_description(new_desc);
        assert_eq!(m.description().as_str(), "updated description");
    }

    #[test]
    fn min_max_difficulty_single_variation() {
        let m = make_maneuver();
        assert_eq!(m.min_difficulty(), Difficulty::Level1);
        assert_eq!(m.max_difficulty(), Difficulty::Level1);
    }

    #[test]
    fn min_max_difficulty_multiple_variations() {
        let m = Maneuver::new(
            ManeuverId::new(Uuid::new_v4()),
            VehicleType::Helicopter,
            "test".to_string(),
            BTreeSet::new(),
            MarkdownText::new("desc".to_string()).unwrap(),
            make_variation(Difficulty::Level2),
            vec![make_variation(Difficulty::Level4), make_variation(Difficulty::Level1)],
        );
        assert_eq!(m.min_difficulty(), Difficulty::Level1);
        assert_eq!(m.max_difficulty(), Difficulty::Level4);
    }
}

