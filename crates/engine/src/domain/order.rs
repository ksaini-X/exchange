use crate::domain::{
    ids::{AccountId, InstrumentId, OrderId},
    instrument::Instrument,
    price::{PriceError, PriceTicks},
    quantity::{QuantityError, QuantityLots},
};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    instrument_id: InstrumentId,
    account_id: AccountId,
    order_id: OrderId,
    side: OrderSide,
    price: PriceTicks,
    quantity: QuantityLots,
    time_in_force: TimeInForce,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderError {
    InvalidPrice(PriceError),
    InvalidQuantity(QuantityError),
}

impl From<PriceError> for OrderError {
    fn from(error: PriceError) -> Self {
        Self::InvalidPrice(error)
    }
}

impl From<QuantityError> for OrderError {
    fn from(error: QuantityError) -> Self {
        Self::InvalidQuantity(error)
    }
}

impl Order {
    pub fn new(
        instrument: &Instrument,
        account_id: AccountId,
        side: OrderSide,
        price: Decimal,
        quantity: Decimal,
        time_in_force: TimeInForce,
    ) -> Result<Self, OrderError> {
        let price_ticks = PriceTicks::from_price(price, instrument.tick_size())?;
        let quantity_lots = QuantityLots::from_quantity(quantity, instrument.lot_size())?;
        let order_id = OrderId::new();

        Ok(Self {
            instrument_id: instrument.id(),
            account_id,
            order_id,
            side,
            quantity: quantity_lots,
            price: price_ticks,
            time_in_force,
        })
    }
    pub fn quantity(&self) -> QuantityLots {
        self.quantity
    }
    pub fn price(&self) -> PriceTicks {
        self.price
    }
    pub fn side(&self) -> OrderSide {
        self.side
    }

    pub fn get_order_id(&self) -> OrderId {
        self.order_id
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::dec;

    use super::*;
    use crate::domain::{asset::Asset, ids::AccountId, instrument::Instrument, order::Order};
    #[test]
    fn test_order_creation() {
        let instrument = Instrument::new(Asset::Btc, Asset::Usdc, dec!(0.01), dec!(0.01)).unwrap();
        let account_id = AccountId::new();

        let order = Order::new(
            &instrument,
            account_id,
            OrderSide::Buy,
            dec!(100),
            dec!(10),
            TimeInForce::Gtc,
        );

        assert!(&order.is_ok());
    }
}
