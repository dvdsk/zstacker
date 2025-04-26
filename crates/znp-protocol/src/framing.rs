//! This provides functions to build `zstack` frames.
//!
//! These look like this:
//!
//!_____________________________________ 
//! n bytes: | 1       | 2       | 0-250
//!          | Length  | Command | Data
//! ____________________________________
//! 
//! Length: length of data field (0 to 250)

use crate::commands::{CommandType, SubSystem};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandMeta {
    pub ty: CommandType,
    pub sub_system: SubSystem,
    pub id: u8,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CommandMetaError {
    #[error("Could not deserialize {got} into a command type")]
    NotCommandType { got: u8 },
    #[error("Could not deserialize {got} into a subsystem")]
    NotSubSystem { got: u8 },
}

impl CommandMeta {
    pub fn serialize(&self) -> [u8; 2] {
        let mut res = [0u8; 2];
        res[0] = self.ty as u8 | self.sub_system as u8;
        res[1] = self.id;
        res
    }

    pub fn deserialize(buf: [u8; 2]) -> Result<Self, CommandMetaError> {
        let ty = buf[0] & 0b1111_0000;
        let sub_system = buf[0] & 0b0000_1111;
        Ok(Self {
            ty: CommandType::from_repr(ty)
                .ok_or(CommandMetaError::NotCommandType { got: ty })?,
            sub_system: SubSystem::from_repr(sub_system)
                .ok_or(CommandMetaError::NotSubSystem { got: sub_system })?,
            id: buf[1],
        })
    }
}
