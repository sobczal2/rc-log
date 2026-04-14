use std::future::Future;

use crate::maneuver::id::ManeuverId;
use crate::model::id::ModelId;
use crate::session::Session;
use crate::session::id::SessionId;
use crate::shared::pagination::Pagination;
use crate::shared::transaction::{Transaction, TransactionError};
use crate::user::id::UserId;

pub trait SessionTransaction: Transaction<Session> {
    fn get_by_id(
        &mut self,
        id: SessionId,
    ) -> impl Future<Output = Result<Option<Session>, TransactionError>> + Send;

    fn list_by_owner(
        &mut self,
        owner_id: UserId,
        pagination: Pagination,
        filter: SessionFilter,
        sort: SessionSort,
    ) -> impl Future<Output = Result<(Vec<Session>, u64), TransactionError>> + Send;

    fn delete_by_id(
        &mut self,
        id: SessionId,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}

#[derive(Debug, Clone, Default)]
pub struct SessionFilter {
    pub model_ids: Vec<ModelId>,
    pub maneuver_ids: Vec<ManeuverId>,
    pub search_query: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum SessionSortField {
    Date,
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionSort {
    pub field: SessionSortField,
    pub direction: SortDirection,
}

impl Default for SessionSort {
    fn default() -> Self {
        Self { field: SessionSortField::Date, direction: SortDirection::Desc }
    }
}
