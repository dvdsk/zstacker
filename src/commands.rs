use std::iter;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;
use crate::framing::CommandInfo;

pub const START_OF_FRAME: u8 = 0xFE;

mod command_types;
pub use command_types::{AsyncReply, AsyncRequest, SyncReply, SyncRequest};

pub mod af;
pub mod app;
pub mod appconfig;
pub mod debug;
pub mod greenpower;
pub mod mac;
pub mod sapi;
pub mod sys;
pub mod util;
// most of these are wrong
// pub(crate) mod zdo;

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
    #[error("Second byte of package should be {expected} however we got {got}")]
    WrongCmd1Field { expected: u8, got: u8 },
    #[error("Third byte of package should be {expected} however we got {got}")]
    WrongCmd0Field { expected: u8, got: u8 },
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

    // TODO construct CMD0 and CMD1 from org
    // then probably do away with this again :)
    // and absorb it in Command
    let [START_OF_FRAME, length, cmd0, cmd1] = reply_header else {
        return Err(E::ExpectedStartOfFrame);
    };
    let info = CommandInfo::deserialize([cmd0, cmd1]).unwrap();
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
    tracing::debug!(
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

/// The status parameter that is returned from the `ZNP` device
///
/// From: `Z-Stack ZNP Interface Specification.pdf` revision 1.1 (11/11/2016)
/// url: https://community.silabs.com/s/contentversion/0681M00000EWPKrQAP
/// viewed: 2025-04-15
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum Status {
    ZSuccess = 0x00,
    Zfailure = 0x01,
    ZinvalidParameter = 0x02,
    NvItemUninit = 0x09,
    NvOperFailed = 0x0a,
    NvBadItemLen = 0x0c,
    ZmemError = 0x10,
    ZbufferFull = 0x11,
    ZunsupportedMode = 0x12,
    ZmacMemError = 0x13,
    ZdoInvalidRequestType = 0x80,
    ZdoInvalidEndpoint = 0x82,
    ZdoUnsupported = 0x84,
    ZdoTimeout = 0x85,
    ZdoNoMatch = 0x86,
    ZdoTableFull = 0x87,
    ZdoNoBindEntry = 0x88,
    ZsecNoKey = 0xa1,
    ZsecMaxFrmCount = 0xa3,
    ZapsFail = 0xb1,
    ZapsTableFull = 0xb2,
    ZapsIllegalRequest = 0xb3,
    ZapsInvalidBinding = 0xb4,
    ZapsUnsupportedAttrib = 0xb5,
    ZapsNotSupported = 0xb6,
    ZapsNoAck = 0xb7,
    ZapsDuplicateEntry = 0xb8,
    ZapsNoBoundDevice = 0xb9,
    ZnwkInvalidParam = 0xc1,
    ZnwkInvalidRequest = 0xc2,
    ZnwkNotPermitted = 0xc3,
    ZnwkStartupFailure = 0xc4,
    ZnwkTableFull = 0xc7,
    ZnwkUnknownDevice = 0xc8,
    ZnwkUnsupportedAttribute = 0xc9,
    ZnwkNoNetworks = 0xca,
    ZnwkLeaveUnconfirmed = 0xcb,
    ZnwkNoAck = 0xcc,
    ZnwkNoRoute = 0xcd,
    ZMacNoACK = 0xe9,
}

#[derive(Clone, Copy, Debug, strum::FromRepr, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, strum::FromRepr, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, strum::EnumIter)]
#[repr(u8)]
pub enum DeviceType {
    None = 0,
    Coordinator = 1,
    Router = 2,
    EndDevice = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IeeeAddr(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortAddr(u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
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
