use std::collections::HashSet;

use serde::Serialize;
use statistical::standard_deviation;

use super::position::Position;

#[derive(Default, Debug, Serialize)]
pub struct Metrics {
    positions_number: u64,
    profit_positions_number: u64,
    profit_positions_percent: f64,
    loss_positions_number: u64,
    loss_positions_percent: f64,
    average_profit_position: f64,
    average_loss_position: f64,
    number_of_currency: u32,
    profit_per_position_in_percent: f64,
    profit_factor: f64,
    expected_payoff: f64,
    sortino: f64,
    average_position_size: f64,
    start_deposit: f64,
    finish_deposit: f64,
    total_profit: f64,
    total_profit_percent: f64,
    max_deposit: f64,
    max_drawdown: f64,
    drawdown: f64,
    max_use_of_funds: f64,
}

impl Metrics {
    pub fn new(positions: &Vec<Position>, start_deposit: f64, finish_deposit: f64) -> Self {
        let positions_number = positions.len() as u64;
        if positions_number == 0 {
            return Metrics::default();
        }

        let profit_positions = Self::get_profit_positions(positions);

        let loss_positions = Self::get_loss_positions(positions);

        let profit_positions_number = profit_positions.len();
        let loss_positions_number = loss_positions.len();

        let profit_positions_percent =
            Self::get_profit_positions_percent(profit_positions_number, loss_positions_number);
        let loss_positions_percent =
            Self::get_loss_positions_percent(profit_positions_number, loss_positions_number);

        let average_profit_position =
            Self::get_average_position(profit_positions_number, &profit_positions);
        let average_loss_position =
            Self::get_average_position(loss_positions_number, &loss_positions);

        let number_of_currency = Self::get_number_of_currency(positions);

        let profit_per_position_in_percent = Self::get_profit_per_position_in_percent(
            positions,
            profit_positions_number,
            average_profit_position,
        );

        let profit_factor = Self::get_profit_factor(&profit_positions, &loss_positions);
        let expected_payoff = Self::get_expected_payoff(
            profit_positions_percent,
            average_profit_position,
            loss_positions_percent,
            average_loss_position,
        );

        let sortino = Self::get_sortino(positions);

        let average_position_size = Self::get_average_position_size(positions);
        let total_profit = Self::get_total_profit(finish_deposit, start_deposit);
        let total_profit_percent = Self::get_total_profit_percent(finish_deposit, start_deposit);
        let max_deposit = 0.0;
        let max_drawdown = 0.0;
        let drawdown = 0.0;
        let max_use_of_funds = Self::get_max_use_of_funds(positions);

        Metrics {
            positions_number,
            profit_positions_number: profit_positions_number as u64,
            profit_positions_percent,
            loss_positions_number: loss_positions_number as u64,
            loss_positions_percent,
            average_profit_position,
            average_loss_position,
            number_of_currency: number_of_currency as u32,
            profit_per_position_in_percent,
            profit_factor,
            expected_payoff,
            sortino,
            average_position_size,
            start_deposit,
            finish_deposit,
            total_profit,
            total_profit_percent,
            max_deposit,
            max_drawdown,
            drawdown,
            max_use_of_funds,
        }
    }

    fn get_profit_positions(positions: &Vec<Position>) -> Vec<&Position> {
        positions
            .iter()
            .filter(|position| position.pnl.unwrap() > 0.0)
            .collect()
    }

    fn get_loss_positions(positions: &Vec<Position>) -> Vec<&Position> {
        positions
            .iter()
            .filter(|position| position.pnl.unwrap() < 0.0)
            .collect()
    }

    fn get_profit_positions_percent(
        profit_positions_number: usize,
        loss_positions_number: usize,
    ) -> f64 {
        (profit_positions_number as f64 / (profit_positions_number + loss_positions_number) as f64)
            * 100.0
    }

    fn get_loss_positions_percent(
        profit_positions_number: usize,
        loss_positions_number: usize,
    ) -> f64 {
        (loss_positions_number as f64 / (profit_positions_number + loss_positions_number) as f64)
            * 100.0
    }

