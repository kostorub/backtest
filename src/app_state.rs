use std::sync::Arc;

use handlebars::Handlebars;
use tera::Tera;

use crate::config::AppSettings;

pub struct AppState {
    pub app_settings: Arc<AppSettings>,
    pub tera: Arc<Tera>,
}
