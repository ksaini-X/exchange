use std::{
    cmp::min,
    collections::{BTreeMap, HashMap},
};

use crate::{
    domain::{
        ids::{InstrumentId, OrderId},
        order::{Order, OrderSide, TimeInForce},
        price::PriceTicks,
        quantity::QuantityLots,
    },
    orderbook::{
        fill::Fill,
        price_level::PriceLevel,
        resting_order::{RestingOrder, RestingOrderError},
    },
};

#[derive(Debug)]
pub struct OrderBook {
    instrument_id: InstrumentId,
    bids: BTreeMap<PriceTicks, PriceLevel>,
    asks: BTreeMap<PriceTicks, PriceLevel>,
    last_traded_price: Option<PriceTicks>,
    order_index: HashMap<OrderId, (OrderSide, PriceTicks)>,
}

impl OrderBook {
    pub fn new(instrument_id: InstrumentId) -> Self {
        Self {
            instrument_id,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_traded_price: None,
            order_index: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        if order.instrument_id() != self.instrument_id {
            return Err(OrderBookError::InstrumentMismatch);
        }

        if order.time_in_force() != TimeInForce::Gtc {
            return Err(OrderBookError::UnsupportedTimeInForce);
        }

        if self.order_index.contains_key(&order.get_order_id()) {
            return Err(OrderBookError::DuplicateOrderId);
        }

        match order.side() {
            OrderSide::Buy => self.match_asks(order),
            OrderSide::Sell => self.match_bids(order),
        }
    }

    fn match_asks(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        let taker_order_id = order.get_order_id();
        let limit_price = order.price();

        let mut taker = RestingOrder::new(order);
        let mut filled_quantity = QuantityLots::ZERO();
        let mut fills = Vec::new();
        let mut last_execution_price = None;

        for (price_ticks, price_level) in self.asks.iter_mut() {
            if limit_price < *price_ticks {
                break;
            }

            while !taker.is_filled() && !price_level.is_empty() {
                let (maker_order_id, maker_filled, fill_quantity) = {
                    let maker = price_level
                        .front_mut()
                        .ok_or(OrderBookError::EmptyPriceLevel)?;

                    let fill_quantity = min(taker.remaining_quantity(), maker.remaining_quantity());

                    maker.fill(fill_quantity)?;

                    (
                        maker.order().get_order_id(),
                        maker.is_filled(),
                        fill_quantity,
                    )
                };

                taker.fill(fill_quantity)?;

                filled_quantity = filled_quantity
                    .checked_add(fill_quantity)
                    .ok_or(OrderBookError::QuantityOverflow)?;

                fills.push(Fill::new(
                    maker_order_id,
                    taker_order_id,
                    fill_quantity,
                    *price_ticks,
                    OrderSide::Buy,
                ));

                last_execution_price = Some(*price_ticks);

                if maker_filled {
                    price_level
                        .pop_front()
                        .ok_or(OrderBookError::EmptyPriceLevel)?;

                    self.order_index.remove(&maker_order_id);
                }
            }

            if taker.is_filled() {
                break;
            }
        }

        self.asks.retain(|_, level| !level.is_empty());

        if let Some(price) = last_execution_price {
            self.last_traded_price = Some(price);
        }

        if !taker.is_filled() {
            self.insert_resting_order(taker);
        }

        Ok((filled_quantity, fills))
    }

    fn match_bids(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        let taker_order_id = order.get_order_id();
        let limit_price = order.price();

        let mut taker = RestingOrder::new(order);
        let mut filled_quantity = QuantityLots::ZERO();
        let mut fills = Vec::new();
        let mut last_execution_price = None;

        for (price_ticks, price_level) in self.bids.iter_mut().rev() {
            if *price_ticks < limit_price {
                break;
            }

            while !taker.is_filled() && !price_level.is_empty() {
                let (maker_order_id, maker_filled, fill_quantity) = {
                    let maker = price_level
                        .front_mut()
                        .ok_or(OrderBookError::EmptyPriceLevel)?;

                    let fill_quantity = min(taker.remaining_quantity(), maker.remaining_quantity());

                    maker.fill(fill_quantity)?;

                    (
                        maker.order().get_order_id(),
                        maker.is_filled(),
                        fill_quantity,
                    )
                };

                taker.fill(fill_quantity)?;

                filled_quantity = filled_quantity
                    .checked_add(fill_quantity)
                    .ok_or(OrderBookError::QuantityOverflow)?;

                fills.push(Fill::new(
                    maker_order_id,
                    taker_order_id,
                    fill_quantity,
                    *price_ticks,
                    OrderSide::Sell,
                ));

                last_execution_price = Some(*price_ticks);

                if maker_filled {
                    price_level
                        .pop_front()
                        .ok_or(OrderBookError::EmptyPriceLevel)?;

                    self.order_index.remove(&maker_order_id);
                }
            }

            if taker.is_filled() {
                break;
            }
        }

        self.bids.retain(|_, level| !level.is_empty());

        if let Some(price) = last_execution_price {
            self.last_traded_price = Some(price);
        }

        if !taker.is_filled() {
            self.insert_resting_order(taker);
        }

        Ok((filled_quantity, fills))
    }

    pub fn best_bid(&self) -> Option<PriceTicks> {
        self.bids.last_key_value().map(|(price, _)| *price)
    }

    pub fn best_ask(&self) -> Option<PriceTicks> {
        self.asks.first_key_value().map(|(price, _)| *price)
    }

    pub fn last_traded_price(&self) -> Option<PriceTicks> {
        self.last_traded_price
    }

    fn insert_resting_order(&mut self, resting_order: RestingOrder) {
        let order_id = resting_order.order().get_order_id();
        let side = resting_order.order().side();
        let price = resting_order.order().price();

        let levels = match side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        levels
            .entry(price)
            .or_insert_with(PriceLevel::new)
            .add_order(resting_order);

        self.order_index.insert(order_id, (side, price));
    }

    pub fn get_order(&self, order_id: OrderId) -> Result<&RestingOrder, OrderBookError> {
        let (order_side, price_ticks) = self
            .order_index
            .get(&order_id)
            .ok_or(OrderBookError::OrderIndexInconsistent)?;

        let levels = match *order_side {
            OrderSide::Buy => &self.bids,
            OrderSide::Sell => &self.asks,
        };

        let price_level = levels
            .get(price_ticks)
            .ok_or(OrderBookError::OrderIndexInconsistent)?;

        price_level
            .get_order(order_id)
            .ok_or(OrderBookError::OrderIndexInconsistent)
    }

    pub fn cancel_order(&mut self, order_id: OrderId) -> Result<RestingOrder, OrderBookError> {
        let (side, price) = self
            .order_index
            .get(&order_id)
            .copied()
            .ok_or(OrderBookError::OrderNotFound)?;

        let levels = match side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        let (removed_order, level_is_empty) = {
            let price_level = levels
                .get_mut(&price)
                .ok_or(OrderBookError::OrderIndexInconsistent)?;

            let removed_order = price_level
                .remove_order(order_id)
                .ok_or(OrderBookError::OrderIndexInconsistent)?;

            (removed_order, price_level.is_empty())
        };

        if level_is_empty {
            levels.remove(&price);
        }

        self.order_index.remove(&order_id);

        Ok(removed_order)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderBookError {
    InstrumentMismatch,
    UnsupportedTimeInForce,
    DuplicateOrderId,
    EmptyPriceLevel,
    QuantityOverflow,
    RestingOrder(RestingOrderError),
    OrderNotFound,
    OrderIndexInconsistent,
}

impl From<RestingOrderError> for OrderBookError {
    fn from(error: RestingOrderError) -> Self {
        Self::RestingOrder(error)
    }
}
