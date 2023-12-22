pub mod model;
pub mod query;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Sqlite;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .connect(connection_str)
                    .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{e}\n");
                eprintln!("If the database has not yet been created, run: \n   $ sqlx database setup\n");
                panic!("database connection error");
            }
        }
    }
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}
impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}

#[cfg(test)]
pub mod test {
    use sqlx::migrate::Migrator;
    use crate::data::*;
    use tokio::runtime::Handle;

    pub fn new_db(handle: &Handle) -> AppDatabase {
        use sqlx::migrate::Migrator;
        use std::path::Path;
        handle.block_on(async move {
            let db = Database::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
            let pool = db.get_pool();
            migrator.run(pool).await.unwrap();
            db
        })
    }
}