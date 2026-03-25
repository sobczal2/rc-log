use std::collections::BTreeSet;

use rc_log_domain::maneuver::{difficulty::Difficulty, tag::Tag, Maneuver};
use rc_log_domain::shared::repository::{RepositoryError, Transaction, UnitOfWork};
use rc_log_domain::shared::{markdown_text::MarkdownText, vehicle_type::VehicleType, video_path::VideoPath};
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct ManeuverRow {
    id: Uuid,
    vehicle_type: String,
    name: String,
    description: String,
    difficulty: i32,
    video_path: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct TagRow {
    id: Uuid,
    name: String,
}

impl From<TagRow> for Tag {
    fn from(row: TagRow) -> Self {
        Tag::new(row.id, row.name)
    }
}

impl ManeuverRow {
    fn try_into_maneuver(self, tags: BTreeSet<Tag>) -> Option<Maneuver> {
        let vehicle_type = match self.vehicle_type.as_str() {
            "Helicopter" => VehicleType::Helicopter,
            "Plane" => VehicleType::Plane,
            "Drone" => VehicleType::Drone,
            _ => return None,
        };

        let difficulty = match self.difficulty {
            1 => Difficulty::Level1,
            2 => Difficulty::Level2,
            3 => Difficulty::Level3,
            4 => Difficulty::Level4,
            5 => Difficulty::Level5,
            6 => Difficulty::Level6,
            7 => Difficulty::Level7,
            _ => return None,
        };

        let video_path = self.video_path.map(VideoPath::new);

        Some(Maneuver::new(
            self.id,
            vehicle_type,
            self.name,
            tags,
            MarkdownText::new(self.description),
            difficulty,
            video_path,
        ))
    }

    fn from_maneuver(maneuver: &Maneuver) -> Self {
        let video_path = maneuver.video_path().map(|vp| vp.as_str().to_string());
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
            video_path,
        }
    }
}

pub struct SqlxManeuverTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Maneuver> for SqlxManeuverTransaction {
    async fn get_by_id(&mut self, id: Uuid) -> Option<Maneuver> {
        let maneuver_row: Option<ManeuverRow> = sqlx::query_as(
            r#"
            SELECT id, vehicle_type, name, description, difficulty, video_path
            FROM maneuver.maneuver
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *self.tx)
        .await
        .ok()?;

        let maneuver_row = maneuver_row?;

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
        .ok()?;

        let tags: BTreeSet<Tag> = tag_rows.into_iter().map(Tag::from).collect();

        maneuver_row.try_into_maneuver(tags)
    }

    async fn save(&mut self, maneuver: &Maneuver) -> Result<(), RepositoryError> {
        let maneuver_row = ManeuverRow::from_maneuver(maneuver);

        sqlx::query(
            r#"
            INSERT INTO maneuver.maneuver (id, vehicle_type, name, description, difficulty, video_path)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                vehicle_type = EXCLUDED.vehicle_type,
                name = EXCLUDED.name,
                description = EXCLUDED.description,
                difficulty = EXCLUDED.difficulty,
                video_path = EXCLUDED.video_path
            "#,
        )
        .bind(maneuver_row.id)
        .bind(&maneuver_row.vehicle_type)
        .bind(&maneuver_row.name)
        .bind(&maneuver_row.description)
        .bind(maneuver_row.difficulty)
        .bind(&maneuver_row.video_path)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;

        sqlx::query("DELETE FROM maneuver.maneuver_tag WHERE maneuver_id = $1")
            .bind(maneuver.id())
            .execute(&mut *self.tx)
            .await
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;

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
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;
        }

        Ok(())
    }

    async fn commit(self) -> Result<(), RepositoryError> {
        self.tx
            .commit()
            .await
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), RepositoryError> {
        self.tx
            .rollback()
            .await
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))
    }
}

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

    async fn begin(&mut self) -> Result<Self::Transaction, RepositoryError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;

        Ok(SqlxManeuverTransaction { tx })
    }
}
