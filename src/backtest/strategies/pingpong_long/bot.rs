use rand::{rngs::StdRng, SeedableRng};

use super::settings::PingPongLongSettings;

#[derive(Debug, Clone)]
pub struct PingPongLongBot {
    pub settings: PingPongLongSettings,
    pub rng: StdRng,
}

impl PingPongLongBot {
    pub fn new(settings: PingPongLongSettings) -> Self {
        let rng = match settings.random_seed {
            Some(seed) => StdRng::seed_from_u64(seed as u64),
            None => StdRng::from_entropy(),
        };

        Self { settings, rng }
    }
}
