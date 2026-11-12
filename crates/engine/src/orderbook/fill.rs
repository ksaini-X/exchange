use crate::domain::{ids::OrderId, order::OrderSide, price::PriceTicks, quantity::QuantityLots};

pub struct Fill {
    maker_order_id: OrderId,
    taker_order_id: OrderId,
    quantity: QuantityLots,
    price: PriceTicks,
    side: OrderSide,
}

impl Fill {
    pub fn new(
        maker_order_id: OrderId,
        taker_order_id: OrderId,
        quantity: QuantityLots,
        price: PriceTicks,
        side: OrderSide,
    ) -> Self {
        Self {
            maker_order_id,
            taker_order_id,
            quantity,
            price,
            side,
        }
    }
}
