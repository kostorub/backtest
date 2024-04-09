use std::path::Path;

use log::info;
use sqlx::{migrate::MigrateDatabase, Sqlite};

use crate::config::AppSettings;

pub async fn init(settings: &AppSettings) {
    if settings.database_drop
        && Sqlite::database_exists(&settings.database_url)
            .await
            .unwrap_or(false)
    {
        info!("Database {} exists. Dropping...", &settings.database_url);
        Sqlite::drop_database(&settings.database_url).await.unwrap();
        info!("Database {} dropped.", &settings.database_url)
    }
    info!("Creating database {}.", &settings.database_url);
    match Sqlite::create_database(&settings.database_url).await {
        Ok(_) => info!("Database creation was successful!"),
        Err(error) => panic!("error: {}", error),
    }
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
