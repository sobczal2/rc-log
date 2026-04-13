use rc_log_domain::maneuver::variation::VariationId;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::session::Session;
use rc_log_domain::session::date::Date;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::rating::{Comfort, Quality, Rating, Repeatability};
use rc_log_domain::session::transaction::SessionTransaction;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct SessionRow {
    id: Uuid,
    user_id: Uuid,
    date: String,
    model_id: Option<Uuid>,
    note: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PerformedVariationRow {
    variation_id: Uuid,
    quality: i16,
    comfort: i16,
    repeatability: i16,
    note: Option<String>,
}

impl SessionRow {
    fn from_session(session: &Session) -> Self {
        Self {
            id: Uuid::from(session.id()),
            user_id: Uuid::from(session.user_id()),
            date: session.date().as_naive_date().format("%Y-%m-%d").to_string(),
            model_id: session.model_id().map(Uuid::from),
            note: session.note().map(|n| n.as_str().to_string()),
        }
    }

    fn try_into_session(
        self,
        performed_variations: Vec<PerformedVariation>,
    ) -> Result<Session, TransactionError> {
        let date =
            Date::parse(&self.date).map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        let note = self
            .note
            .map(|n| MarkdownText::new(n).map_err(|e| TransactionError::InvalidData(e.to_string())))
            .transpose()?;

        Ok(Session::new(
            SessionId::new(self.id),
            UserId::new(self.user_id),
            date,
            self.model_id.map(ModelId::new),
            note,
            performed_variations,
        ))
    }
}

impl PerformedVariationRow {
    fn from_performed_variation(performed_variation: &PerformedVariation) -> Self {
        let rating = performed_variation.rating();
        Self {
            variation_id: Uuid::from(performed_variation.variation_id()),
            quality: rating.quality().as_i16(),
            comfort: rating.comfort().as_i16(),
            repeatability: rating.repeatability().as_i16(),
            note: performed_variation.note().map(|n| n.as_str().to_string()),
        }
    }

    fn try_into_performed_variation(self) -> Result<PerformedVariation, TransactionError> {
        let quality = Quality::from_i16(self.quality)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let comfort = Comfort::from_i16(self.comfort)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let repeatability = Repeatability::from_i16(self.repeatability)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        let rating = Rating::new(quality, comfort, repeatability);

        let note = self
            .note
            .map(|n| MarkdownText::new(n).map_err(|e| TransactionError::InvalidData(e.to_string())))
            .transpose()?;

        Ok(PerformedVariation::new(VariationId::new(self.variation_id), rating, note))
    }
}

pub struct SqlxSessionTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Session> for SqlxSessionTransaction {
    async fn save(&mut self, session: &Session) -> Result<(), TransactionError> {
        let row = SessionRow::from_session(session);

        sqlx::query(
            r#"
            INSERT INTO session.session (id, user_id, date, model_id, note)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                user_id = EXCLUDED.user_id,
                date = EXCLUDED.date,
                model_id = EXCLUDED.model_id,
                note = EXCLUDED.note
            "#,
        )
        .bind(row.id)
        .bind(row.user_id)
        .bind(&row.date)
        .bind(row.model_id)
        .bind(&row.note)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM session.performed_variation
            WHERE session_id = $1
            "#,
        )
        .bind(row.id)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        for performed_variation in session.performed_variations() {
            let performed_row = PerformedVariationRow::from_performed_variation(performed_variation);

            sqlx::query(
                r#"
                INSERT INTO session.performed_variation
                    (session_id, variation_id, quality, comfort, repeatability, note)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(row.id)
            .bind(performed_row.variation_id)
            .bind(performed_row.quality)
            .bind(performed_row.comfort)
            .bind(performed_row.repeatability)
            .bind(&performed_row.note)
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

impl SessionTransaction for SqlxSessionTransaction {
    async fn get_by_id(&mut self, id: SessionId) -> Result<Option<Session>, TransactionError> {
        let row: Option<SessionRow> = sqlx::query_as(
            r#"
            SELECT id, user_id, date::text AS date, model_id, note
            FROM session.session
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

        let performed_rows: Vec<PerformedVariationRow> = sqlx::query_as(
            r#"
            SELECT variation_id, quality, comfort, repeatability, note
            FROM session.performed_variation
            WHERE session_id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let performed_variations = performed_rows
            .into_iter()
            .map(PerformedVariationRow::try_into_performed_variation)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some(row.try_into_session(performed_variations)?))
    }
}

#[derive(Clone)]
pub struct SqlxSessionUnitOfWork {
    pool: PgPool,
}

impl SqlxSessionUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<Session> for SqlxSessionUnitOfWork {
    type Transaction = SqlxSessionTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxSessionTransaction { tx })
    }
}