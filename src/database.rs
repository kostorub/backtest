use std::path::Path;

use log::info;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Sqlite, SqlitePool,
};

use crate::config::AppSettings;

pub async fn drop(settings: &AppSettings) {
    if settings.database_drop
        && Sqlite::database_exists(&settings.database_url)
            .await
            .unwrap_or(false)
    {
        info!("Database {} exists. Dropping...", &settings.database_url);
        Sqlite::drop_database(&settings.database_url).await.unwrap();
        info!("Database {} dropped.", &settings.database_url)
    }
}

pub async fn init(settings: &AppSettings) -> sqlx::Pool<sqlx::Sqlite> {
    let options = SqliteConnectOptions::new()
        .filename(&settings.database_url.split(":").last().unwrap())
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options)
        .await
        .unwrap();
    pool
}

pub async fn migration(pool: &sqlx::Pool<Sqlite>) {
    let migrations = Path::new("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(pool)
        .await;
    match migration_results {
        Ok(_) => info!("Migration process was successful!"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    info!("Migration result: {:?}", migration_results);
}
