use std::iter;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;

pub const START_OF_FRAME: u8 = 0xFE;

pub(crate) mod af;
pub(crate) mod app;
pub(crate) mod appconfig;
pub(crate) mod debug;
pub(crate) mod greenpower;
pub(crate) mod mac;
pub(crate) mod sapi;
pub(crate) mod sys;
pub(crate) mod util;
pub(crate) mod zdo;

#[derive(Debug, thiserror::Error)]
#[error("Could not send command: {command}")]
pub struct CommandError {
    command: &'static str,
    #[source]
    cause: data_format::Error,
}

pub trait Command: Serialize {
    const ID: u8;
    const TYPE: CommandType;
    const SUBSYSTEM: Subsystem;
    type Reply: CommandReply;

    fn data_to_vec(&self) -> Result<Vec<u8>, data_format::Error>
    where
        Self: Sized,
    {
        data_format::to_vec(self)
    }

    fn to_frame(&self) -> Result<Vec<u8>, CommandError>
    where
        Self: Sized,
    {
        let serialized = self.data_to_vec().map_err(|error| CommandError {
            command: std::any::type_name::<Self>(),
            cause: error,
        })?;
        let frame = [
            serialized.len() as u8,
            Self::SUBSYSTEM as u8 | Self::TYPE as u8,
            Self::ID,
        ]
        .into_iter()
        .chain(serialized);

        let checksum = frame
            .clone()
            .reduce(|checksum, byte| checksum ^ byte)
            .expect("never empty");

        Ok(iter::once(START_OF_FRAME)
            .chain(frame)
            .chain([checksum])
            .collect())
    }
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
}

fn from_reader_inner<R: CommandReply>(
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
    let [start_of_frame, length, cmd0, cmd1] = reply_header;
    if start_of_frame != START_OF_FRAME {
        return Err(E::ExpectedStartOfFrame);
    } else if cmd0 != R::CMD0 {
        return Err(E::WrongCmd0Field {
            expected: R::CMD1,
            got: cmd0,
        });
    } else if cmd1 != R::CMD1 {
        return Err(E::WrongCmd1Field {
            expected: R::CMD1,
            got: cmd1,
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

fn split_off_and_verify_checksum<R: CommandReply>(
    length: u8,
    buf: &mut Vec<u8>,
) -> Result<(), ReplyErrorCause> {
    let expected_checksum =
        buf.pop().expect("read_exact only Ok if the buffer is full");
    let frame = [length, R::CMD0, R::CMD1]
        .into_iter()
        .chain(buf.iter().copied());
    let calculated_checksum = frame
        .reduce(|checksum, byte| checksum ^ byte)
        .expect("never empty");
    if expected_checksum != calculated_checksum {
        return Err(ReplyErrorCause::CheckSumMismatch);
    }
    Ok(())
}

pub trait CommandReply: DeserializeOwned {
    const CMD0: u8;
    const CMD1: u8;

    fn from_reader(
        reader: &mut impl std::io::Read,
    ) -> Result<Self, ReplyError> {
        from_reader_inner(reader).map_err(|cause| ReplyError {
            reply: std::any::type_name::<Self>(),
            cause,
        })
    }
}

impl CommandReply for () {
    const CMD0: u8 = 0;
    const CMD1: u8 = 0;

    fn from_reader(_: &mut impl std::io::Read) -> Result<Self, ReplyError> {
        Ok(())
    }
}

impl CommandReply for Status {
    const CMD0: u8 = 0x67;
    const CMD1: u8 = 0x23;
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

#[derive(Clone, Copy, Debug, Serialize_repr)]
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

#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum Subsystem {
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
struct IeeeAddr(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShortAddr(u16);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Endpoint(u8);

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
enum RouterStatus {
    Active = 0,
    DiscoveryUnderway = 1,
    DiscoveryFailed = 2,
    Inactive = 3,
}

/// See: Z-Stack Monitor and Test API section 3.12.2.17 revision 1.14
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoutingTable {
    destination_address: ShortAddr,
    status: RouterStatus,
    next_hop: ShortAddr,
}

/// See Z-Stack Monitor and Test API section 3.12.2.18 revision 1.14
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BindTable {
    src: IeeeAddr,
    src_endpoint: Endpoint,
    cluster_id: u8,
    dst_addr_mode: u8,
    dst: IeeeAddr,
    dst_endpoint: Endpoint,
}
