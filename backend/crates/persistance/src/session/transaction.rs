use std::collections::HashMap;

use chrono::NaiveDate;
use rc_log_domain::maneuver::variation::VariationId;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::session::Session;
use rc_log_domain::session::date::Date;
use rc_log_domain::session::id::SessionId;
use rc_log_domain::session::performed_variation::PerformedVariation;
use rc_log_domain::session::performed_variation::id::PerformedVariationId;
use rc_log_domain::session::rating::Rating;
use rc_log_domain::session::transaction::{
    SessionFilter, SessionSort, SessionSortField, SessionTransaction, SortDirection,
};
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct SessionRow {
    id: Uuid,
    user_id: Uuid,
    date: NaiveDate,
    model_id: Option<Uuid>,
    note: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PerformedVariationRow {
    id: Uuid,
    variation_id: Uuid,
    quality: i16,
    comfort: i16,
    repeatability: i16,
    note: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PerformedVariationRowWithSession {
    session_id: Uuid,
    id: Uuid,
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
            date: session.date().as_naive_date(),
            model_id: session.model_id().map(Uuid::from),
            note: session.note().map(|n| n.as_str().to_string()),
        }
    }

    fn try_into_session(
        self,
        performed_variations: Vec<PerformedVariation>,
    ) -> Result<Session, TransactionError> {
        let date = Date::new(self.date);

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
        Self {
            id: Uuid::from(performed_variation.id()),
            variation_id: Uuid::from(performed_variation.variation_id()),
            quality: performed_variation.quality().as_i16(),
            comfort: performed_variation.comfort().as_i16(),
            repeatability: performed_variation.repeatability().as_i16(),
            note: performed_variation.note().map(|n| n.as_str().to_string()),
        }
    }

    fn try_into_performed_variation(self) -> Result<PerformedVariation, TransactionError> {
        let quality =
            Rating::from_i16(self.quality).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let comfort =
            Rating::from_i16(self.comfort).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let repeatability = Rating::from_i16(self.repeatability)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        let note = self
            .note
            .map(|n| MarkdownText::new(n).map_err(|e| TransactionError::InvalidData(e.to_string())))
            .transpose()?;

        Ok(PerformedVariation::new(
            PerformedVariationId::new(self.id),
            VariationId::new(self.variation_id),
            quality,
            comfort,
            repeatability,
            note,
        ))
    }
}

