use std::future::Future;

use crate::maneuver::Maneuver;
use crate::maneuver::difficulty::Difficulty;
use crate::maneuver::id::ManeuverId;
use crate::maneuver::variation::{Variation, VariationId};
use crate::shared::pagination::Pagination;
use crate::shared::transaction::{Transaction, TransactionError};
use crate::shared::vehicle_type::VehicleType;

/// Transaction trait extended with Maneuver-specific operations
pub trait ManeuverTransaction: Transaction<Maneuver> {
    fn get_by_id(
        &mut self,
        id: ManeuverId,
    ) -> impl Future<Output = Result<Option<Maneuver>, TransactionError>> + Send;

    fn get_variation_by_id(
        &mut self,
        id: VariationId,
    ) -> impl Future<Output = Result<Option<Variation>, TransactionError>> + Send;

    fn list(
        &mut self,
        pagination: Pagination,
        filter: ManeuverFilter,
        sort: ManeuverSort,
    ) -> impl Future<Output = Result<(Vec<Maneuver>, u64), TransactionError>> + Send;
}

#[derive(Debug, Clone, Default)]
pub struct ManeuverFilter {
    pub tags: Vec<String>,
    pub vehicle_type: Option<VehicleType>,
    pub difficulty: Option<Difficulty>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ManeuverSortField {
    Name,
    Difficulty,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy)]
pub struct ManeuverSort {
    pub field: ManeuverSortField,
    pub direction: SortDirection,
}

impl Default for ManeuverSort {
    fn default() -> Self {
        Self { field: ManeuverSortField::Name, direction: SortDirection::Asc }
    }
}
