use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct InstrumentId(Uuid);

impl InstrumentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::ids::InstrumentId;

    #[test]
    fn test_instrument_id() {
        let instrument_id = InstrumentId::new();
        assert!(!instrument_id.0.is_nil())
    }
}
