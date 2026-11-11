use rust_decimal::Decimal;

use crate::domain::{asset::Asset, ids::InstrumentId};

#[derive(PartialEq, Debug)]
pub struct Instrument {
    pub instrument_id: InstrumentId,
    pub symbol: String,
    pub base_asset: Asset,
    pub quote_asset: Asset,
    pub tick_size: Decimal,
    pub lot_size: Decimal,
}

impl Instrument {
    pub fn new(
        base_asset: Asset,
        quote_asset: Asset,
        tick_size: Decimal,
        lot_size: Decimal,
    ) -> Result<Self, InstrumentError> {
        if base_asset == quote_asset {
            return Err(InstrumentError::IdenticalAssets);
        }

        if tick_size <= Decimal::ZERO {
            return Err(InstrumentError::InvalidTickSize);
        }

        if lot_size <= Decimal::ZERO {
            return Err(InstrumentError::InvalidLotSize);
        }
        let instrument_id = InstrumentId::new();
        let symbol = base_asset.to_string() + &"/" + &quote_asset.to_string();
        Ok(Self {
            instrument_id,
            symbol,
            quote_asset,
            base_asset,
            tick_size,
            lot_size,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum InstrumentError {
    IdenticalAssets,
    InvalidTickSize,
    InvalidLotSize,
}

#[cfg(test)]
mod instrument_test {
    use rust_decimal::dec;
    use std::assert_eq;

    use crate::domain::{
        asset::Asset,
        instrument::{Instrument, InstrumentError},
    };
    #[test]
    fn test_instrument_creation() {
        let instrument = Instrument::new(Asset::Btc, Asset::Usdc, dec!(0.01), dec!(0.001)).unwrap();
        assert_eq!(instrument.base_asset, Asset::Btc);
        assert_eq!(instrument.quote_asset, Asset::Usdc);
        assert_eq!(instrument.symbol, "BTC/USDC");
        assert_eq!(instrument.tick_size, dec!(0.01));
        assert_eq!(instrument.lot_size, dec!(0.001));
    }

    #[test]
    fn test_identical_asset_instrument_creation() {
        let instrument = Instrument::new(Asset::Btc, Asset::Btc, dec!(0.01), dec!(0.001));
        assert!(instrument.is_err());
        assert_eq!(instrument, Err(InstrumentError::IdenticalAssets));
    }

    #[test]
    fn test_instrument_creation_with_invalid_tick_size() {
        let instrument = Instrument::new(Asset::Btc, Asset::Sol, dec!(0), dec!(0.001));
        assert!(instrument.is_err());
        assert_eq!(instrument, Err(InstrumentError::InvalidTickSize));
    }

    #[test]
    fn test_instrument_creation_with_negative_lot_size() {
        let instrument = Instrument::new(Asset::Btc, Asset::Sol, dec!(0.01), dec!(-0.01));
        assert!(instrument.is_err());
        assert_eq!(instrument, Err(InstrumentError::InvalidLotSize));
    }
    #[test]
    fn test_instrument_creation_with_negative_tick_size() {
        let instrument = Instrument::new(Asset::Btc, Asset::Sol, dec!(-0.01), dec!(0.001));
        assert!(instrument.is_err());
        assert_eq!(instrument, Err(InstrumentError::InvalidTickSize));
    }

    #[test]
    fn test_instrument_creation_with_invalid_lot_size() {
        let instrument = Instrument::new(Asset::Btc, Asset::Sol, dec!(0.01), dec!(0));
        assert!(instrument.is_err());
        assert_eq!(instrument, Err(InstrumentError::InvalidLotSize));
    }
}
