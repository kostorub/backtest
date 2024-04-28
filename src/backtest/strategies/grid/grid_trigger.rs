use crate::data_models::market_data::enums::Side;

#[derive(Debug, Clone)]
pub struct GridTrigger {
    pub price: f64,
    pub trigger_type: Side,
}

pub fn generate_trigger_prices(start: f64, end: f64, step: i64) -> Vec<f64> {
    if step <= 0 {
        return vec![];
    }
    (0..step + 1)
        .map(|i| start + (i as f64 * (end - start) / step as f64))
        .collect()
}

pub fn generate_grid_triggers(
    price_low: f64,
    price_high: f64,
    grids_count: i64,
    start_price: f64,
) -> Vec<GridTrigger> {
    generate_trigger_prices(price_low, price_high, grids_count)
        .iter()
        .map(|price| {
            let trigger_type = if *price >= start_price {
                Side::Sell
            } else {
                Side::Buy
            };

            GridTrigger {
                price: *price,
                trigger_type: trigger_type,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_trigger_prices() {
        assert_eq!(
            generate_trigger_prices(0.0, 10.0, 5),
            vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]
        );
        assert_eq!(
            generate_trigger_prices(0.0, 10.0, 10),
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6., 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(generate_trigger_prices(0.0, 10.0, 1), vec![0.0, 10.0]);
        let r: Vec<f64> = vec![];
        assert_eq!(generate_trigger_prices(0.0, 10.0, 0), r);
        #[rustfmt::skip]
        assert_eq!(
            generate_trigger_prices(27333.0, 30000.0, 10),
            vec![27333.0, 27599.7, 27866.4, 28133.1, 28399.8, 28666.5, 28933.2, 29199.9, 29466.6, 29733.3, 30000.0]
        );
    }

    #[test]
    fn test_generate_grid_triggers() {
        let triggers = generate_grid_triggers(0.0, 10.0, 5, 6.0);
        assert_eq!(triggers.len(), 6);
        assert_eq!(triggers[1].price, 2.0);
        assert_eq!(triggers[1].trigger_type, Side::Buy);
        assert_eq!(triggers[3].price, 6.0);
        assert_eq!(triggers[3].trigger_type, Side::Sell);
        assert_eq!(triggers[4].price, 8.0);
        assert_eq!(triggers[4].trigger_type, Side::Sell);
    }
}
