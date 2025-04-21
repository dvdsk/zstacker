use std::iter;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;
use crate::framing::CommandMeta;

pub const START_OF_FRAME: u8 = 0xFE;

mod command_types;
pub use command_types::{
    AsyncNotify, AsyncReply, AsyncRequest, SyncReply, SyncRequest,
};

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

fn from_reader_inner<R: SyncReply>(
    reader: &mut impl std::io::Read,
) -> Result<R, ReplyErrorCause> {
    use ReplyErrorCause as E;
    let mut reply_header = [0u8; 4];
    reader
        .read_exact(&mut reply_header)
        .map_err(E::ReadingHeader)?;

    let [START_OF_FRAME, length, cmd0, cmd1] = reply_header else {
        return Err(E::ExpectedStartOfFrame);
    };
    let info = CommandMeta::deserialize([cmd0, cmd1]).unwrap();
    if info.ty != CommandType::SRSP {
        return Err(E::ExpectedSynchrousReply(info.ty));
    } else if info.sub_system != R::Request::SUBSYSTEM {
        return Err(E::WrongSubSystem {
            expected: R::Request::SUBSYSTEM,
            got: info.sub_system,
        });
    } else if info.id != R::Request::ID {
        return Err(E::WrongId {
            expected: R::Request::ID,
            got: info.id,
        });
    }

    let mut buf = vec![0u8; length as usize + 1];
    reader.read_exact(&mut buf).map_err(E::ReadingData)?;
    split_off_and_verify_checksum::<R>(length, &mut buf)?;

    use itertools::Itertools;
    tracing::trace!(
        "data: [{}]",
        buf.iter().map(|byte| format!("{byte:x}")).join(",")
    );

    let mut data = std::io::Cursor::new(buf);
    let reply = data_format::from_reader(&mut data).map_err(E::Deserialize)?;

    Ok(reply)
}

fn split_off_and_verify_checksum<R: SyncReply>(
    length: u8,
    buf: &mut Vec<u8>,
) -> Result<(), ReplyErrorCause> {
    let expected_checksum =
        buf.pop().expect("read_exact only Ok if the buffer is full");
    let expected_meta = R::META.serialize();
    let frame = iter::once(length)
        .chain(expected_meta)
        .chain(buf.iter().copied());
    let calculated_checksum = frame
        .reduce(|checksum, byte| checksum ^ byte)
        .expect("never empty");
    if expected_checksum != calculated_checksum {
        return Err(ReplyErrorCause::CheckSumMismatch);
    }
    Ok(())
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

// /// See: Z-Stack Monitor and Test API section 3.12.2.16 revision 1.14
// /// Note this is not serialized like this in the wire format
// /// Can not use serde to get this over the wire. Overwrite the default
// /// `Command::data_to_vec` implementation instead.
// #[derive(Debug, Clone)]
// struct NeighborLqi {
//     /// Extended PAN ID of the neighbor device
//     extended_pan_id: u64,
//     /// Network extended address
//     extended_address: IeeeAddr,
//     network_address: ShortAddr,
//     device_type: DeviceType, // 19th byte bits 1-0
//     rx_on_when_idle: bool,   // 19th byte bits 3-2
//     relationship: u8,        // 19th byte bits 6-4
//     permit_joining: u8,      // 20th byte bits 1-0
//     depth: u8,
//     lqi: u8,
// }

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
