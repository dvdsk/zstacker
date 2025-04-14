use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use super::{Command, CommandType, IeeeAddr, Subsystem};

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum InstallCodeFormat {
    InstallCodePlusCRC = 0x01,
    KeyDerivedFromInstallCode = 0x02,
}

#[derive(Debug, Clone)]
pub struct AddInstallCode {
    format: InstallCodeFormat,
    addr: IeeeAddr,
    code: Vec<u8>,
}

impl Serialize for AddInstallCode {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!(
            "can not be automatically serialized. Instead uses custom data_to_vec"
        )
    }
}

impl Command for AddInstallCode {
    const ID: u8 = 4;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;

    fn data_to_vec(&self) -> Result<Vec<u8>, crate::data_format::Error>
    where
        Self: Sized,
    {
        let code = match self.format {
            InstallCodeFormat::InstallCodePlusCRC => {
                self.code.iter().copied().take(usize::MAX)
            }
            InstallCodeFormat::KeyDerivedFromInstallCode => {
                self.code.iter().copied().take(16)
            }
        };

        Ok(std::iter::once(self.format as u8)
            .chain(self.addr.0.to_le_bytes())
            .chain(code)
            .collect())
    }
}

#[derive(Debug, Clone, Serialize)]
struct BdbStartCommissioning {
    mode: u8,
}
impl Command for BdbStartCommissioning {
    const ID: u8 = 5;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbSetChannel {
    is_primary: u8,
    channel: u32,
}

impl Command for BdbSetChannel {
    const ID: u8 = 8;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbSetTcRequireKeyExchange {
    value: u8,
}
impl Command for BdbSetTcRequireKeyExchange {
    const ID: u8 = 9;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbComissioningNotifcation {
    status: u8,
}
impl Command for BdbComissioningNotifcation {
    const ID: u8 = 128;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SetNwkFrameCounter {
    value: u32,
}
impl Command for SetNwkFrameCounter {
    const ID: u8 = 255;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::AppConfig;
    type Reply = Status;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    status: u8,
}
