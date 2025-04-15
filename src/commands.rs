use std::iter;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::data_format;

pub const START_OF_FRAME: u8 = 0xFE;

// TODO set subsystem for all of these
pub(crate) mod sys;
pub(crate) mod mac;
pub(crate) mod af;
pub(crate) mod zdo;
pub(crate) mod sapi;
pub(crate) mod util;
pub(crate) mod debug;
pub(crate) mod app;
pub(crate) mod appconfig;
pub(crate) mod greenpower;

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
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
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

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Endpoint(u8);

struct ProfileAndVersion {
    stack_profile: u8,
    zigbee_version: u8,
}

/// See: Z-Stack Monitor and Test API section 3.12.2.15 revision 1.14
/// Note this is not serialized like this in the wire format
/// Can not use serde to get this over the wire. Overwrite the default
/// `Command::data_to_vec` implementation instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Network {
    // PAN ID of the neighbor device
    pan_id: u16,
    // The current logical channel occupied by the network.
    logical_channel: u8,
    stack_profile: u8,     // 4th byte bits 3-0
    zigbee_version: u8,    // 4th byte bits 7-4
    beacon_order: u8,      // 4th byte bits 3-0
    super_frame_order: u8, // pth byte bits 7-4
    permit_joining: bool,
}

/// See: Z-Stack Monitor and Test API section 3.12.2.16 revision 1.14
/// Note this is not serialized like this in the wire format
/// Can not use serde to get this over the wire. Overwrite the default
/// `Command::data_to_vec` implementation instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NeighborLqi {
    /// Extended PAN ID of the neighbor device
    extended_pan_id: u64,
    /// Network extended address
    extended_address: IeeeAddr,
    network_address: ShortAddr,
    device_type: DeviceType, // 19th byte bits 1-0
    rx_on_when_idle: bool,   // 19th byte bits 3-2
    relationship: u8,        // 19th byte bits 6-4
    permit_joining: u8,      // 20th byte bits 1-0
    depth: u8,
    lqi: u8,
}

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
