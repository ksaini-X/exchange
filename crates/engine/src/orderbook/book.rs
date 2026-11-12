use std::{cmp::min, collections::BTreeMap};

use crate::{
    domain::{
        ids::InstrumentId,
        order::{Order, OrderSide},
        price::PriceTicks,
        quantity::QuantityLots,
    },
    orderbook::{fill::Fill, price_level::PriceLevel, resting_order::RestingOrder},
};

pub struct OrderBook {
    instrument_id: InstrumentId,
    bids: BTreeMap<PriceTicks, PriceLevel>,
    asks: BTreeMap<PriceTicks, PriceLevel>,
    last_traded_price: Option<PriceTicks>,
}

impl OrderBook {
    pub fn new(instrument_id: InstrumentId) -> Self {
        Self {
            instrument_id,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_traded_price: None,
        }
    }

    fn add_order(&mut self, order: Order) {
        match order.side() {
            OrderSide::Buy => {}
            OrderSide::Sell => {}
        }
    }

    fn match_bids(&mut self, order: Order) {
        let mut remaining_quantity: QuantityLots = order.quantity();
        let mut fills: Vec<Fill> = Vec::new();
    }
    fn match_asks(&mut self, order: Order) -> Result<(QuantityLots, Vec<Fill>), OrderBookError> {
        let mut filled_quantity_lots: QuantityLots = QuantityLots::ZERO();
        let mut fills: Vec<Fill> = Vec::new();
        let mut resting_order = RestingOrder::new(order);

        for (price_ticks, price_levels) in self.asks.iter_mut() {
            if order.price() >= *price_ticks
                && resting_order.remaining_quantity() != QuantityLots::ZERO()
            {
                for asks in &price_levels.orders {
                    let ask: &mut RestingOrder = price_levels.front_mut().unwrap();
                    let available_quantity_lots = ask.remaining_quantity();

                    let fulfilling_lots =
                        if resting_order.remaining_quantity() > available_quantity_lots {
                            available_quantity_lots
                        } else {
                            min(resting_order.remaining_quantity(), available_quantity_lots)
                        };

                    resting_order.fill(fulfilling_lots).unwrap();
                    ask.fill(fulfilling_lots).unwrap();

                    let fill = Fill::new(
                        ask.order().get_order_id(),
                        order.get_order_id(),
                        fulfilling_lots,
                        *price_ticks,
                        order.side(),
                    );
                    fills.push(fill);
                    filled_quantity_lots =
                        filled_quantity_lots.checked_add(fulfilling_lots).unwrap();

                    if ask.remaining_quantity().is_zero() {
                        price_levels.pop_front();
                    }
                }
            } else {
                break;
            }
        }
        if !resting_order.is_filled() {
            self.insert_resting_order(order.side(), order.price(), resting_order);
        }

        Ok((filled_quantity_lots, fills))
    }

    fn best_bid(&self) -> Result<PriceTicks, OrderBookError> {
        let best_bid = self.bids.last_key_value();
        match best_bid {
            None => Err(OrderBookError::OrderBookEmpty),
            Some((price_ticks, _)) => Ok(*price_ticks),
        }
    }
    fn best_ask(&self) -> Result<PriceTicks, OrderBookError> {
        let best_ask = self.asks.first_key_value();
        match best_ask {
            None => Err(OrderBookError::OrderBookEmpty),
            Some((price_ticks, _)) => Ok(*price_ticks),
        }
    }
    fn insert_resting_order(
        &mut self,
        side: OrderSide,
        price: PriceTicks,
        resting_order: RestingOrder,
    ) {
        let levels = match side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        levels
            .entry(price)
            .or_insert_with(PriceLevel::new)
            .add_order(resting_order);
    }
    fn cancel_order() {}
}

enum OrderBookError {
    OrderBookEmpty,
}