    fn get_average_position(positions_number: usize, positions: &Vec<&Position>) -> f64 {
        if positions_number != 0 {
            positions.iter().map(|t| t.pnl.unwrap()).sum::<f64>() / positions_number as f64
        } else {
            0.0
        }
    }

    fn get_number_of_currency(positions: &Vec<Position>) -> u32 {
        positions
            .iter()
            .map(|position| position.symbol.clone())
            .collect::<HashSet<_>>()
            .len() as u32
    }

    fn get_profit_per_position_in_percent(
        positions: &Vec<Position>,
        profit_positions_number: usize,
        average_profit_position: f64,
    ) -> f64 {
        if profit_positions_number != 0 && average_profit_position != 0.0 {
            positions
                .iter()
                .map(|p| p.volume_buy() * p.weighted_avg_price_buy())
                .sum::<f64>()
                / profit_positions_number as f64
                / average_profit_position
        } else {
            0.0
        }
    }

    fn get_profit_factor(
        profit_positions: &Vec<&Position>,
        loss_positions: &Vec<&Position>,
    ) -> f64 {
        (profit_positions.iter().map(|t| t.pnl.unwrap()).sum::<f64>()
            / loss_positions.iter().map(|t| t.pnl.unwrap()).sum::<f64>())
        .abs()
    }

    fn get_expected_payoff(
        profit_positions_percent: f64,
        average_profit_position: f64,
        loss_positions_percent: f64,
        average_loss_position: f64,
    ) -> f64 {
        profit_positions_percent * average_profit_position
            + loss_positions_percent * average_loss_position
    }

    fn get_sortino(positions: &Vec<Position>) -> f64 {
        let mut stddev = 0.0;
        let s: Vec<f64> = positions
            .iter()
            .filter(|t| t.pnl.unwrap() < 0.0)
            .map(|t| t.pnl.unwrap())
            .collect();
        if s.len() >= 2 {
            stddev = standard_deviation(&s, None);
        }
        if stddev != 0.0 {
            (positions.iter().map(|t| t.pnl.unwrap()).sum::<f64>() / positions.len() as f64)
                / stddev
                * 252_f64.sqrt()
        } else {
            0.0
        }
    }

    fn get_average_position_size(positions: &Vec<Position>) -> f64 {
        positions
            .iter()
            .map(|p| p.volume_buy() * p.weighted_avg_price_buy())
            .sum::<f64>()
            / positions.len() as f64
    }

    fn get_total_profit(finish_deposit: f64, start_deposit: f64) -> f64 {
        finish_deposit - start_deposit
    }

    fn get_total_profit_percent(finish_deposit: f64, start_deposit: f64) -> f64 {
        (finish_deposit - start_deposit) / start_deposit as f64 * 100.0
    }

    fn get_max_use_of_funds(positions: &Vec<Position>) -> f64 {
        positions
            .iter()
            .map(|p| p.volume_buy() * p.weighted_avg_price_buy())
            .reduce(f64::max)
            .unwrap()
    }
}

#[cfg(test)]
mod test {

    use crate::data_models::market_data::{
        enums::{OrderType, Side},
        order::Order,
        position::PositionStatus,
    };

    use super::*;

