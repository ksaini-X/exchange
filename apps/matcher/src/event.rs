use engine::{
    domain::ids::{AccountId, InstrumentId, OrderId},
    orderbook::trade::Trade,
};

pub enum Event {
    AccountCreated { account_id: AccountId },
    InstrumentRegistered { instrument_id: InstrumentId },
    OrderAccepted { order_id: OrderId },
    OrderCancelled { order_id: OrderId },
    TradeExecuted { trade: Trade },
}
