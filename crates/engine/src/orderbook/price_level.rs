use std::collections::VecDeque;

use crate::orderbook::resting_order::RestingOrder;

pub struct PriceLevel {
    pub orders: VecDeque<RestingOrder>,
}

impl PriceLevel {
    pub fn new() -> Self {
        Self {
            orders: VecDeque::new(),
        }
    }

    pub fn add_order(&mut self, order: RestingOrder) {
        self.orders.push_back(order);
    }

    pub fn front(&self) -> Option<&RestingOrder> {
        self.orders.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut RestingOrder> {
        self.orders.front_mut()
    }

    pub fn pop_front(&mut self) -> Option<RestingOrder> {
        self.orders.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}
