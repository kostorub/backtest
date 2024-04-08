use std::sync::Arc;

use tera::Tera;

use crate::config::AppSettings;

pub struct AppState {
    pub app_settings: Arc<AppSettings>,
    pub tera: Arc<Tera>,
    pub pool: Arc<sqlx::sqlite::SqlitePool>,
}
