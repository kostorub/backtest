use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PingPongLongSettings {
    pub first_movement_min: f64,
    pub first_movement_max: f64,
    pub first_movement_step: f64,
    pub second_movement_min: f64,
    pub second_movement_max: f64,
    pub second_movement_step: f64,
    pub third_movement_min: f64,
    pub third_movement_max: f64,
    pub third_movement_step: f64,
    pub close_min: f64,
    pub close_max: f64,
    pub close_step: f64,
}

impl PingPongLongSettings {
    #[allow(dead_code)]
    pub fn new(
        first_movement_min: f64,
        first_movement_max: f64,
        first_movement_step: f64,
        second_movement_min: f64,
        second_movement_max: f64,
        second_movement_step: f64,
        third_movement_min: f64,
        third_movement_max: f64,
        third_movement_step: f64,
        close_min: f64,
        close_max: f64,
        close_step: f64,
    ) -> Self {
        Self {
            first_movement_min,
            first_movement_max,
            first_movement_step,
            second_movement_min,
            second_movement_max,
            second_movement_step,
            third_movement_min,
            third_movement_max,
            third_movement_step,
            close_min,
            close_max,
            close_step,
        }
    }
}
