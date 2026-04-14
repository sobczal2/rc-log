use rc_log_domain::asset::photo::PhotoId;
use rc_log_domain::model::Model;
use rc_log_domain::model::Type;
use rc_log_domain::model::id::ModelId;
use rc_log_domain::model::name::Name;
use rc_log_domain::model::transaction::ModelTransaction;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::id::UserId;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ModelRow {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    r#type: String,
    photo_asset_id: Option<Uuid>,
}

impl ModelRow {
    fn try_into_model(self) -> Result<Model, TransactionError> {
        let name =
            Name::new(self.name).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let r#type = match self.r#type.as_str() {
            "Helicopter" => Type::Helicopter,
            "Plane" => Type::Plane,
            "Drone" => Type::Drone,
            other => {
                return Err(TransactionError::InvalidData(format!(
                    "unknown model type: {other}"
                )));
            }
        };
        let photo_asset_id = self.photo_asset_id.map(PhotoId::new);
        Ok(Model::new(
            ModelId::new(self.id),
            UserId::new(self.owner_id),
            name,
            r#type,
            photo_asset_id,
        ))
    }

    fn from_model(model: &Model) -> Self {
        let r#type = match model.r#type() {
            Type::Helicopter => "Helicopter".to_string(),
            Type::Plane => "Plane".to_string(),
            Type::Drone => "Drone".to_string(),
        };
        Self {
            id: Uuid::from(model.id()),
            owner_id: Uuid::from(model.owner_id()),
            name: model.name().as_str().to_string(),
            r#type,
            photo_asset_id: model.photo_asset_id().map(|id| id.as_uuid()),
        }
    }
}

pub struct SqlxModelTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Model> for SqlxModelTransaction {
    async fn save(&mut self, model: &Model) -> Result<(), TransactionError> {
        let row = ModelRow::from_model(model);

        sqlx::query(
            r#"
            INSERT INTO model.model (id, owner_id, name, type, photo_asset_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                type = EXCLUDED.type,
                photo_asset_id = EXCLUDED.photo_asset_id
            "#,
        )
        .bind(row.id)
        .bind(row.owner_id)
        .bind(&row.name)
        .bind(&row.r#type)
        .bind(&row.photo_asset_id)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.tx.commit().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.tx.rollback().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }
}

impl ModelTransaction for SqlxModelTransaction {
    async fn get_by_id(&mut self, id: ModelId) -> Result<Option<Model>, TransactionError> {
        let row: Option<ModelRow> = sqlx::query_as(
            r#"
            SELECT id, owner_id, name, type, photo_asset_id
            FROM model.model
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        row.map(ModelRow::try_into_model).transpose()
    }

    async fn list_by_owner(
        &mut self,
        owner_id: UserId,
        pagination: Pagination,
    ) -> Result<(Vec<Model>, u64), TransactionError> {
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM model.model
            WHERE owner_id = $1
            "#,
        )
        .bind(owner_id.as_uuid())
        .fetch_one(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let rows: Vec<ModelRow> = sqlx::query_as(
            r#"
            SELECT id, owner_id, name, type, photo_asset_id
            FROM model.model
            WHERE owner_id = $1
            ORDER BY name ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(owner_id.as_uuid())
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let models =
            rows.into_iter().map(ModelRow::try_into_model).collect::<Result<Vec<_>, _>>()?;

        Ok((models, total as u64))
    }

    async fn delete_by_id(&mut self, id: ModelId) -> Result<(), TransactionError> {
        sqlx::query(
            r#"
            DELETE FROM model.model
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
pub struct SqlxModelUnitOfWork {
    pool: PgPool,
}

impl SqlxModelUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<Model> for SqlxModelUnitOfWork {
    type Transaction = SqlxModelTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxModelTransaction { tx })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::ModelRow;

    fn make_model_row(name: &str, r#type: &str) -> ModelRow {
        ModelRow {
            id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            name: name.to_string(),
            r#type: r#type.to_string(),
            photo_asset_id: None,
        }
    }

    #[test]
    fn valid_model_row_converts() {
        let row = make_model_row("My Trex 700", "Helicopter");
        assert!(row.try_into_model().is_ok());
    }

    #[test]
    fn empty_name_fails() {
        let row = make_model_row("", "Plane");
        assert!(row.try_into_model().is_err());
    }

    #[test]
    fn whitespace_only_name_fails() {
        let row = make_model_row("   ", "Drone");
        assert!(row.try_into_model().is_err());
    }

    #[test]
    fn unknown_type_fails() {
        let row = make_model_row("My Model", "Submarine");
        assert!(row.try_into_model().is_err());
    }

    #[test]
    fn all_types_convert() {
        for vt in ["Helicopter", "Plane", "Drone"] {
            let row = make_model_row("My Model", vt);
            assert!(row.try_into_model().is_ok(), "failed for {vt}");
        }
    }

    #[test]
    fn photo_asset_id_none_converts() {
        let row = make_model_row("My Model", "Drone");
        assert!(row.try_into_model().is_ok());
    }

    #[test]
    fn photo_asset_id_some_valid_converts() {
        let mut row = make_model_row("My Model", "Drone");
        row.photo_asset_id = Some(Uuid::new_v4());
        assert!(row.try_into_model().is_ok());
    }

    #[test]
    fn from_model_round_trip() {
        use rc_log_domain::model::Model;
        use rc_log_domain::model::Type;
        use rc_log_domain::model::id::ModelId;
        use rc_log_domain::model::name::Name;
        use rc_log_domain::user::id::UserId;

        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let model = Model::new(
            ModelId::new(id),
            UserId::new(owner_id),
            Name::new("My Trex 700".to_string()).unwrap(),
            Type::Helicopter,
            None,
        );
        let row = ModelRow::from_model(&model);
        assert_eq!(row.id, id);
        assert_eq!(row.owner_id, owner_id);
        assert_eq!(row.name, "My Trex 700");
        assert_eq!(row.r#type, "Helicopter");
        assert_eq!(row.photo_asset_id, None);
    }
}
