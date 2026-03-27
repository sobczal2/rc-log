use rc_log_domain::user::{User, query::UserTransaction, username::Username};
use rc_log_domain::shared::email::Email;
use rc_log_domain::shared::transaction::{TransactionError, Transaction};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use rc_log_domain::shared::password_hash::PasswordHash;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    username: String,
    email: String,
    password_hash: String,
}

impl UserRow {
    fn try_into_user(self) -> Result<User, TransactionError> {
        let username = Username::new(self.username)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let email = Email::new(self.email)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let password_hash = PasswordHash::new(self.password_hash)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        Ok(User::new(self.id, username, email, password_hash))
    }

    fn from_user(user: &User) -> Self {
        Self {
            id: user.id(),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            password_hash: user.password_hash().as_str().to_string(),
        }
    }
}

pub struct SqlxUserTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<User> for SqlxUserTransaction {
    async fn get_by_id(&mut self, id: Uuid) -> Result<Option<User>, TransactionError> {
        let user_row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash
            FROM "user"."user"
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        user_row.map(UserRow::try_into_user).transpose()
    }

    async fn save(&mut self, user: &User) -> Result<(), TransactionError> {
        let user_row = UserRow::from_user(user);

        sqlx::query(
            r#"
            INSERT INTO "user"."user" (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash
            "#,
        )
        .bind(user_row.id)
        .bind(&user_row.username)
        .bind(&user_row.email)
        .bind(&user_row.password_hash)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.tx
            .commit()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.tx
            .rollback()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))
    }
}

impl UserTransaction for SqlxUserTransaction {
    async fn get_by_username(&mut self, username: &Username) -> Result<Option<User>, TransactionError> {
        let user_row: Option<UserRow> = sqlx::query_as(
            r#"
            SELECT id, username, email, password_hash
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
