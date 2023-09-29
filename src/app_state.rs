use std::sync::Arc;

use crate::config::AppSettings;

#[derive(Debug)]
pub struct AppState {
    pub app_settings: Arc<AppSettings>,
}
