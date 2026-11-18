use crate::{command::Command, sequencer::SequenceNumber};
use engine::domain::ids::CommandId;
use std::time::SystemTime;

pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub sequence: SequenceNumber,
    pub timestamp: SystemTime,
    pub command: Command,
}

impl CommandEnvelope {
    pub fn new(sequence: SequenceNumber, command: Command) -> Self {
        Self {
            command_id: CommandId::new(),
            sequence,
            timestamp: SystemTime::now(),
            command,
        }
    }
}
