use crate::domain::{order::Order, quantity::QuantityLots};

pub struct RestingOrder {
    remaining_quantity: QuantityLots,
    order: Order,
}

impl RestingOrder {
    pub fn new(order: Order) -> Self {
        Self {
            remaining_quantity: order.quantity(),
            order,
        }
    }
    pub fn fill(&mut self, fill_quantity: QuantityLots) -> Result<(), RestingOrderError> {
        if fill_quantity.is_zero() {
            return Err(RestingOrderError::ZeroFillQuantity);
        }
        self.remaining_quantity = self
            .remaining_quantity
            .checked_sub(fill_quantity)
            .ok_or(RestingOrderError::FillMoreThanRemainingLots)?;

        Ok(())
    }
    pub fn remaining_quantity(&self) -> QuantityLots {
        self.remaining_quantity
    }

    pub fn is_filled(&self) -> bool {
        self.remaining_quantity().is_zero()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RestingOrderError {
    FillMoreThanRemainingLots,
    ZeroFillQuantity,
}

#[cfg(test)]
mod test {
    use std::{assert_eq, result};

    use rust_decimal::dec;

    use super::*;
    use crate::domain::{
        asset::Asset,
        ids::AccountId,
        instrument::Instrument,
        order::{Order, OrderSide, TimeInForce},
    };
    fn create_order() -> Order {
        let instrument = Instrument::new(Asset::Btc, Asset::Usdc, dec!(0.01), dec!(0.01)).unwrap();
        let account_id = AccountId::new();
        Order::new(
            &instrument,
            account_id,
            OrderSide::Buy,
            dec!(65),
            dec!(10),
            TimeInForce::Gtc,
        )
        .unwrap()
    }
    #[test]
    fn test_resting_order_creation() {
        let order = create_order();
        let resting_order = RestingOrder::new(order);
        assert_eq!(resting_order.remaining_quantity, order.quantity());
    }

    #[test]
    fn test_resting_order_fill() {
        let order = create_order();
        let mut resting_order = RestingOrder::new(order);
        assert_eq!(resting_order.remaining_quantity, order.quantity());
        resting_order
            .fill(QuantityLots::from_quantity(dec!(5), dec!(0.01)).unwrap())
            .unwrap();

        assert_eq!(
            resting_order.remaining_quantity,
            QuantityLots::from_quantity(dec!(5), dec!(0.01)).unwrap()
        );
        assert_eq!(resting_order.is_filled(), false);
    }

    #[test]
    fn test_resting_order_is_filled() {
        let order = create_order();
        let mut resting_order = RestingOrder::new(order);
        assert_eq!(resting_order.remaining_quantity, order.quantity());
        resting_order
            .fill(QuantityLots::from_quantity(dec!(10), dec!(0.01)).unwrap())
            .unwrap();

        assert_eq!(resting_order.is_filled(), true);
    }
    #[test]
    fn test_resting_order_fill_more_than_remaining() {
        let order = create_order();
        let mut resting_order = RestingOrder::new(order);
        assert_eq!(resting_order.remaining_quantity, order.quantity());
        let result =
            resting_order.fill(QuantityLots::from_quantity(dec!(100), dec!(0.01)).unwrap());

        assert_eq!(result, Err(RestingOrderError::FillMoreThanRemainingLots));
    }

    #[test]
    fn test_resting_order_zero_fill() {
        let order = create_order();
        let mut resting_order = RestingOrder::new(order);
        assert_eq!(resting_order.remaining_quantity, order.quantity());
        let one = QuantityLots::from_quantity(dec!(1), dec!(1)).unwrap();
        let zero = one.checked_sub(one).unwrap();
        let result = resting_order.fill(zero);
        assert_eq!(result, Err(RestingOrderError::ZeroFillQuantity));
    }
}
