use rust_decimal::{Decimal, prelude::ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantityLots(u64);

impl QuantityLots {
    pub fn from_quantity(quantity: Decimal, lot_size: Decimal) -> Result<Self, QuantityError> {
        if lot_size <= Decimal::ZERO {
            return Err(QuantityError::InvalidLotSize);
        }

        if quantity <= Decimal::ZERO {
            return Err(QuantityError::InvalidQuantity);
        }

        if quantity % lot_size != Decimal::ZERO {
            return Err(QuantityError::QuantityNotMultipleOfLot);
        }

        let lots = quantity
            .checked_div(lot_size)
            .and_then(|value| value.to_u64())
            .ok_or(QuantityError::InvalidComputation)?;

        Ok(Self(lots))
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0).map(Self)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn ZERO() -> Self {
        Self(0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum QuantityError {
    InvalidQuantity,
    InvalidLotSize,
    QuantityNotMultipleOfLot,
    InvalidComputation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn test_valid_quantity() {
        let quantity = QuantityLots::from_quantity(dec!(1.250), dec!(0.001));

        assert_eq!(quantity, Ok(QuantityLots(1250)));
    }

    #[test]
    fn test_invalid_zero_quantity() {
        let quantity = QuantityLots::from_quantity(dec!(0), dec!(0.001));

        assert_eq!(quantity, Err(QuantityError::InvalidQuantity));
    }

    #[test]
    fn test_invalid_negative_quantity() {
        let quantity = QuantityLots::from_quantity(dec!(-1), dec!(0.001));

        assert_eq!(quantity, Err(QuantityError::InvalidQuantity));
    }

    #[test]
    fn test_invalid_zero_lot_size() {
        let quantity = QuantityLots::from_quantity(dec!(1.250), dec!(0));

        assert_eq!(quantity, Err(QuantityError::InvalidLotSize));
    }

    #[test]
    fn test_invalid_negative_lot_size() {
        let quantity = QuantityLots::from_quantity(dec!(1.250), dec!(-0.001));

        assert_eq!(quantity, Err(QuantityError::InvalidLotSize));
    }

    #[test]
    fn test_quantity_not_multiple_of_lot() {
        let quantity = QuantityLots::from_quantity(dec!(1.2505), dec!(0.001));

        assert_eq!(quantity, Err(QuantityError::QuantityNotMultipleOfLot));
    }

    #[test]
    fn test_invalid_computation() {
        let quantity = QuantityLots::from_quantity(dec!(18446744073709551616), dec!(1));

        assert_eq!(quantity, Err(QuantityError::InvalidComputation));
    }
}
