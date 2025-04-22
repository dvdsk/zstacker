use std::iter;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;

pub const START_OF_FRAME: u8 = 0xFE;

mod command_types;
pub use command_types::{
    AsyncNotify, AsyncReply, AsyncRequest, SyncReply, SyncRequest,
};

#[cfg(feature = "mocking")]
pub use command_types::to_frame;

pub mod af;
pub mod app;
pub mod appconfig;
pub mod debug;
pub mod greenpower;
pub mod mac;
pub mod sapi;
pub mod sys;
pub mod util;
pub mod zdo;

#[derive(Debug, Clone, Copy, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum BasicStatus {
    Ok = 0,
    Err = 1,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum PatternElement {
    NeedsToMatchExact(u8),
    #[default]
    NeedsNotMatch,
}

impl PatternElement {
    fn matches(&self, byte: u8) -> bool {
        match self {
            Self::NeedsToMatchExact(val) => byte == *val,
            Self::NeedsNotMatch => true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Pattern(Vec<PatternElement>);

impl Pattern {
    pub fn matches(&self, data: &[u8]) -> bool {
        let mut pattern = self.0.iter();

        for (matcher, byte) in pattern.by_ref().zip(data) {
            if !matcher.matches(*byte) {
                return false;
            }
        }

        pattern.next().is_none()
    }

    pub(crate) fn match_exact<T: Serialize>(mut self, val: &T) -> Self {
        self.0.extend(
            data_format::to_vec(val)
                .expect(
                    "only usable in crate, we only use it for commands, \
                    they should serialize",
                )
                .into_iter()
                .map(PatternElement::NeedsToMatchExact),
        );
        self
    }
    pub(crate) fn skip(mut self, n: usize) -> Self {
        self.0
            .extend(iter::repeat_n(PatternElement::NeedsNotMatch, n));
        self
    }
}

macro_rules! basic_reply {
    ($request_name:ident, $reply_name:ident) => {
        #[derive(Debug, Clone, Deserialize)]
        pub struct $reply_name(crate::commands::BasicStatus);

        impl $reply_name {
            pub fn is_ok(&self) -> bool {
                match self.0 {
                    crate::commands::BasicStatus::Ok => true,
                    crate::commands::BasicStatus::Err => false,
                }
            }
            pub fn is_err(&self) -> bool {
                !self.is_ok()
            }
            pub fn map_err<E>(&self, err: E) -> Result<(), E> {
                Err(err)
            }
        }

        impl SyncReply for $reply_name {
            type Request = $request_name;
        }
    };
}
pub(crate) use basic_reply;

#[expect(unused_macros, reason = "requests that use this are commented out")]
macro_rules! empty_reply {
    ($request_name:ident, $reply_name:ident) => {
        #[derive(Debug, Clone, Deserialize)]
        pub struct $reply_name;

        impl SyncReply for $reply_name {
            type Request = $request_name;
        }
    };
}
#[expect(
    unused_imports,
    reason = "request that use this are commented out"
)]
pub(crate) use empty_reply;

#[derive(Debug, thiserror::Error)]
#[error("Could not send command: {command}")]
pub struct CommandError {
    command: &'static str,
    #[source]
    cause: data_format::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to read reply ({reply}) to command")]
pub struct ReplyError {
    reply: &'static str,
    #[source]
    cause: ReplyErrorCause,
}

#[derive(Debug, thiserror::Error)]
pub enum ReplyErrorCause {
    #[error("Could not deserialize reply data")]
    Deserialize(#[source] data_format::Error),
    #[error("Could not read reply header (4 bytes) from serial")]
    ReadingHeader(#[source] std::io::Error),
    #[error("Could not read reply data from serial")]
    ReadingData(#[source] std::io::Error),
    #[error("First byte of reply should have been: {START_OF_FRAME}")]
    ExpectedStartOfFrame,
    #[error("Could not read checksum")]
    ReadingChecksum(#[source] std::io::Error),
    #[error("Checksum is not correct")]
    CheckSumMismatch,
    #[error("Expected a synchronous reply, got: {0:?}")]
    ExpectedSynchrousReply(CommandType),
    #[error(
        "Expected a reply for subsystem: {expected:?} however got one for {got:?}"
    )]
    WrongSubSystem { expected: SubSystem, got: SubSystem },
    #[error(
        "Expected a reply for command: {expected} however got one for {got}"
    )]
    WrongId { expected: u8, got: u8 },
}

#[derive(Clone, Copy, Debug, strum::FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CommandType {
    /// A POLL command is used to retrieve queued data. Third command is only
    /// applicable to `SPI` transport. For a `POLL` command the subsystem and
    /// Id are set to zero and data length is zero.
    POLL = 0x00,
    /// A synchronous request that requires an immediate response. For example,
    /// a function call with a return value would use an `SREQ` command.
    SREQ = 0x20,
    /// AREQ: An asynchronous request. For example, a callback event or a
    /// function call with no return value would use an `AREQ` command.
    AREQ = 0x40,
    /// A synchronous response. This type of command is only sent in response
    /// to an `SREQ` command. For an `SRSP` command the subsystem and Id are set to
    /// the same values as the corresponding `SREQ`. The length of an `SRSP` is
    /// generally nonzero, so an `SRSP` with length=0 can be used to indicate an
    /// error.
    SRSP = 0x60,
}

#[derive(Clone, Copy, Debug, strum::FromRepr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubSystem {
    Reserved = 0x00,
    Sys = 0x01,
    /// Media access control
    Mac = 0x02,
    /// Network layer
    Nwk = 0x03,
    /// Application framework
    Af = 0x04,
    Zdo = 0x05,
    Sapi = 0x06,
    Util = 0x07,
    Debug = 0x08,
    App = 0x09,
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

#[derive(Clone, Copy, Debug, strum::EnumIter, strum::FromRepr)]
#[repr(u8)]
pub enum DeviceType {
    None = 0,
    Coordinator = 1,
    Router = 2,
    EndDevice = 3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IeeeAddr(u64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ShortAddr(u16);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Endpoint(u8);

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RouterStatus {
    Active = 0,
    DiscoveryUnderway = 1,
    DiscoveryFailed = 2,
    Inactive = 3,
}

/// See: Z-Stack Monitor and Test API section 3.12.2.17 revision 1.14
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTable {
    pub destination_address: ShortAddr,
    pub status: RouterStatus,
    pub next_hop: ShortAddr,
}

/// See Z-Stack Monitor and Test API section 3.12.2.18 revision 1.14
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindTable {
    pub src: IeeeAddr,
    pub src_endpoint: Endpoint,
    pub cluster_id: u8,
    pub dst_addr_mode: u8,
    pub dst: IeeeAddr,
    pub dst_endpoint: Endpoint,
}
