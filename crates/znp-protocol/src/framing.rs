//! This provides functions to build `zstack` frames.
//!
//! These look like this:
//!
//! ```
//! n bytes: | 1       | 2       | 0-250
//!          | Length  | Command | Data
//! ```
//! Length: length of data field (0 to 250)

use crate::commands::{CommandType, SubSystem};

pub struct CommandInfo {
    pub ty: CommandType,
    pub sub_system: SubSystem,
    pub id: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandIDError {
    #[error("Could not deserialize {got} into a command type")]
    NotCommandType { got: u8 },
    #[error("Could not deserialize {got} into a subsystem")]
    NotSubSystem { got: u8 },
}

impl CommandInfo {
    pub fn serialize(&self) -> [u8; 2] {
        let mut res = [0u8; 2];
        res[0] = self.ty as u8 | self.sub_system as u8;
        res[1] = self.id;
        res
    }

    pub fn deserialize(buf: [u8; 2]) -> Result<Self, CommandIDError> {
        let ty = buf[0] & 0b1111_0000;
        let sub_system = buf[0] & 0b0000_1111;
        Ok(Self {
            ty: CommandType::from_repr(ty)
                .ok_or(CommandIDError::NotCommandType { got: ty })?,
            sub_system: SubSystem::from_repr(sub_system)
                .ok_or(CommandIDError::NotSubSystem { got: sub_system })?,
            id: buf[1],
        })
    }
}
