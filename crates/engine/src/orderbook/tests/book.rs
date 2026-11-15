#[cfg(test)]
pub mod test {
    use rust_decimal::dec;

    use crate::{
        domain::{asset::Asset, instrument::Instrument, quantity::QuantityLots},
        orderbook::book,
    };
    fn create_instrument() -> Instrument {
        let instrument = Instrument::new(Asset::Sol, Asset::Usdc, dec!(0.01), dec!(0.01)).unwrap();
        instrument
    }
    #[test]
    fn test_orderbook_create() {}

    #[test]
    fn test_orderbook_create_multiple_instances() {}

    #[test]
    fn test_orderbook_starts_with_empty_bids_and_asks() {}

    #[test]
    fn test_orderbook_starts_with_no_last_traded_price() {}
}
