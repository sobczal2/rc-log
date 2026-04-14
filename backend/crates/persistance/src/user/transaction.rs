use rc_log_domain::asset::photo::PhotoId;
use rc_log_domain::shared::email::Email;
use rc_log_domain::shared::password_hash::PasswordHash;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::user::{User, id::UserId, query::UserTransaction, username::Username};
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
    photo_asset_id: Option<Uuid>,
}

impl UserRow {
    fn try_into_user(self) -> Result<User, TransactionError> {
        let username = Username::new(self.username)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let email =
            Email::new(self.email).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let password_hash = PasswordHash::new(self.password_hash)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let photo_asset_id = self.photo_asset_id.map(PhotoId::new);
        Ok(User::new(UserId::new(self.id), username, email, password_hash, photo_asset_id))
    }

    fn from_user(user: &User) -> Self {
        Self {
            id: Uuid::from(user.id()),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            password_hash: user.password_hash().as_str().to_string(),
            photo_asset_id: user.photo_asset_id().map(|id| id.as_uuid()),
        }
    }
}

pub struct SqlxUserTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<User> for SqlxUserTransaction {
    async fn save(&mut self, user: &User) -> Result<(), TransactionError> {
        let user_row = UserRow::from_user(user);

        sqlx::query(
            r#"
            INSERT INTO "user"."user" (id, username, email, password_hash, photo_asset_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                photo_asset_id = EXCLUDED.photo_asset_id
            "#,
        )
        .bind(user_row.id)
        .bind(&user_row.username)
        .bind(&user_row.email)
        .bind(&user_row.password_hash)
        .bind(&user_row.photo_asset_id)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if let Some(constraint) = db_err.constraint() {
                    if constraint.contains("username") {
                        return TransactionError::InvalidData("unique_username".to_string());
                    }
                    if constraint.contains("email") {
                        return TransactionError::InvalidData("unique_email".to_string());
                    }
                }
            }
            TransactionError::TransactionError(e.to_string())
        })?;

        Ok(())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.tx.commit().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.tx.rollback().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }
}

impl UserTransaction for SqlxUserTransaction {
    async fn get_by_id(&mut self, id: UserId) -> Result<Option<User>, TransactionError> {
        let user_row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, photo_asset_id
            FROM "user"."user"
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        user_row.map(UserRow::try_into_user).transpose()
    }

    async fn get_by_username(
        &mut self,
        username: &Username,
    ) -> Result<Option<User>, TransactionError> {
        let user_row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, photo_asset_id
            FROM "user"."user"
            WHERE username = $1
            "#,
        )
        .bind(username.as_str())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        user_row.map(UserRow::try_into_user).transpose()
    }

    async fn get_by_email(&mut self, email: &Email) -> Result<Option<User>, TransactionError> {
        let user_row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash, photo_asset_id
            FROM "user"."user"
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        user_row.map(UserRow::try_into_user).transpose()
    }
}

#[derive(Clone)]
pub struct SqlxUserUnitOfWork {
    pool: PgPool,
}

impl SqlxUserUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<User> for SqlxUserUnitOfWork {
    type Transaction = SqlxUserTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxUserTransaction { tx })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::UserRow;

    fn make_user_row(username: &str, email: &str, password_hash: &str) -> UserRow {
        UserRow {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            photo_asset_id: None,
        }
    }

    #[test]
    fn valid_user_row_converts() {
        let row = make_user_row("alice", "alice@example.com", "$argon2id$hash");
        assert!(row.try_into_user().is_ok());
    }

    #[test]
    fn empty_username_fails() {
        let row = make_user_row("", "alice@example.com", "$argon2id$hash");
        assert!(row.try_into_user().is_err());
    }

    #[test]
    fn whitespace_only_username_fails() {
        let row = make_user_row("   ", "alice@example.com", "$argon2id$hash");
        assert!(row.try_into_user().is_err());
    }

    #[test]
    fn invalid_email_fails() {
        let row = make_user_row("alice", "not-an-email", "$argon2id$hash");
        assert!(row.try_into_user().is_err());
    }

    #[test]
    fn empty_password_hash_fails() {
        let row = make_user_row("alice", "alice@example.com", "");
        assert!(row.try_into_user().is_err());
    }

    #[test]
    fn from_user_preserves_all_fields() {
        use rc_log_domain::shared::email::Email;
        use rc_log_domain::shared::password_hash::PasswordHash;
        use rc_log_domain::user::User;
        use rc_log_domain::user::id::UserId;
        use rc_log_domain::user::username::Username;

        let id = Uuid::new_v4();
        let user = User::new(
            UserId::new(id),
            Username::new("bob".to_string()).unwrap(),
            Email::new("bob@example.com".to_string()).unwrap(),
            PasswordHash::new("hash123".to_string()).unwrap(),
            None,
        );
        let row = UserRow::from_user(&user);
        assert_eq!(row.id, id); // row.id is Uuid, id is Uuid — OK
        assert_eq!(row.username, "bob");
        assert_eq!(row.email, "bob@example.com");
        assert_eq!(row.password_hash, "hash123");
    }
}
