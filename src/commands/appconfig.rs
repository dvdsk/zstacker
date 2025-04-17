#![allow(dead_code)]

use serde::Serialize;
use serde_repr::Serialize_repr;

use super::{AsyncRequest, SyncRequest, IeeeAddr, Status, SubSystem};

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

impl SyncRequest for AddInstallCode {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
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
impl SyncRequest for BdbStartCommissioning {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbSetChannel {
    is_primary: u8,
    channel: u32,
}

impl SyncRequest for BdbSetChannel {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbSetTcRequireKeyExchange {
    value: u8,
}
impl SyncRequest for BdbSetTcRequireKeyExchange {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BdbComissioningNotification {
    status: u8,
}

impl AsyncRequest for BdbComissioningNotification {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
}

#[derive(Debug, Clone, Serialize)]
struct SetNwkFrameCounter {
    value: u32,
}
impl SyncRequest for SetNwkFrameCounter {
    const ID: u8 = 255;
    const SUBSYSTEM: SubSystem = SubSystem::AppConfig;
    type Reply = Status;
}
