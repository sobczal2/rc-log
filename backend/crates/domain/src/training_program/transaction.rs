use std::future::Future;

use crate::shared::pagination::Pagination;
use crate::shared::transaction::{Transaction, TransactionError};
use crate::training_program::TrainingProgram;
use crate::training_program::id::TrainingProgramId;
use crate::user::id::UserId;

pub trait TrainingProgramTransaction: Transaction<TrainingProgram> {
    fn get_by_id(
        &mut self,
        id: TrainingProgramId,
    ) -> impl Future<Output = Result<Option<TrainingProgram>, TransactionError>> + Send;

    fn list(
        &mut self,
        pagination: Pagination,
    ) -> impl Future<Output = Result<(Vec<TrainingProgram>, u64), TransactionError>> + Send;

    fn list_by_author(
        &mut self,
        author_id: UserId,
        pagination: Pagination,
    ) -> impl Future<Output = Result<(Vec<TrainingProgram>, u64), TransactionError>> + Send;

    fn delete_by_id(
        &mut self,
        id: TrainingProgramId,
    ) -> impl Future<Output = Result<(), TransactionError>> + Send;
}
