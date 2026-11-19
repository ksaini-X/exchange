use engine::domain::{
    asset::Asset,
    ids::{AccountId, InstrumentId, OrderId},
    order::{OrderSide, TimeInForce},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub enum Command {
    CreateAccount,
    RegisterInstrument {
        base_asset: Asset,
        symbol: String,
        qoute_asset: Asset,
        lot_size: Decimal,
        tick_size: Decimal,
    },
    PlaceOrder {
        account_id: AccountId,
        instrument_id: InstrumentId,
        price: Decimal,
        quantity: Decimal,
        side: OrderSide,
        time_in_force: TimeInForce,
    },
    CancelOrder {
        account_id: AccountId,
        instrument_id: InstrumentId,
        order_id: OrderId,
    },
}
