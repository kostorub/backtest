use std::path::Path;

use log::info;
use sqlx::{
    migrate::{Migrate, MigrateDatabase, Migrator},
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
    let pool = SqlitePool::connect_with(options).await.unwrap();
    pool
}

pub async fn migration(pool: &sqlx::Pool<Sqlite>, settings: &AppSettings) {
    info!("Running migrations...");
    let mut c = pool.acquire().await.unwrap();
    let last_migration = match c.list_applied_migrations().await {
        Ok(migrations) => {
            info!("There were some migrations applied: {:?}", migrations);
            migrations.last().map(|m| m.version).unwrap_or(0)
        }
        Err(_) => {
            info!("There were no migrations applied.");
            0
        }
    };

    if let Some(version) = &settings.database_migration_version {
        let version = version.parse::<i64>().unwrap();
        let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();

        info!(
            "Last migration version: {}, .env version: {}",
            last_migration, version
        );
        if last_migration > version {
            let migration = migrator.undo(pool, version).await;
            info!(
                "Last migration version > .env version. Migration undo result: {:?}",
                migration
            );
        } else if last_migration < version {
            let migration = migrator.run(pool).await;
            info!(
                "Last migration version <= .env version. Migration run result: {:?}",
                migration
            );
        } else {
            info!("Last migration version == .env version. No migrations needed.");
        }
    }
}