impl PerformedVariationRowWithSession {
    fn try_into_performed_variation(self) -> Result<(Uuid, PerformedVariation), TransactionError> {
        let quality =
            Rating::from_i16(self.quality).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let comfort =
            Rating::from_i16(self.comfort).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let repeatability = Rating::from_i16(self.repeatability)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        let note = self
            .note
            .map(|n| MarkdownText::new(n).map_err(|e| TransactionError::InvalidData(e.to_string())))
            .transpose()?;

        let performed = PerformedVariation::new(
            PerformedVariationId::new(self.id),
            VariationId::new(self.variation_id),
            quality,
            comfort,
            repeatability,
            note,
        );
        Ok((self.session_id, performed))
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

        let mut performed_rows = session
            .performed_variations()
            .iter()
            .map(PerformedVariationRow::from_performed_variation)
            .collect::<Vec<_>>();
        performed_rows.sort_by_key(|row| row.id);

        for performed_row in performed_rows {
            sqlx::query(
                r#"
                INSERT INTO session.performed_variation
                    (id, session_id, variation_id, quality, comfort, repeatability, note)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(performed_row.id)
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
            SELECT id, user_id, date, model_id, note
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
            SELECT id, variation_id, quality, comfort, repeatability, note
            FROM session.performed_variation
            WHERE session_id = $1
            ORDER BY id ASC
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

    async fn list_by_owner(
        &mut self,
        owner_id: UserId,
        pagination: Pagination,
        filter: SessionFilter,
        sort: SessionSort,
    ) -> Result<(Vec<Session>, u64), TransactionError> {
        let mut count_query =
            QueryBuilder::<'_, Postgres>::new("SELECT COUNT(*) FROM session.session s");
        let mut select_query = QueryBuilder::<'_, Postgres>::new(
            "SELECT s.id, s.user_id, s.date, s.model_id, s.note FROM session.session s",
        );

        let apply_conditions = |q: &mut QueryBuilder<'_, Postgres>| {
            let mut has_where = false;
            let mut add_clause = |builder: &mut QueryBuilder<'_, Postgres>| {
                if !has_where {
                    builder.push(" WHERE ");
                    has_where = true;
                } else {
                    builder.push(" AND ");
                }
            };

            add_clause(q);
            q.push("s.user_id = ");
            q.push_bind(owner_id.as_uuid());

            if !filter.model_ids.is_empty() {
                add_clause(q);
                let model_ids: Vec<Uuid> = filter.model_ids.iter().map(|id| id.as_uuid()).collect();
                q.push("s.model_id = ANY(");
                q.push_bind(model_ids);
                q.push(")");
            }

            if !filter.maneuver_ids.is_empty() {
                add_clause(q);
                let maneuver_ids: Vec<Uuid> =
                    filter.maneuver_ids.iter().map(|id| id.as_uuid()).collect();
                q.push(
                    "EXISTS (SELECT 1 FROM session.performed_variation pv JOIN maneuver.variation v ON v.id = pv.variation_id WHERE pv.session_id = s.id AND v.maneuver_id = ANY(",
                );
                q.push_bind(maneuver_ids);
                q.push("))");
            }

            if let Some(search_query) = &filter.search_query {
                add_clause(q);
                q.push("(");
                q.push("EXISTS (SELECT 1 FROM model.model m WHERE m.id = s.model_id AND m.name ILIKE '%' || ");
                q.push_bind(search_query.clone());
                q.push(" || '%')");
                q.push(" OR ");
                q.push(
                    "EXISTS (SELECT 1 FROM session.performed_variation pv JOIN maneuver.variation v ON v.id = pv.variation_id JOIN maneuver.maneuver mn ON mn.id = v.maneuver_id WHERE pv.session_id = s.id AND (mn.name ILIKE '%' || ",
                );
                q.push_bind(search_query.clone());
                q.push(" || '%' OR v.name ILIKE '%' || ");
                q.push_bind(search_query.clone());
                q.push(" || '%')");
                q.push(")");
                q.push(")");
            }
        };

        apply_conditions(&mut count_query);
        apply_conditions(&mut select_query);

        let total: i64 = count_query
            .build_query_scalar()
            .fetch_one(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        select_query.push(" ORDER BY ");
        match sort.field {
            SessionSortField::Date => {
                select_query.push("s.date ");
            }
        };
        match sort.direction {
            SortDirection::Asc => {
                select_query.push("ASC");
            }
            SortDirection::Desc => {
                select_query.push("DESC");
            }
        };
        select_query.push(", s.id DESC");

        select_query.push(" LIMIT ");
        select_query.push_bind(pagination.limit() as i64);
        select_query.push(" OFFSET ");
        select_query.push_bind(pagination.offset() as i64);

        let session_rows: Vec<SessionRow> = select_query
            .build_query_as()
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        if session_rows.is_empty() {
            return Ok((vec![], total as u64));
        }

        let session_ids: Vec<Uuid> = session_rows.iter().map(|row| row.id).collect();

        let performed_rows: Vec<PerformedVariationRowWithSession> = sqlx::query_as(
            r#"
            SELECT session_id, id, variation_id, quality, comfort, repeatability, note
            FROM session.performed_variation
            WHERE session_id = ANY($1)
            ORDER BY session_id ASC, id ASC
            "#,
        )
        .bind(&session_ids)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let mut performed_by_session: HashMap<Uuid, Vec<PerformedVariation>> = HashMap::new();
        for row in performed_rows {
            let (session_id, performed) = row.try_into_performed_variation()?;
            performed_by_session.entry(session_id).or_default().push(performed);
        }

        let sessions = session_rows
            .into_iter()
            .map(|row| {
                let performed_variations = performed_by_session.remove(&row.id).unwrap_or_default();
                row.try_into_session(performed_variations)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((sessions, total as u64))
    }

    async fn delete_by_id(&mut self, id: SessionId) -> Result<(), TransactionError> {
        sqlx::query(
            r#"
            DELETE FROM session.session
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
