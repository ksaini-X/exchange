use std::io::{Read, Seek, SeekFrom, Write};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::{envelope::CommandEnvelope, sequencer::SequenceNumber};

pub struct Wal {
    file: File,
    last_sequence_number: Option<SequenceNumber>,
}
impl Wal {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
            .map_err(WalError::Io)?;

        let mut wal = Self {
            file,
            last_sequence_number: None,
        };

        let logs = wal.read_all()?;

        wal.last_sequence_number = logs.last().map(|log| log.sequence);

        Ok(wal)
    }

    pub fn last_sequence(&self) -> Option<SequenceNumber> {
        self.last_sequence_number
    }

    pub fn read_all(&mut self) -> Result<Vec<CommandEnvelope>, WalError> {
        self.file.seek(SeekFrom::Start(0)).map_err(WalError::Io)?;

        let mut logs = Vec::new();
        let mut contents = String::new();

        self.file
            .read_to_string(&mut contents)
            .map_err(WalError::Io)?;

        for line in contents.lines() {
            let envelope: CommandEnvelope =
                serde_json::from_str(line).map_err(WalError::Deserialization)?;

            logs.push(envelope);
        }

        Ok(logs)
    }
}
