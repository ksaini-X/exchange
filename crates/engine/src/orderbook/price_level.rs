use std::collections::VecDeque;

use crate::{domain::ids::OrderId, orderbook::resting_order::RestingOrder};

#[derive(Debug)]
pub struct PriceLevel {
    orders: VecDeque<RestingOrder>,
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

    pub fn pop_back(&mut self) -> Option<RestingOrder> {
        self.orders.pop_back()
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn get_order(&self, order_id: OrderId) -> Option<&RestingOrder> {
        self.orders
            .iter()
            .find(|order| order.order().get_order_id() == order_id)
    }
    pub fn remove_order(&mut self, order_id: OrderId) -> Option<RestingOrder> {
        let index = self
            .orders
            .iter()
            .position(|order| order.order().get_order_id() == order_id)?;

        self.orders.remove(index)
    }
}
