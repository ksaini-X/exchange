#[cfg(test)]
#[cfg(predicate)]
mod tests {
    use crate::orderbook::book;
    use rust_decimal::{Decimal, dec};
    use std::{assert_eq, dbg, println};

    use crate::{
        domain::{
            asset::Asset,
            ids::AccountId,
            instrument::Instrument,
            order::{Order, OrderSide, TimeInForce},
            quantity::QuantityLots,
        },
        orderbook::book::OrderBook,
    };
    const TICK_SIZE: Decimal = dec!(0.01);
    const LOT_SIZE: Decimal = dec!(0.01);
    fn creatre_instrument() -> Instrument {
        Instrument::new(Asset::Btc, Asset::Usdc, TICK_SIZE, LOT_SIZE).unwrap()
    }
    #[test]
    fn test_create_orderbook() {
        let orderbook = OrderBook::new(creatre_instrument().id());
        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(orderbook.bids.len(), 0);
        assert_eq!(orderbook.last_traded_price, None);
    }
    #[test]
    fn test_add_order() {
        let instrument = creatre_instrument();
        let mut orderbook = OrderBook::new(instrument.id());
        let account_id = AccountId::new();
        let bid_order = Order::new(
            &instrument,
            account_id,
            OrderSide::Buy,
            dec!(100),
            dec!(10),
            TimeInForce::Gtc,
        )
        .unwrap();
        let ask_order = Order::new(
            &instrument,
            account_id,
            OrderSide::Sell,
            dec!(101),
            dec!(10),
            TimeInForce::Gtc,
        )
        .unwrap();

        orderbook.add_order(bid_order).unwrap();
        orderbook.add_order(ask_order).unwrap();
        assert_eq!(orderbook.asks.len(), 1);
        assert_eq!(orderbook.bids.len(), 1);
    }
    #[test]
    fn test_match_order() {
        let instrument = creatre_instrument();
        let mut orderbook = OrderBook::new(instrument.id());
        let account_id = AccountId::new();
        let bid_order = Order::new(
            &instrument,
            account_id,
            OrderSide::Buy,
            dec!(100),
            dec!(10),
            TimeInForce::Gtc,
        )
        .unwrap();
        let ask_order = Order::new(
            &instrument,
            account_id,
            OrderSide::Sell,
            dec!(100),
            dec!(10),
            TimeInForce::Gtc,
        )
        .unwrap();

        println!("{:?}", orderbook);

        let (processed_quantityt_lots, fills) = orderbook.add_order(bid_order).unwrap();
        orderbook.add_order(ask_order).unwrap();
        assert_eq!(orderbook.asks.len(), 0);
        assert_eq!(
            processed_quantityt_lots,
            QuantityLots::from_quantity(dec!(10), LOT_SIZE).unwrap()
        );
        assert_eq!(fills.len(), 1);
        dbg!(orderbook);
    }
}
