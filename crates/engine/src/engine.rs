use std::collections::{HashMap, HashSet};

use rust_decimal::Decimal;

use crate::{
    accounts::{
        account::{Account, AccountError},
        balance::AssetAmount,
    },
    domain::{
        asset::Asset,
        ids::{AccountId, InstrumentId, OrderId},
        instrument::{Instrument, InstrumentError},
        order::Order,
        quantity::QuantityLots,
    },
    orderbook::{
        book::{OrderBook, OrderBookError},
        fill::Fill,
        resting_order::RestingOrder,
    },
};

#[derive(Debug)]
pub struct Engine {
    instruments: HashMap<InstrumentId, Instrument>,
    accounts: HashMap<AccountId, Account>,
    orderbooks: HashMap<InstrumentId, OrderBook>,
    supported_assets: HashSet<Asset>,
}

impl Engine {
    pub fn new(supported_assets: impl IntoIterator<Item = Asset>) -> Self {
        Self {
            instruments: HashMap::new(),
            accounts: HashMap::new(),
            orderbooks: HashMap::new(),
            supported_assets: supported_assets.into_iter().collect(),
        }
    }

    pub fn register_instrument(
        &mut self,
        base_asset: Asset,
        quote_asset: Asset,
        tick_size: Decimal,
        lot_size: Decimal,
    ) -> Result<InstrumentId, EngineError> {
        let exists = self.instruments.values().find(|i| {
            i.get_assets().quote_asset == quote_asset && i.get_assets().base_asset == base_asset
        });
        if exists.is_some() {
            return Err(EngineError::InstrumentExists);
        }
        if !self.supported_assets.contains(&quote_asset)
            || !self.supported_assets.contains(&base_asset)
        {
            return Err(EngineError::AssetNotSupported);
        }
        let instrument = Instrument::new(base_asset, quote_asset, tick_size, lot_size)?;
        let instrument_id = instrument.id();
        let orderbook = OrderBook::new(instrument_id);
        self.orderbooks.insert(instrument_id, orderbook);
        self.instruments.insert(instrument_id, instrument);
        Ok(instrument_id)
    }

    pub fn create_account(&mut self) -> Result<AccountId, EngineError> {
        let mut account = Account::new(self.supported_assets.iter().copied());
        let account_id = account.id();
        account.deposit_asset(Asset::Usdc, AssetAmount::from_atomic_units(5_000_000_000))?;
        self.accounts.insert(account_id, account);
        Ok(account_id)
    }

    pub fn get_account(&self, account_id: AccountId) -> Result<&Account, EngineError> {
        self.accounts
            .get(&account_id)
            .ok_or(EngineError::AccountNotFound)
    }
    pub fn get_instrument(&self, instrument_id: InstrumentId) -> Result<&Instrument, EngineError> {
        self.instruments
            .get(&instrument_id)
            .ok_or(EngineError::AccountNotFound)
    }

    pub fn place_order(
        &mut self,
        incoming_order: Order,
    ) -> Result<(QuantityLots, Vec<Fill>), EngineError> {
        let instrument = self.get_instrument(incoming_order.instrument_id())?;
        let orderbook = self
            .orderbooks
            .get_mut(&instrument.id())
            .ok_or(EngineError::OrderbookNotFound)?;
        let (quantity_lots, fills) = orderbook.add_order(incoming_order)?;
        Ok((quantity_lots, fills))
    }
    pub fn cancel_order(
        &mut self,
        order_id: OrderId,
        instrument_id: InstrumentId,
    ) -> Result<RestingOrder, EngineError> {
        let instrument = self.get_instrument(instrument_id)?;
        let orderbook = self
            .orderbooks
            .get_mut(&instrument.id())
            .ok_or(EngineError::OrderbookNotFound)?;
        let cancelled_order = orderbook.cancel_order(order_id)?;
        Ok(cancelled_order)
    }
}

pub enum EngineError {
    InstrumentCreationFailed,
    OrderbookCreationFailed,
    AccountCreationFailed,
    InstrumentExists,
    AssetNotSupported,
    AccountNotFound,
    OrderbookNotFound,
    InstrumentNotFound,
    Instrumnet(InstrumentError),
    Account(AccountError),
    Orderbook(OrderBookError),
}

impl From<InstrumentError> for EngineError {
    fn from(error: InstrumentError) -> Self {
        EngineError::Instrumnet(error)
    }
}

impl From<AccountError> for EngineError {
    fn from(error: AccountError) -> Self {
        EngineError::Account(error)
    }
}

impl From<OrderBookError> for EngineError {
    fn from(error: OrderBookError) -> Self {
        EngineError::Orderbook(error)
    }
}
