use std::path::PathBuf;

use log::info;

use crate::{
    data_handlers::bin_files::{bin_file_name, get_values_from_file},
    data_models::market_data::{
        enums::{MarketDataType, OrderStatus, OrderType, Side},
        kline::KLine,
        position::{Position, PositionStatus},
    },
};

pub fn get_klines(
    data_path: PathBuf,
    exchange: String,
    symbol: String,
    market_data_type: MarketDataType,
    date_start: u64,
    date_end: u64,
) -> Vec<KLine> {
    let file_path = PathBuf::from(data_path.clone()).join(bin_file_name(
        exchange.clone(),
        symbol.clone(),
        market_data_type,
    ));
    info!("Loading data from file: {:?}", file_path);
    get_values_from_file::<KLine>(file_path, date_start, date_end).unwrap()
}

pub fn check_tp_sl(kline: &KLine, positions_opened: &mut Vec<Position>, commission: f64) {
    for pos in positions_opened.iter_mut() {
        for order in pos.orders.iter_mut() {
            if order.status == OrderStatus::New {
                if (order.side == Side::Sell
                    && order.order_type == OrderType::TakeProfitMarket
                    && kline.close >= order.price)
                    || (order.side == Side::Sell
                        && order.order_type == OrderType::StopMarket
                        && kline.close <= order.price)
                {
                    let qty = order.qty.unwrap();
                    order
                        .update(kline.date)
                        .set_executed_price(kline.close)
                        .set_qty(qty)
                        .set_commission(kline.close, qty, commission)
                        .fill();
                }
            }
        }
    }
}

pub fn remove_closed_positions(positions_opened: &mut Vec<Position>) -> Vec<Position> {
    let mut result = Vec::new();
    {
        // It's a replacement for drain_filter, which is still a nightly-only experimental API
        let mut i = 0;
        while i < positions_opened.len() {
            if positions_opened[i].volume_all() == 0.0 {
                result.push(positions_opened.remove(i));
                result.last_mut().unwrap().status = PositionStatus::Closed;
            } else {
                i += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::data_models::market_data::order::Order;

    use super::*;

    fn get_positions_opened() -> Vec<Position> {
        vec![
            Position::new("BTCUSDT".to_string())
                .with_order(
                    Order::new(1, 100.0, Side::Buy, OrderType::Market)
                        .updated(1)
                        .with_price_executed(100.0)
                        .with_qty(1.0)
                        .with_commission(100.0, 1.0, 0.0)
                        .filled(),
                )
                .with_order(
                    Order::new(2, 200.0, Side::Sell, OrderType::TakeProfitMarket).with_qty(1.0),
                ),
            Position::new("BTCUSDT".to_string())
                .with_order(
                    Order::new(3, 100.0, Side::Buy, OrderType::Market)
                        .updated(3)
                        .with_price_executed(100.0)
                        .with_qty(1.0)
                        .with_commission(400.0, 1.0, 0.0)
                        .filled(),
                )
                .with_order(Order::new(4, 50.0, Side::Sell, OrderType::StopMarket).with_qty(1.0)),
            Position::new("BTCUSDT".to_string())
                .with_order(
                    Order::new(5, 100.0, Side::Buy, OrderType::Market)
                        .updated(5)
                        .with_price_executed(100.0)
                        .with_qty(1.0)
                        .with_commission(400.0, 1.0, 0.0)
                        .filled(),
                )
                .with_order(
                    Order::new(6, 150.0, Side::Sell, OrderType::TakeProfitMarket)
                        .updated(6)
                        .with_price_executed(150.0)
                        .with_qty(1.0)
                        .with_commission(150.0, 1.0, 0.0)
                        .filled(),
                ),
        ]
    }

    #[test]
    fn test_check_tp_sl() {
        let mut positions_opened = get_positions_opened();
        check_tp_sl(
            &KLine::blank().with_close(300.0),
            &mut positions_opened,
            0.0,
        );
        dbg!(positions_opened[0].orders.clone());
        assert_eq!(positions_opened.len(), 3);
        assert_eq!(positions_opened[0].orders.len(), 2);
        assert_eq!(positions_opened[0].volume_all(), 0.0);
        assert_eq!(positions_opened[0].orders[0].status, OrderStatus::Filled);
        assert_eq!(positions_opened[0].orders[1].status, OrderStatus::Filled);
        assert_eq!(positions_opened[1].orders.len(), 2);
        assert_eq!(positions_opened[1].volume_all(), 1.0);
        assert_eq!(positions_opened[1].orders[0].status, OrderStatus::Filled);
        assert_eq!(positions_opened[1].orders[1].status, OrderStatus::New);
        check_tp_sl(&KLine::blank().with_close(10.0), &mut positions_opened, 0.0);
        assert_eq!(positions_opened[1].orders.len(), 2);
        assert_eq!(positions_opened[1].volume_all(), 0.0);
        assert_eq!(positions_opened[1].orders[0].status, OrderStatus::Filled);
        assert_eq!(positions_opened[1].orders[1].status, OrderStatus::Filled);
    }

    #[test]
    fn test_remove_closed_positions() {
        let mut positions_opened = get_positions_opened();
        let positions_closed = remove_closed_positions(&mut positions_opened);
        assert_eq!(positions_opened.len(), 2);
        assert_eq!(positions_closed.len(), 1);
        assert_eq!(positions_closed[0].orders.len(), 2);
        assert_eq!(positions_closed[0].volume_all(), 0.0);
        assert_eq!(positions_closed[0].orders[0].status, OrderStatus::Filled);
        assert_eq!(positions_closed[0].orders[1].status, OrderStatus::Filled);
    }
}
