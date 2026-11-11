use std::{error::Error, ops::Div};

use rust_decimal::{Decimal, prelude::ToPrimitive};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct PriceTicks(u64);

impl PriceTicks {
    pub fn from_price(price: Decimal, tick_size: Decimal) -> Result<Self, PriceError> {
        if tick_size <= Decimal::ZERO {
            return Err(PriceError::InvalidTickSize);
        };
        if (price % tick_size) != Decimal::ZERO {
            return Err(PriceError::InvalidPrice);
        }

        let conversion = price.div(tick_size).to_u64();
        match conversion {
            None => Err(PriceError::InvalidComputation),
            Some(price) => Ok(Self(price)),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PriceError {
    InvalidPrice,
    InvalidTickSize,
    InvalidComputation,
}

#[cfg(test)]
mod test {
    use crate::domain::price::{PriceError, PriceTicks};
    use rust_decimal::dec;
    use std::assert_eq;

    #[test]
    fn test_from_price() {
        let price_ticks = PriceTicks::from_price(dec!(100.01), dec!(0.01));
        assert_eq!(price_ticks, Ok(PriceTicks(10001)))
    }
    #[test]
    fn test_invalid_price() {
        let price_ticks = PriceTicks::from_price(dec!(100.001), dec!(0.01));
        assert_eq!(price_ticks, Err(PriceError::InvalidPrice))
    }

    #[test]
    fn test_invalid_tick() {
        let price_ticks = PriceTicks::from_price(dec!(100.001), dec!(-0.01));
        assert_eq!(price_ticks, Err(PriceError::InvalidTickSize))
    }

    #[test]
    fn test_invalid_zero_tick() {
        let price_ticks = PriceTicks::from_price(dec!(100.001), dec!(0));
        assert_eq!(price_ticks, Err(PriceError::InvalidTickSize))
    }
}
