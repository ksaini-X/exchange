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
        // Validate instrument.
        if order.instrument_id() != self.instrument_id {
            return Err(OrderBookError::InstrumentMismatch);
        }

        // V1 supports GTC only.
        if order.time_in_force() != TimeInForce::Gtc {
            return Err(OrderBookError::UnsupportedTimeInForce);
        }

        // An active order ID must be unique.
        if self.order_index.contains_key(&order.get_order_id()) {
            return Err(OrderBookError::DuplicateOrderId);
        }

        match order.side() {
            OrderSide::Buy => self.match_asks(order),
            OrderSide::Sell => self.match_bids(order),
        }
    }

    // Incoming BUY matches existing ASK orders.
    fn match_asks(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        let taker_order_id = order.get_order_id();
        let limit_price = order.price();

        let mut taker = RestingOrder::new(order);
        let mut filled_quantity = QuantityLots::ZERO();
        let mut fills = Vec::new();
        let mut last_execution_price = None;

        for (price_ticks, price_level) in self.asks.iter_mut() {
            // Asks are ascending.
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

                // Mutable maker borrow has ended.
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

        // Remove empty ask levels.
        self.asks.retain(|_, level| !level.is_empty());

        if let Some(price) = last_execution_price {
            self.last_traded_price = Some(price);
        }

        // GTC remainder rests on the bid side.
        if !taker.is_filled() {
            self.insert_resting_order(taker);
        }

        Ok((filled_quantity, fills))
    }

    // Incoming SELL matches existing BID orders.
    fn match_bids(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        let taker_order_id = order.get_order_id();
        let limit_price = order.price();

        let mut taker = RestingOrder::new(order);
        let mut filled_quantity = QuantityLots::ZERO();
        let mut fills = Vec::new();
        let mut last_execution_price = None;

        // Bids are processed from highest to lowest.
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

        // Remove empty bid levels.
        self.bids.retain(|_, level| !level.is_empty());

        if let Some(price) = last_execution_price {
            self.last_traded_price = Some(price);
        }

        // GTC remainder rests on the ask side.
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

        // Index only orders that are actually resting.
        self.order_index.insert(order_id, (side, price));
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
}

impl From<RestingOrderError> for OrderBookError {
    fn from(error: RestingOrderError) -> Self {
        Self::RestingOrder(error)
    }
}
