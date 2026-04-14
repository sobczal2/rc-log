use std::collections::HashMap;

use rc_log_domain::maneuver::variation::VariationId;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::training_program::TrainingProgram;
use rc_log_domain::training_program::id::TrainingProgramId;
use rc_log_domain::training_program::name::Name;
use rc_log_domain::training_program::part::Part;
use rc_log_domain::training_program::part::PartVariation;
use rc_log_domain::training_program::part::id::TrainingProgramPartId;
use rc_log_domain::training_program::transaction::TrainingProgramTransaction;
use rc_log_domain::user::id::UserId;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct TrainingProgramRow {
    id: Uuid,
    author_id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TrainingProgramPartRow {
    id: Uuid,
    training_program_id: Uuid,
    position: i32,
    name: String,
    description: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TrainingProgramPartRowWithProgram {
    training_program_id: Uuid,
    id: Uuid,
    position: i32,
    name: String,
    description: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TrainingProgramPartVariationRow {
    part_id: Uuid,
    position: i32,
    variation_id: Uuid,
}

impl TrainingProgramRow {
    fn from_training_program(training_program: &TrainingProgram) -> Self {
        Self {
            id: Uuid::from(training_program.id()),
            author_id: Uuid::from(training_program.author_id()),
            name: training_program.name().as_str().to_string(),
            description: training_program.description().as_str().to_string(),
        }
    }

    fn try_into_training_program(
        self,
        parts: Vec<Part>,
    ) -> Result<TrainingProgram, TransactionError> {
        let name =
            Name::new(self.name).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let description = MarkdownText::new(self.description)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        Ok(TrainingProgram::new(
            TrainingProgramId::new(self.id),
            UserId::new(self.author_id),
            name,
            description,
            parts,
        ))
    }
}

impl TrainingProgramPartRow {
    fn from_training_program_part(part: &Part) -> Result<Self, TransactionError> {
        let position = i32::try_from(part.position()).map_err(|_| {
            TransactionError::InvalidData(
                "training program part position exceeds i32 range".to_string(),
            )
        })?;

        Ok(Self {
            id: Uuid::from(part.id()),
            training_program_id: Uuid::from(part.training_program_id()),
            position,
            name: part.name().as_str().to_string(),
            description: part.description().as_str().to_string(),
        })
    }

    fn try_into_training_program_part(
        self,
        variations: Vec<PartVariation>,
    ) -> Result<Part, TransactionError> {
        let name =
            Name::new(self.name).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let description = MarkdownText::new(self.description)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let position = u32::try_from(self.position).map_err(|_| {
            TransactionError::InvalidData(
                "training program part position must be non-negative".to_string(),
            )
        })?;

        Ok(Part::new(
            TrainingProgramPartId::new(self.id),
            TrainingProgramId::new(self.training_program_id),
            name,
            description,
            position,
            variations,
        ))
    }
}

impl TrainingProgramPartRowWithProgram {
    fn try_into_training_program_part(
        self,
        variations: Vec<PartVariation>,
    ) -> Result<(Uuid, Part), TransactionError> {
        let part = TrainingProgramPartRow {
            id: self.id,
            training_program_id: self.training_program_id,
            position: self.position,
            name: self.name,
            description: self.description,
        }
        .try_into_training_program_part(variations)?;

        Ok((self.training_program_id, part))
    }
}

impl TrainingProgramPartVariationRow {
    fn from_part_variation(
        part_id: TrainingProgramPartId,
        part_variation: &PartVariation,
    ) -> Result<Self, TransactionError> {
        let position = i32::try_from(part_variation.position()).map_err(|_| {
            TransactionError::InvalidData(
                "training program part variation position exceeds i32 range".to_string(),
            )
        })?;

        Ok(Self {
            part_id: Uuid::from(part_id),
            position,
            variation_id: Uuid::from(part_variation.variation_id()),
        })
    }

    fn try_into_part_variation(self) -> Result<(Uuid, PartVariation), TransactionError> {
        let position = u32::try_from(self.position).map_err(|_| {
            TransactionError::InvalidData(
                "training program part variation position must be non-negative".to_string(),
            )
        })?;

        Ok((self.part_id, PartVariation::new(VariationId::new(self.variation_id), position)))
    }
}

pub struct SqlxTrainingProgramTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl SqlxTrainingProgramTransaction {
    async fn list_internal(
        &mut self,
        count_sql: &str,
        list_sql: &str,
        author_id: Option<UserId>,
        pagination: Pagination,
    ) -> Result<(Vec<TrainingProgram>, u64), TransactionError> {
        let total: i64 = if let Some(author_id) = author_id {
            sqlx::query_scalar(count_sql)
                .bind(author_id.as_uuid())
                .fetch_one(&mut *self.tx)
                .await
                .map_err(|e| TransactionError::TransactionError(e.to_string()))?
        } else {
            sqlx::query_scalar(count_sql)
                .fetch_one(&mut *self.tx)
                .await
                .map_err(|e| TransactionError::TransactionError(e.to_string()))?
        };

        let program_rows: Vec<TrainingProgramRow> = if let Some(author_id) = author_id {
            sqlx::query_as(list_sql)
                .bind(author_id.as_uuid())
                .bind(pagination.limit() as i64)
                .bind(pagination.offset() as i64)
                .fetch_all(&mut *self.tx)
                .await
                .map_err(|e| TransactionError::TransactionError(e.to_string()))?
        } else {
            sqlx::query_as(list_sql)
                .bind(pagination.limit() as i64)
                .bind(pagination.offset() as i64)
                .fetch_all(&mut *self.tx)
                .await
                .map_err(|e| TransactionError::TransactionError(e.to_string()))?
        };

        if program_rows.is_empty() {
            return Ok((vec![], total as u64));
        }

        let program_ids: Vec<Uuid> = program_rows.iter().map(|row| row.id).collect();

        let part_rows: Vec<TrainingProgramPartRowWithProgram> = sqlx::query_as(
            r#"
            SELECT training_program_id, id, position, name, description
            FROM training_program.part
            WHERE training_program_id = ANY($1)
            ORDER BY training_program_id ASC, position ASC
            "#,
        )
        .bind(&program_ids)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let part_ids: Vec<Uuid> = part_rows.iter().map(|row| row.id).collect();

        let mut variations_by_part: HashMap<Uuid, Vec<PartVariation>> = HashMap::new();
        if !part_ids.is_empty() {
            let variation_rows: Vec<TrainingProgramPartVariationRow> = sqlx::query_as(
                r#"
                SELECT part_id, position, variation_id
                FROM training_program.part_variation
                WHERE part_id = ANY($1)
                ORDER BY part_id ASC, position ASC
                "#,
            )
            .bind(&part_ids)
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

            for row in variation_rows {
                let (part_id, variation) = row.try_into_part_variation()?;
                variations_by_part.entry(part_id).or_default().push(variation);
            }
        }

        let mut parts_by_program: HashMap<Uuid, Vec<Part>> = HashMap::new();
        for row in part_rows {
            let variations = variations_by_part.remove(&row.id).unwrap_or_default();
            let (program_id, part) = row.try_into_training_program_part(variations)?;
            parts_by_program.entry(program_id).or_default().push(part);
        }

        let programs = program_rows
            .into_iter()
            .map(|row| {
                let parts = parts_by_program.remove(&row.id).unwrap_or_default();
                row.try_into_training_program(parts)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((programs, total as u64))
    }
}

impl Transaction<TrainingProgram> for SqlxTrainingProgramTransaction {
    async fn save(&mut self, training_program: &TrainingProgram) -> Result<(), TransactionError> {
        let row = TrainingProgramRow::from_training_program(training_program);

        sqlx::query(
            r#"
            INSERT INTO training_program.training_program (id, author_id, name, description)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                author_id = EXCLUDED.author_id,
                name = EXCLUDED.name,
                description = EXCLUDED.description
            "#,
        )
        .bind(row.id)
        .bind(row.author_id)
        .bind(&row.name)
        .bind(&row.description)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM training_program.part
            WHERE training_program_id = $1
            "#,
        )
        .bind(row.id)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let mut part_rows = training_program
            .parts()
            .iter()
            .map(TrainingProgramPartRow::from_training_program_part)
            .collect::<Result<Vec<_>, _>>()?;
        part_rows.sort_by_key(|part_row| (part_row.position, part_row.id));

        for part_row in part_rows {
            if part_row.training_program_id != row.id {
                return Err(TransactionError::InvalidData(
                    "training program part does not belong to parent training program".to_string(),
                ));
            }

            sqlx::query(
                r#"
                INSERT INTO training_program.part
                    (id, training_program_id, position, name, description)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(part_row.id)
            .bind(part_row.training_program_id)
            .bind(part_row.position)
            .bind(&part_row.name)
            .bind(&part_row.description)
            .execute(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;
        }

        let mut part_variation_rows = training_program
            .parts()
            .iter()
            .flat_map(|part| {
                part.variations().iter().map(|variation| {
                    TrainingProgramPartVariationRow::from_part_variation(part.id(), variation)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        part_variation_rows
            .sort_by_key(|variation_row| (variation_row.part_id, variation_row.position));

        for variation_row in part_variation_rows {
            sqlx::query(
                r#"
                INSERT INTO training_program.part_variation (part_id, position, variation_id)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(variation_row.part_id)
            .bind(variation_row.position)
            .bind(variation_row.variation_id)
            .execute(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;
        }

        Ok(())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.tx.commit().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.tx.rollback().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }
}

impl TrainingProgramTransaction for SqlxTrainingProgramTransaction {
    async fn get_by_id(
        &mut self,
        id: TrainingProgramId,
    ) -> Result<Option<TrainingProgram>, TransactionError> {
        let row: Option<TrainingProgramRow> = sqlx::query_as(
            r#"
            SELECT id, author_id, name, description
            FROM training_program.training_program
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let part_rows: Vec<TrainingProgramPartRow> = sqlx::query_as(
            r#"
            SELECT id, training_program_id, position, name, description
            FROM training_program.part
            WHERE training_program_id = $1
            ORDER BY position ASC
            "#,
        )
        .bind(id.as_uuid())
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let part_ids: Vec<Uuid> = part_rows.iter().map(|part_row| part_row.id).collect();
        let mut variations_by_part: HashMap<Uuid, Vec<PartVariation>> = HashMap::new();

        if !part_ids.is_empty() {
            let variation_rows: Vec<TrainingProgramPartVariationRow> = sqlx::query_as(
                r#"
                SELECT part_id, position, variation_id
                FROM training_program.part_variation
                WHERE part_id = ANY($1)
                ORDER BY part_id ASC, position ASC
                "#,
            )
            .bind(&part_ids)
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

            for variation_row in variation_rows {
                let (part_id, part_variation) = variation_row.try_into_part_variation()?;
                variations_by_part.entry(part_id).or_default().push(part_variation);
            }
        }

        let parts = part_rows
            .into_iter()
            .map(|part_row| {
                let part_variations = variations_by_part.remove(&part_row.id).unwrap_or_default();
                part_row.try_into_training_program_part(part_variations)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some(row.try_into_training_program(parts)?))
    }

    async fn list(
        &mut self,
        pagination: Pagination,
    ) -> Result<(Vec<TrainingProgram>, u64), TransactionError> {
        self.list_internal(
            "SELECT COUNT(*) FROM training_program.training_program",
            r#"
            SELECT id, author_id, name, description
            FROM training_program.training_program
            ORDER BY name ASC, id ASC
            LIMIT $1 OFFSET $2
            "#,
            None,
            pagination,
        )
        .await
    }

    async fn list_by_author(
        &mut self,
        author_id: UserId,
        pagination: Pagination,
    ) -> Result<(Vec<TrainingProgram>, u64), TransactionError> {
        self.list_internal(
            "SELECT COUNT(*) FROM training_program.training_program WHERE author_id = $1",
            r#"
            SELECT id, author_id, name, description
            FROM training_program.training_program
            WHERE author_id = $1
            ORDER BY name ASC, id ASC
            LIMIT $2 OFFSET $3
            "#,
            Some(author_id),
            pagination,
        )
        .await
    }

    async fn delete_by_id(&mut self, id: TrainingProgramId) -> Result<(), TransactionError> {
        sqlx::query(
            r#"
            DELETE FROM training_program.training_program
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct SqlxTrainingProgramUnitOfWork {
    pool: PgPool,
}

impl SqlxTrainingProgramUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<TrainingProgram> for SqlxTrainingProgramUnitOfWork {
    type Transaction = SqlxTrainingProgramTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxTrainingProgramTransaction { tx })
    }
}
