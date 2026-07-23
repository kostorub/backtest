use rand::Rng;
use serde::Deserialize;

use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};

#[derive(Debug, Clone, Deserialize)]
pub struct PercentRange {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

impl PercentRange {
    pub fn validate(&self) -> Result<(), String> {
        if self.min > self.max {
            return Err("percent range min must be <= max".to_string());
        }
        if self.step <= 0.0 {
            return Err("percent range step must be > 0".to_string());
        }
        if self.min < 0.0 || self.max < 0.0 {
            return Err("percent range values must be non-negative".to_string());
        }
        Ok(())
    }

    pub fn discrete_values(&self) -> Vec<f64> {
        if self.min > self.max || self.step <= 0.0 || self.min < 0.0 || self.max < 0.0 {
            return vec![];
        }

        let epsilon = self.step.abs() * 1e-12;
        let span = self.max - self.min;
        let steps = ((span + epsilon) / self.step).floor() as usize;

        (0..=steps)
            .map(|index| self.min + index as f64 * self.step)
            .filter(|value| *value <= self.max + epsilon)
            .collect()
    }
}

pub fn choose_from_range(range: &PercentRange, rng: &mut impl Rng) -> f64 {
    let values = range.discrete_values();
    if values.is_empty() {
        return f64::NAN;
    }

    let index = rng.gen_range(0..values.len());
    values[index]
}

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
    pub order_size: f64,
    pub min_profit_percent: f64,
    pub bonus_enabled: bool,
    pub random_seed: Option<i64>,
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
        order_size: f64,
        min_profit_percent: f64,
        bonus_enabled: bool,
        random_seed: Option<i64>,
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
            order_size,
            min_profit_percent,
            bonus_enabled,
            random_seed,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        PercentRange {
            min: self.first_movement_min,
            max: self.first_movement_max,
            step: self.first_movement_step,
        }
        .validate()?;
        PercentRange {
            min: self.second_movement_min,
            max: self.second_movement_max,
            step: self.second_movement_step,
        }
        .validate()?;
        PercentRange {
            min: self.third_movement_min,
            max: self.third_movement_max,
            step: self.third_movement_step,
        }
        .validate()?;
        PercentRange {
            min: self.close_min,
            max: self.close_max,
            step: self.close_step,
        }
        .validate()?;

        if self.order_size <= 0.0 {
            return Err("order_size must be > 0".to_string());
        }
        if self.min_profit_percent < 0.0 {
            return Err("min_profit_percent must be >= 0".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PingPongLongSettingsRequest {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub first_movement_min: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub first_movement_max: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub first_movement_step: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub second_movement_min: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub second_movement_max: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub second_movement_step: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub third_movement_min: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub third_movement_max: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub third_movement_step: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub close_min: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub close_max: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub close_step: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub order_size: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub min_profit_percent: f64,
    #[serde(default)]
    pub bonus_enabled: bool,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub random_seed: Option<i64>,
}

impl PingPongLongSettingsRequest {
    pub fn into_settings(self) -> PingPongLongSettings {
        PingPongLongSettings::new(
            self.first_movement_min,
            self.first_movement_max,
            self.first_movement_step,
            self.second_movement_min,
            self.second_movement_max,
            self.second_movement_step,
            self.third_movement_min,
            self.third_movement_max,
            self.third_movement_step,
            self.close_min,
            self.close_max,
            self.close_step,
            self.order_size,
            self.min_profit_percent,
            self.bonus_enabled,
            self.random_seed,
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        self.clone().into_settings().validate()
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn test_choose_from_range_whole_number_step() {
        let range = PercentRange {
            min: 10.0,
            max: 20.0,
            step: 1.0,
        };
        let mut rng = StdRng::seed_from_u64(7);

        let value = choose_from_range(&range, &mut rng);

        assert!(range.discrete_values().contains(&value));
        assert!((value - value.round()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_choose_from_range_decimal_step() {
        let range = PercentRange {
            min: 10.0,
            max: 20.0,
            step: 0.5,
        };
        let mut rng = StdRng::seed_from_u64(11);

        let value = choose_from_range(&range, &mut rng);

        assert!(range.discrete_values().contains(&value));
        let scaled = (value - range.min) / range.step;
        assert!((scaled - scaled.round()).abs() < 1e-10);
    }

    #[test]
    fn test_choose_from_range_min_eq_max() {
        let range = PercentRange {
            min: 12.5,
            max: 12.5,
            step: 0.25,
        };
        let mut rng = StdRng::seed_from_u64(3);

        let value = choose_from_range(&range, &mut rng);

        assert_eq!(value, 12.5);
    }

    #[test]
    fn test_validate_rejects_invalid_step() {
        let range = PercentRange {
            min: 10.0,
            max: 20.0,
            step: 0.0,
        };

        assert_eq!(
            range.validate(),
            Err("percent range step must be > 0".to_string())
        );
    }

    #[test]
    fn test_selected_value_always_belongs_to_discrete_set() {
        let range = PercentRange {
            min: 2.0,
            max: 5.0,
            step: 0.5,
        };
        let allowed = range.discrete_values();
        let mut rng = StdRng::seed_from_u64(99);

        for _ in 0..100 {
            let value = choose_from_range(&range, &mut rng);
            assert!(allowed.contains(&value));
        }
    }
}
