use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstrumentId(Uuid);
impl InstrumentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(Uuid);
impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);
impl AccountId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TradeId(Uuid);
impl TradeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
