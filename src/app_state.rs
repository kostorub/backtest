use std::sync::{Arc, Mutex};

use crate::config::Settings;

#[derive(Debug)]
pub struct AppState {
    pub settings: Arc<Settings>,
}
