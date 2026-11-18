#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequenceNumber(u64);

#[derive(Debug)]
pub struct Sequencer {
    last_committed: SequenceNumber,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            last_committed: SequenceNumber(0),
        }
    }

    pub fn next(&self) -> Result<SequenceNumber, SequenceError> {
        let next = self
            .last_committed
            .0
            .checked_add(1)
            .ok_or(SequenceError::Overflow)?;

        Ok(SequenceNumber(next))
    }

    pub fn commit(&mut self, sequence: SequenceNumber) -> Result<(), SequenceError> {
        let expected = self.next()?;

        if sequence != expected {
            return Err(SequenceError::UnexpectedSequence);
        }

        self.last_committed = sequence;

        Ok(())
    }

    pub fn last_committed(&self) -> SequenceNumber {
        self.last_committed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceError {
    Overflow,
    UnexpectedSequence,
}
