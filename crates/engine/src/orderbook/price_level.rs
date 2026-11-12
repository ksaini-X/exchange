use std::{collections::VecDeque, ops::Index};

use crate::orderbook::resting_order::RestingOrder;

pub struct PriceLevel {
    orders: VecDeque<RestingOrder>,
}

impl PriceLevel {
    pub fn new() -> Self {
        Self {
            orders: VecDeque::new(),
        }
    }

    pub fn add_order(&mut self, order: RestingOrder) -> Result<()> {
        self.orders.push_back(order);
        Ok(())
    }

    pub fn front(&self) -> Option<&RestingOrder> {
        self.orders.get(0)
    }

    pub fn front_mut(&mut self) -> Option<&mut RestingOrder> {
        self.orders.get_mut(0)
    }

    pub fn pop_front(&mut self) -> Result<RestingOrder, PriceLevelError> {
        self.orders.pop_front().ok_or(PriceLevelError::InvalidEntry)
    }

    fn is_empty(&self) -> bool {
        self.orders.len() == 0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PriceLevelError {
    InvalidPriceLevel,
    InvalidEntry,
}