    #[rustfmt::skip]
    fn get_positions_info() -> Vec<Position> {
        vec![
            Position {
                symbol: "USD_BTC".into(),
                pnl: Some(20.0),
                status: PositionStatus::Closed,
                orders: vec![
                    Order::new(1502942400, 100.0, Side::Buy, OrderType::Market).with_qty(1.0),
                    Order::new(1502942400 + 3600, 120.0, Side::Sell, OrderType::Market).with_qty(1.0),
                ],
            },
            Position {
                symbol: "USD_ETH".into(),
                pnl: Some(20.0),
                status: PositionStatus::Closed,
                orders: vec![
                    Order::new(1502942400, 100., Side::Buy, OrderType::Market).with_qty(1.0),
                    Order::new(1502942400 + 3600, 120.0, Side::Sell, OrderType::Market).with_qty(1.0),
                ],
            },
            Position {
                symbol: "USD_ETH".into(),
                pnl: Some(-20.0),
                status: PositionStatus::Closed,
                orders: vec![
                    Order::new(1502942400, 100.0, Side::Buy, OrderType::Market).with_qty(1.0),
                    Order::new(1502942400 + 3600, 80.0, Side::Sell, OrderType::Market).with_qty(1.0),
                ],
            },
            Position {
                symbol: "USD_ETH".into(),
                pnl: Some(-40.0),
                status: PositionStatus::Closed,
                orders: vec![
                    Order::new(1502942400, 100.0, Side::Buy, OrderType::Market).with_qty(1.0),
                    Order::new(1502942400 + 3600, 60.0, Side::Sell, OrderType::Market).with_qty(1.0),
                ],            },
        ]
    }

    #[test]
    fn test_get_profit_positions() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_profit_positions(&positions).len(), 2);
    }

    #[test]
    fn test_get_loss_positions() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_loss_positions(&positions).len(), 2);
    }

    #[test]
    fn test_get_profit_positions_percent() {
        assert_eq!(Metrics::get_profit_positions_percent(2, 3), 40.0);
    }

    #[test]
    fn test_get_loss_positions_percent() {
        assert_eq!(Metrics::get_loss_positions_percent(2, 3), 60.0);
    }

    #[test]
    fn test_get_average_position() {
        let positions = get_positions_info();
        let profit_positions = Metrics::get_profit_positions(&positions);
        assert_eq!(
            Metrics::get_average_position(profit_positions.len(), &profit_positions),
            20.0
        );
        assert_eq!(Metrics::get_average_position(0, &profit_positions), 0.0);
    }

    #[test]
    fn test_get_number_of_currency() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_number_of_currency(&positions), 2);
    }

    #[test]
    fn test_get_profit_per_position_in_percent() {
        let positions = get_positions_info();
        let profit_positions = Metrics::get_profit_positions(&positions);
        let average_profit_position =
            Metrics::get_average_position(profit_positions.len(), &profit_positions);
        assert_eq!(
            Metrics::get_profit_per_position_in_percent(
                &positions,
                profit_positions.len(),
                average_profit_position
            ),
            10.0
        );
        assert_eq!(
            Metrics::get_profit_per_position_in_percent(&positions, 0, average_profit_position),
            0.0
        );
        assert_eq!(
            Metrics::get_profit_per_position_in_percent(&positions, profit_positions.len(), 0.0),
            0.0
        );
        assert_eq!(
            Metrics::get_profit_per_position_in_percent(&positions, 0, 0.0),
            0.0
        );
    }

    #[test]
    fn test_get_profit_factor() {
        let positions = get_positions_info();
        let profit_positions = Metrics::get_profit_positions(&positions);
        let loss_positions = Metrics::get_loss_positions(&positions);
        assert_eq!(
            Metrics::get_profit_factor(&profit_positions, &loss_positions),
            0.6666666666666666
        );
    }

    #[test]
    fn test_get_expected_payoff() {
        assert_eq!(Metrics::get_expected_payoff(2.0, 2.0, 2.0, 2.0), 8.0);
        assert_eq!(Metrics::get_expected_payoff(0.0, 0.0, 0.0, 0.0), 0.0);
    }

    #[test]
    fn test_get_sortino() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_sortino(&positions), -5.612486080160912);
    }

    #[test]
    fn test_get_average_position_size() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_average_position_size(&positions), 100.0);
    }

    #[test]
    fn test_get_total_profit() {
        assert_eq!(Metrics::get_total_profit(200.0, 100.0), 100.0);
    }

    #[test]
    fn test_get_total_profit_percent() {
        assert_eq!(Metrics::get_total_profit(200.0, 100.0), 100.0);
    }

    #[test]
    fn test_get_max_use_of_funds() {
        let positions = get_positions_info();
        assert_eq!(Metrics::get_max_use_of_funds(&positions), 100.0);
    }
}
