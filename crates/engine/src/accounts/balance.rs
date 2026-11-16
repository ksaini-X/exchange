#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetAmount(u128);

impl AssetAmount {
    pub const ZERO: Self = Self(0);

    pub const fn from_atomic_units(value: u128) -> Self {
        Self(value)
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn as_atomic_units(self) -> u128 {
        self.0
    }

    pub fn checked_add(self, amount: Self) -> Result<Self, AssetAmountError> {
        self.0
            .checked_add(amount.0)
            .map(Self)
            .ok_or(AssetAmountError::Overflow)
    }

    pub fn checked_sub(self, amount: Self) -> Result<Self, AssetAmountError> {
        self.0
            .checked_sub(amount.0)
            .map(Self)
            .ok_or(AssetAmountError::Underflow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance {
    available_balance: AssetAmount,
    locked_balance: AssetAmount,
}

impl Balance {
    pub fn new() -> Self {
        Self {
            available_balance: AssetAmount::ZERO,
            locked_balance: AssetAmount::ZERO,
        }
    }

    pub fn available(&self) -> AssetAmount {
        self.available_balance
    }

    pub fn locked(&self) -> AssetAmount {
        self.locked_balance
    }

    pub fn lock(&mut self, amount: AssetAmount) -> Result<(), BalanceError> {
        if amount.is_zero() {
            return Err(BalanceError::InvalidAmount);
        }

        let new_available = self
            .available_balance
            .checked_sub(amount)
            .map_err(|_| BalanceError::InsufficientAvailableBalance)?;

        let new_locked = self.locked_balance.checked_add(amount)?;

        // Commit only after both calculations succeed.
        self.available_balance = new_available;
        self.locked_balance = new_locked;

        Ok(())
    }

    pub fn unlock(&mut self, amount: AssetAmount) -> Result<(), BalanceError> {
        if amount.is_zero() {
            return Err(BalanceError::InvalidAmount);
        }

        let new_locked = self
            .locked_balance
            .checked_sub(amount)
            .map_err(|_| BalanceError::InsufficientLockedBalance)?;

        let new_available = self.available_balance.checked_add(amount)?;

        // Commit only after both calculations succeed.
        self.available_balance = new_available;
        self.locked_balance = new_locked;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetAmountError {
    Overflow,
    Underflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    InvalidAmount,
    InsufficientAvailableBalance,
    InsufficientLockedBalance,
    Arithmetic(AssetAmountError),
}

impl From<AssetAmountError> for BalanceError {
    fn from(error: AssetAmountError) -> Self {
        Self::Arithmetic(error)
    }
}
