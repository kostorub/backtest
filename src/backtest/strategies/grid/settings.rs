pub struct PingPongSettings {
    b1_step: f64,
    b2_step: f64,
    b3_step: f64,
    b4_step: f64,
}

impl PingPongSettings {
    pub fn new(b1_step: f64, b2_step: f64, b3_step: f64, b4_step: f64) -> Self {
        Self {
            b1_step,
            b2_step,
            b3_step,
            b4_step,
        }
    }
}
