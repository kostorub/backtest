use serde::{Deserialize, Serialize};

use super::enums::{OrderStatus, OrderType, Side};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub date: i64,
    pub date_update: Option<i64>,
    pub price: f64,
    pub price_executed: Option<f64>,
    pub qty: Option<f64>,
    pub commission: Option<f64>,
    pub order_type: OrderType,
    pub side: Side,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(date: i64, price: f64, side: Side, order_type: OrderType) -> Self {
        Self {
            date,
            date_update: None,
            price,
            price_executed: None,
            qty: None,
            commission: None,
            side,
            status: OrderStatus::default(),
            order_type,
        }
    }

    pub fn updated(mut self, date_update: i64) -> Self {
        self.date_update = Some(date_update);
        self
    }

    pub fn with_price_executed(mut self, price: f64) -> Self {
        self.price_executed = Some(price);
        self
    }

    pub fn with_qty(mut self, qty: f64) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn with_commission(mut self, price: f64, qty: f64, commission: f64) -> Self {
        // commission is in percents like 1% or 0.5%
        self.commission = Some(price * qty * commission / 100.0);
        self
    }

    pub fn filled(mut self) -> Self {
        self.status = OrderStatus::Filled;
        self
    }

    pub fn update(&mut self, date_update: i64) -> &mut Self {
        self.date_update = Some(date_update);
        self
    }

    pub fn set_executed_price(&mut self, price: f64) -> &mut Self {
        self.price_executed = Some(price);
        self
    }

    pub fn set_qty(&mut self, qty: f64) -> &mut Self {
        self.qty = Some(qty);
        self
    }

    pub fn set_commission(&mut self, price: f64, qty: f64, commission: f64) -> &mut Self {
        // commission is in percents like 1% or 0.5%
        self.commission = Some(price * qty * commission / 100.0);
        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.status = OrderStatus::Filled;
        self
    }
}
