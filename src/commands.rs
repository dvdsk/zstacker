use std::iter;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;

pub const START_OF_FRAME: u8 = 0xFE;

pub(crate) mod appconfig;
pub(crate) mod util;
pub(crate) mod sys;
pub(crate) mod app;

pub trait Command: Serialize {
    const ID: u8;
    const TYPE: CommandType;
    const SUBSYSTEM: Subsystem;
    type Reply: DeserializeOwned;

    fn data_to_vec(&self) -> Result<Vec<u8>, data_format::Error>
    where
        Self: Sized,
    {
        data_format::to_vec(self)
    }

    fn to_frame(&self) -> Result<Vec<u8>, data_format::Error>
    where
        Self: Sized,
    {
        let serialized = self.data_to_vec()?;
        let frame = [
            serialized.len() as u8,
            Self::SUBSYSTEM as u8 | Self::TYPE as u8,
            Self::ID,
        ]
        .into_iter()
        .chain(serialized.into_iter());

        let checksum = frame
            .clone()
            .into_iter()
            .reduce(|checksum, byte| checksum ^ byte)
            .expect("never empty");

        Ok(iter::once(START_OF_FRAME)
            .chain(frame)
            .chain([checksum])
            .collect())
    }
}

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum CommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum Subsystem {
    Reserved = 0x00,
    SysInterface = 0x01,
    MacInterface = 0x02,
    NwkInterface = 0x03,
    AfInterface = 0x04,
    ZdoInterface = 0x05,
    SapiInterface = 0x06,
    UtilInterface = 0x07,
    DebugInterface = 0x08,
    AppInterface = 0x09,
    AppConfig = 0x0F,
    GreenPower = 0x15,
}

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DeviceState {
    InitializatedNotStartedAutomatically = 0x00,
    InitializatedNotConnected = 0x01,
    DiscoveringPansToJoin = 0x02,
    JoiningAPan = 0x03,
    ReJoiningAPan = 0x04,
    JoinedButNotYetAuthenticatedByTS = 0x05,
    StartedAsDeviceAfterAuthentication = 0x06,
    DeviceJoinedAuthenticatedAndIsRouter = 0x07,
    StartingAsZBCoordinator = 0x08,
    StartedAsZBCoordinator = 0x09,
    DeviceLostInfoAboutParent = 0x0A,
}

#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum DeviceType {
    None = 0,
    Coordinator = 1,
    Router = 2,
    EndDevice = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IeeeAddr(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShortAddr(u16);
