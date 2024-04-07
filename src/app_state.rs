use std::sync::Arc;

use tera::Tera;

use crate::config::AppSettings;

pub struct AppState {
    pub app_settings: Arc<AppSettings>,
    pub tera: Arc<Tera>,
    pub pool: Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
}
