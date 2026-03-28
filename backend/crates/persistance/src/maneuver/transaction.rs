use std::collections::{BTreeSet, HashMap};

use rc_log_domain::asset::name::AssetName;
use rc_log_domain::maneuver::Maneuver;
use rc_log_domain::maneuver::difficulty::Difficulty;
use rc_log_domain::maneuver::tag::Tag;
use rc_log_domain::maneuver::transaction::ManeuverTransaction;
use rc_log_domain::maneuver::variation::Variation;
use rc_log_domain::shared::markdown_text::MarkdownText;
use rc_log_domain::shared::pagination::Pagination;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::shared::vehicle_type::VehicleType;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ManeuverRow {
    id: Uuid,
    vehicle_type: String,
    name: String,
    description: String,
    difficulty: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct VariationRow {
    id: Uuid,
    maneuver_id: Uuid,
    name: String,
    description: String,
    video_asset_name: String,
    is_default: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TagRow {
    id: Uuid,
    name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TagRowWithManeuver {
    id: Uuid,
    name: String,
    maneuver_id: Uuid,
}

impl From<TagRow> for Tag {
    fn from(row: TagRow) -> Self {
        Tag::new(row.id, row.name)
    }
}

impl From<TagRowWithManeuver> for Tag {
    fn from(row: TagRowWithManeuver) -> Self {
        Tag::new(row.id, row.name)
    }
}

impl VariationRow {
    fn try_into_variation(self) -> Result<Variation, TransactionError> {
        let description = MarkdownText::new(self.description)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let video_asset_name = AssetName::new(self.video_asset_name)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        Ok(Variation::new(self.id, self.name, description, video_asset_name))
    }
}

impl ManeuverRow {
    fn try_into_maneuver(
        self,
        tags: BTreeSet<Tag>,
        default_variation: Variation,
        other_variations: Vec<Variation>,
    ) -> Result<Maneuver, TransactionError> {
        let vehicle_type = match self.vehicle_type.as_str() {
            "Helicopter" => VehicleType::Helicopter,
            "Plane" => VehicleType::Plane,
            "Drone" => VehicleType::Drone,
            other => {
                return Err(TransactionError::InvalidData(format!(
                    "Unknown vehicle_type: {}",
                    other
                )));
            }
        };

        let difficulty = match self.difficulty {
            1 => Difficulty::Level1,
            2 => Difficulty::Level2,
            3 => Difficulty::Level3,
            4 => Difficulty::Level4,
            5 => Difficulty::Level5,
            6 => Difficulty::Level6,
            7 => Difficulty::Level7,
            other => {
                return Err(TransactionError::InvalidData(format!("Unknown difficulty: {}", other)));
            }
        };

        let description = MarkdownText::new(self.description)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;

        Ok(Maneuver::new(
            self.id,
            vehicle_type,
            self.name,
            tags,
            description,
            difficulty,
            default_variation,
            other_variations,
        ))
    }

    fn from_maneuver(maneuver: &Maneuver) -> Self {
        let vehicle_type = match maneuver.vehicle_type() {
            VehicleType::Helicopter => "Helicopter".to_string(),
            VehicleType::Plane => "Plane".to_string(),
            VehicleType::Drone => "Drone".to_string(),
        };
        let difficulty = match maneuver.difficulty() {
            Difficulty::Level1 => 1,
            Difficulty::Level2 => 2,
            Difficulty::Level3 => 3,
            Difficulty::Level4 => 4,
            Difficulty::Level5 => 5,
            Difficulty::Level6 => 6,
            Difficulty::Level7 => 7,
        };

        Self {
            id: maneuver.id(),
            vehicle_type,
            name: maneuver.name().to_string(),
            description: maneuver.description().as_str().to_string(),
            difficulty,
        }
    }
}

pub struct SqlxManeuverTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Maneuver> for SqlxManeuverTransaction {
    async fn get_by_id(&mut self, id: Uuid) -> Result<Option<Maneuver>, TransactionError> {
        let maneuver_row: Option<ManeuverRow> = sqlx::query_as(
            r#"
            SELECT id, vehicle_type, name, description, difficulty
            FROM maneuver.maneuver
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let maneuver_row = match maneuver_row {
            None => return Ok(None),
            Some(r) => r,
        };

        let tag_rows: Vec<TagRow> = sqlx::query_as(
            r#"
            SELECT t.id, t.name
            FROM maneuver.tag t
            INNER JOIN maneuver.maneuver_tag mt ON t.id = mt.tag_id
            WHERE mt.maneuver_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let variation_rows: Vec<VariationRow> = sqlx::query_as(
            r#"
            SELECT id, maneuver_id, name, description, video_asset_name, is_default
            FROM maneuver.variation
            WHERE maneuver_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let tags: BTreeSet<Tag> = tag_rows.into_iter().map(Tag::from).collect();

        let mut default_variation: Option<Variation> = None;
        let mut other_variations: Vec<Variation> = Vec::new();
        for row in variation_rows {
            let is_default = row.is_default;
            let variation = row.try_into_variation()?;
            if is_default {
                default_variation = Some(variation);
            } else {
                other_variations.push(variation);
            }
        }

        let default_variation = default_variation.ok_or_else(|| {
            TransactionError::InvalidData(format!("Maneuver {} has no default variation", id))
        })?;

        Ok(Some(maneuver_row.try_into_maneuver(tags, default_variation, other_variations)?))
    }

    async fn save(&mut self, maneuver: &Maneuver) -> Result<(), TransactionError> {
        let maneuver_row = ManeuverRow::from_maneuver(maneuver);

        sqlx::query(
            r#"
            INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                vehicle_type = EXCLUDED.vehicle_type,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                difficulty = EXCLUDED.difficulty
            "#,
        )
        .bind(maneuver_row.id)
        .bind(&maneuver_row.vehicle_type)
        .bind(&maneuver_row.name)
        .bind(&maneuver_row.description)
        .bind(maneuver_row.difficulty)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        sqlx::query("DELETE FROM maneuver.maneuver_tag WHERE maneuver_id = $1")
            .bind(maneuver.id())
            .execute(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        for tag in maneuver.tags() {
            sqlx::query(
                r#"
                INSERT INTO maneuver.maneuver_tag (maneuver_id, tag_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
            )
            .bind(maneuver.id())
            .bind(tag.id())
            .execute(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;
        }

        sqlx::query("DELETE FROM maneuver.variation WHERE maneuver_id = $1")
            .bind(maneuver.id())
            .execute(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let default_var = maneuver.default_variation();
        sqlx::query(
            r#"
            INSERT INTO maneuver.variation (id, maneuver_id, name, description, video_asset_name, is_default)
            VALUES ($1, $2, $3, $4, $5, TRUE)
            "#,
        )
        .bind(default_var.id())
        .bind(maneuver.id())
        .bind(default_var.name())
        .bind(default_var.description().as_str())
        .bind(default_var.video_asset_name().as_str())
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        for var in maneuver.other_variations() {
            sqlx::query(
                r#"
                INSERT INTO maneuver.variation (id, maneuver_id, name, description, video_asset_name, is_default)
                VALUES ($1, $2, $3, $4, $5, FALSE)
                "#,
            )
            .bind(var.id())
            .bind(maneuver.id())
            .bind(var.name())
            .bind(var.description().as_str())
            .bind(var.video_asset_name().as_str())
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

impl SqlxManeuverTransaction {
    async fn list(
        &mut self,
        pagination: Pagination,
        filter: rc_log_domain::maneuver::transaction::ManeuverFilter,
        sort: rc_log_domain::maneuver::transaction::ManeuverSort,
    ) -> Result<(Vec<Maneuver>, u64), TransactionError> {
        let mut count_query =
            QueryBuilder::<'_, Postgres>::new("SELECT COUNT(*) FROM maneuver.maneuver m");
        let mut select_query = QueryBuilder::<'_, Postgres>::new(
            "SELECT m.id, m.vehicle_type, m.name, m.description, m.difficulty FROM maneuver.maneuver m",
        );

        let apply_conditions = |q: &mut QueryBuilder<'_, Postgres>| {
            let mut has_where = false;
            let mut add_clause = |b: &mut QueryBuilder<'_, Postgres>| {
                if !has_where {
                    b.push(" WHERE ");
                    has_where = true;
                } else {
                    b.push(" AND ");
                }
            };

            if let Some(vt) = &filter.vehicle_type {
                add_clause(q);
                let vt_str = match vt {
                    VehicleType::Helicopter => "Helicopter",
                    VehicleType::Plane => "Plane",
                    VehicleType::Drone => "Drone",
                };
                q.push("m.vehicle_type = ");
                q.push_bind(vt_str);
            }

            if let Some(diff) = &filter.difficulty {
                add_clause(q);
                let d_val = match diff {
                    Difficulty::Level1 => 1,
                    Difficulty::Level2 => 2,
                    Difficulty::Level3 => 3,
                    Difficulty::Level4 => 4,
                    Difficulty::Level5 => 5,
                    Difficulty::Level6 => 6,
                    Difficulty::Level7 => 7,
                };
                q.push("m.difficulty = ");
                q.push_bind(d_val);
            }

            if let Some(sq) = &filter.search_query {
                add_clause(q);
                q.push("(m.name ILIKE '%' || ");
                q.push_bind(sq.clone());
                q.push(" || '%' OR EXISTS (SELECT 1 FROM maneuver.maneuver_tag mt JOIN maneuver.tag t ON mt.tag_id = t.id WHERE mt.maneuver_id = m.id AND t.name ILIKE '%' || ");
                q.push_bind(sq.clone());
                q.push(" || '%'))");
            }

            if !filter.tags.is_empty() {
                add_clause(q);
                q.push("EXISTS (SELECT 1 FROM maneuver.maneuver_tag mt JOIN maneuver.tag t ON mt.tag_id = t.id WHERE mt.maneuver_id = m.id AND t.name = ANY(");
                q.push_bind(filter.tags.clone());
                q.push("))");
            }
        };

        apply_conditions(&mut count_query);
        apply_conditions(&mut select_query);

        let total: i64 = count_query
            .build_query_scalar()
            .fetch_one(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        use rc_log_domain::maneuver::transaction::{ManeuverSortField, SortDirection};

        select_query.push(" ORDER BY ");
        match sort.field {
            ManeuverSortField::Name => {
                select_query.push("m.name ");
            }
            ManeuverSortField::Difficulty => {
                select_query.push("m.difficulty ");
            }
        }
        match sort.direction {
            SortDirection::Asc => {
                select_query.push("ASC");
            }
            SortDirection::Desc => {
                select_query.push("DESC");
            }
        }

        select_query.push(" LIMIT ");
        select_query.push_bind(pagination.limit() as i64);
        select_query.push(" OFFSET ");
        select_query.push_bind(pagination.offset() as i64);

        let maneuver_rows: Vec<ManeuverRow> = select_query
            .build_query_as()
            .fetch_all(&mut *self.tx)
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        if maneuver_rows.is_empty() {
            return Ok((vec![], total as u64));
        }

        let maneuver_ids: Vec<Uuid> = maneuver_rows.iter().map(|r| r.id).collect();

        let tag_rows: Vec<TagRowWithManeuver> = sqlx::query_as(
            r#"
            SELECT t.id, t.name, mt.maneuver_id
            FROM maneuver.tag t
            INNER JOIN maneuver.maneuver_tag mt ON t.id = mt.tag_id
            WHERE mt.maneuver_id = ANY($1)
            "#,
        )
        .bind(&maneuver_ids)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let mut tags_by_maneuver: HashMap<Uuid, BTreeSet<Tag>> = HashMap::new();
        for row in tag_rows {
            tags_by_maneuver.entry(row.maneuver_id).or_default().insert(Tag::from(row));
        }

        let default_variation_rows: Vec<VariationRow> = sqlx::query_as(
            r#"
            SELECT id, maneuver_id, name, description, video_asset_name, is_default
            FROM maneuver.variation
            WHERE maneuver_id = ANY($1) AND is_default = TRUE
            "#,
        )
        .bind(&maneuver_ids)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        let mut default_vars_by_maneuver: HashMap<Uuid, Variation> = HashMap::new();
        for row in default_variation_rows {
            let maneuver_id = row.maneuver_id;
            let variation = row.try_into_variation()?;
            default_vars_by_maneuver.insert(maneuver_id, variation);
        }

        let maneuvers: Result<Vec<Maneuver>, TransactionError> = maneuver_rows
            .into_iter()
            .map(|row| {
                let id = row.id;
                let tags = tags_by_maneuver.remove(&id).unwrap_or_default();
                let default_variation = default_vars_by_maneuver.remove(&id).ok_or_else(|| {
                    TransactionError::InvalidData(format!(
                        "Maneuver {} has no default variation",
                        id
                    ))
                })?;
                row.try_into_maneuver(tags, default_variation, vec![])
            })
            .collect();

        Ok((maneuvers?, total as u64))
    }
}

impl ManeuverTransaction for SqlxManeuverTransaction {
    async fn list(
        &mut self,
        pagination: Pagination,
        filter: rc_log_domain::maneuver::transaction::ManeuverFilter,
        sort: rc_log_domain::maneuver::transaction::ManeuverSort,
    ) -> Result<(Vec<Maneuver>, u64), TransactionError> {
        // Delegate to the standalone impl
        <Self>::list(self, pagination, filter, sort).await
    }
}

#[derive(Clone)]
pub struct SqlxManeuverUnitOfWork {
    pool: PgPool,
}

impl SqlxManeuverUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<Maneuver> for SqlxManeuverUnitOfWork {
    type Transaction = SqlxManeuverTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxManeuverTransaction { tx })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use super::{ManeuverRow, VariationRow};

    fn make_variation_row(description: &str, asset_name: &str) -> VariationRow {
        VariationRow {
            id: Uuid::new_v4(),
            maneuver_id: Uuid::new_v4(),
            name: "default".to_string(),
            description: description.to_string(),
            video_asset_name: asset_name.to_string(),
            is_default: true,
        }
    }

    fn make_maneuver_row(vehicle_type: &str, difficulty: i32) -> ManeuverRow {
        ManeuverRow {
            id: Uuid::new_v4(),
            vehicle_type: vehicle_type.to_string(),
            name: "Test Maneuver".to_string(),
            description: "A valid description".to_string(),
            difficulty,
        }
    }

    // --- VariationRow ---

    #[test]
    fn variation_row_valid_converts_to_variation() {
        let row = make_variation_row("description text", "video_small");
        assert!(row.try_into_variation().is_ok());
    }

    #[test]
    fn variation_row_empty_description_fails() {
        let row = make_variation_row("", "video_small");
        assert!(row.try_into_variation().is_err());
    }

    #[test]
    fn variation_row_empty_asset_name_fails() {
        let row = make_variation_row("valid description", "");
        assert!(row.try_into_variation().is_err());
    }

    // --- ManeuverRow ---

    fn make_default_variation() -> rc_log_domain::maneuver::variation::Variation {
        use rc_log_domain::asset::name::AssetName;
        use rc_log_domain::shared::markdown_text::MarkdownText;
        rc_log_domain::maneuver::variation::Variation::new(
            Uuid::new_v4(),
            "default".to_string(),
            MarkdownText::new("desc".to_string()).unwrap(),
            AssetName::new("asset".to_string()).unwrap(),
        )
    }

    #[test]
    fn maneuver_row_helicopter_converts() {
        let row = make_maneuver_row("Helicopter", 1);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn maneuver_row_plane_converts() {
        let row = make_maneuver_row("Plane", 3);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn maneuver_row_drone_converts() {
        let row = make_maneuver_row("Drone", 7);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn maneuver_row_all_difficulties_convert() {
        for d in 1..=7i32 {
            let row = make_maneuver_row("Helicopter", d);
            let result =
                row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
            assert!(result.is_ok(), "difficulty {d} should convert");
        }
    }

    #[test]
    fn maneuver_row_unknown_vehicle_type_fails() {
        let row = make_maneuver_row("Submarine", 1);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn maneuver_row_difficulty_zero_fails() {
        let row = make_maneuver_row("Helicopter", 0);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn maneuver_row_difficulty_eight_fails() {
        let row = make_maneuver_row("Helicopter", 8);
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn maneuver_row_empty_description_fails() {
        let mut row = make_maneuver_row("Helicopter", 1);
        row.description = String::new();
        let result = row.try_into_maneuver(BTreeSet::new(), make_default_variation(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn maneuver_row_from_maneuver_round_trip() {
        use std::collections::BTreeSet;
        use rc_log_domain::maneuver::Maneuver;
        use rc_log_domain::maneuver::difficulty::Difficulty;
        use rc_log_domain::shared::markdown_text::MarkdownText;
        use rc_log_domain::shared::vehicle_type::VehicleType;

        let id = Uuid::new_v4();
        let maneuver = Maneuver::new(
            id,
            VehicleType::Plane,
            "Stall Turn".to_string(),
            BTreeSet::new(),
            MarkdownText::new("A description".to_string()).unwrap(),
            Difficulty::Level4,
            make_default_variation(),
            vec![],
        );
        let row = ManeuverRow::from_maneuver(&maneuver);
        assert_eq!(row.id, id);
        assert_eq!(row.vehicle_type, "Plane");
        assert_eq!(row.difficulty, 4);
        assert_eq!(row.name, "Stall Turn");
        assert_eq!(row.description, "A description");
    }
}
