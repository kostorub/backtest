use std::sync::{Arc, Mutex};

use crate::config::AppSettings;

#[derive(Debug)]
pub struct AppState {
    pub settings: Arc<AppSettings>,
}
