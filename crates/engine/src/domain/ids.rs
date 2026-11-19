use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct InstrumentId(Uuid);
impl InstrumentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct OrderId(Uuid);
impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct AccountId(Uuid);
impl AccountId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TradeId(Uuid);
impl TradeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CommandId(Uuid);
impl CommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
