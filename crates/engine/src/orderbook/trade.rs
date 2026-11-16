use std::time::SystemTime;

use crate::domain::{
    ids::{InstrumentId, OrderId, TradeId},
    order::OrderSide,
    price::PriceTicks,
    quantity::QuantityLots,
};

#[derive(Debug)]

pub struct Trade {
    trade_id: TradeId,
    instrument_id: InstrumentId,
    taker_order_id: OrderId,
    maker_order_id: OrderId,
    price: PriceTicks,
    quantity: QuantityLots,
    timestamp: SystemTime,
    taker_side: OrderSide,
}

impl Trade {
    pub fn new(
        instrument_id: InstrumentId,
        taker_order_id: OrderId,
        maker_order_id: OrderId,
        price: PriceTicks,
        quantity: QuantityLots,
        taker_side: OrderSide,
    ) -> Self {
        Self {
            trade_id: TradeId::new(),
            instrument_id,
            taker_order_id,
            maker_order_id,
            price,
            quantity,
            timestamp: SystemTime::now(),
            taker_side,
        }
    }
}
