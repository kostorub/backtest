use super::settings::PingPongLongSettings;

#[derive(Debug, Clone)]
pub struct PingPongLongBot {
    pub settings: PingPongLongSettings,
}

impl PingPongLongBot {
    pub fn new(settings: PingPongLongSettings) -> Self {
        Self { settings }
    }
}
