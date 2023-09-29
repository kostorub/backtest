use std::sync::{Arc, Mutex};

use crate::config::AppSettings;

#[derive(Debug)]
pub struct AppState {
    pub app_settings: Arc<AppSettings>,
}
