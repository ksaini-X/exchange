use std::collections::HashMap;

use crate::{
    accounts::balance::{AssetAmount, Balance, BalanceError},
    domain::{asset::Asset, ids::AccountId},
};

pub struct Account {
    account_id: AccountId,
    assets: HashMap<Asset, Balance>,
}
impl Account {
    pub fn new(account_id: AccountId, supported_assets: impl IntoIterator<Item = Asset>) -> Self {
        let assets = supported_assets
            .into_iter()
            .map(|asset| (asset, Balance::new()))
            .collect();

        Self { account_id, assets }
    }

    pub fn id(&self) -> AccountId {
        self.account_id
    }

    pub fn get_asset(&self, asset: Asset) -> Result<&Balance, AccountError> {
        self.assets.get(&asset).ok_or(AccountError::AssetNotFound)
    }

    pub fn lock_asset(&mut self, asset: Asset, amount: AssetAmount) -> Result<(), AccountError> {
        let balance = self
            .assets
            .get_mut(&asset)
            .ok_or(AccountError::AssetNotFound)?;

        balance.lock(amount)?;

        Ok(())
    }

    pub fn unlock_asset(&mut self, asset: Asset, amount: AssetAmount) -> Result<(), AccountError> {
        let balance = self
            .assets
            .get_mut(&asset)
            .ok_or(AccountError::AssetNotFound)?;

        balance.unlock(amount)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccountError {
    AssetNotFound,
    Balance(BalanceError),
}

impl From<BalanceError> for AccountError {
    fn from(error: BalanceError) -> Self {
        Self::Balance(error)
    }
}
