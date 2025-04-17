#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{AsyncRequest, SyncRequest, SyncReply, IeeeAddr, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ResetReq {
    ty: u8,
}

impl AsyncRequest for ResetReq {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Ping;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PingReply {
    #[serde(deserialize_with = "capabilities_from_u16")]
    pub(crate) capabilities: Vec<Capability>,
}

impl SyncReply for PingReply {
    const CMD0: u8 = 0x61; // placeholder
    const CMD1: u8 = 0x01; // placeholder
}

impl SyncRequest for Ping {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = PingReply;
}

/// Defines which parts of the API are supported by the device.
/// These correspond to the modules in [`crate::commands`].
#[derive(Debug, Clone, Copy, strum::EnumIter)]
#[repr(u16)]
pub enum Capability {
    /// Can interact at system level such as reset,
    /// read/write memory, read/write extended address, etc.
    Sys = 0x0001,
    Mac = 0x0002,
    /// Control NWK layer.
    Nwk = 0x0004,
    /// Control application framework.
    Af = 0x0008,
    Zdo = 0x0010,
    /// Control using the simple API
    Sapi = 0x0020,
    Util = 0x0040,
    Debug = 0x0080,
    App = 0x0100,
    Zoad = 0x1000,
}

fn capabilities_from_u16<'de, D>(
    deserializer: D,
) -> Result<Vec<Capability>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use strum::IntoEnumIterator;
    let bitset: u16 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(Capability::iter()
        .filter(|cap| bitset & (*cap as u16) > 0)
        .collect())
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Version;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VersionReply {
    transportrev: u8,
    product: u8,
    majorrel: u8,
    minorrel: u8,
    maintrel: u8,
    revision: u32,
}

impl SyncReply for VersionReply {
    const CMD0: u8 = 0x21;
    const CMD1: u8 = 0x02;
}

impl SyncRequest for Version {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = VersionReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SetExtAddr {
    extaddress: IeeeAddr,
}

impl SyncRequest for SetExtAddr {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GetExtAddr;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GetExtAddrReply {
    extaddress: IeeeAddr,
}

impl SyncReply for GetExtAddrReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for GetExtAddr {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GetExtAddrReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RamRead {
    address: u16,
    len: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RamReadReply {
    status: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncReply for RamReadReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for RamRead {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = RamReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RamWrite {
    address: u16,
    len: u8,
    value: Vec<u8>,
}

impl SyncRequest for RamWrite {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvItemInit {
    id: u16,
    len: u16,
    initlen: u8,
    initvalue: Vec<u8>,
}

impl SyncRequest for OsalNvItemInit {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvRead {
    id: u16,
    offset: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct OsalNvReadReply {
    status: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncReply for OsalNvReadReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for OsalNvRead {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvWrite {
    id: u16,
    offset: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncRequest for OsalNvWrite {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalStartTimer {
    id: u8,
    timeout: u16,
}

impl SyncRequest for OsalStartTimer {
    const ID: u8 = 10;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalStopTimer {
    id: u8,
}

impl SyncRequest for OsalStopTimer {
    const ID: u8 = 11;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Random;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RandomReply {
    value: u16,
}

impl SyncReply for RandomReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for Random {
    const ID: u8 = 12;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = RandomReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AdcRead {
    channel: u8,
    resolution: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AdcReadReply {
    value: u16,
}

impl SyncReply for AdcReadReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for AdcRead {
    const ID: u8 = 13;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = AdcReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Gpio {
    operation: u8,
    value: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GpioReply {
    value: u8,
}

impl SyncReply for GpioReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for Gpio {
    const ID: u8 = 14;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GpioReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct StackTune {
    operation: u8,
    value: i8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StackTuneReply {
    value: u8,
}

impl SyncReply for StackTuneReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for StackTune {
    const ID: u8 = 15;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = StackTuneReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SetTime {
    utc: u32,
    hour: u8,
    minute: u8,
    second: u8,
    month: u8,
    day: u8,
    year: u16,
}

impl SyncRequest for SetTime {
    const ID: u8 = 16;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GetTime;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GetTimeReply {
    utc: u32,
    hour: u8,
    minute: u8,
    second: u8,
    month: u8,
    day: u8,
    year: u16,
}

impl SyncReply for GetTimeReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for GetTime {
    const ID: u8 = 17;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GetTimeReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvDelete {
    id: u16,
    len: u16,
}

impl SyncRequest for OsalNvDelete {
    const ID: u8 = 18;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvLength {
    id: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct OsalNvLengthReply {
    length: u16,
}

impl SyncReply for OsalNvLengthReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for OsalNvLength {
    const ID: u8 = 19;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SetTxPower {
    level: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SetTxPowerReply {
    txpower: u8,
}

impl SyncReply for SetTxPowerReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for SetTxPower {
    const ID: u8 = 20;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = SetTxPowerReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct JammerParameters {
    jmrcntievents: u16,
    jmrhinoiselvl: u8,
    jmrdetectperiod: u32,
}

impl SyncRequest for JammerParameters {
    const ID: u8 = 21;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct SnifferParameters {
    param: u8,
}

impl SyncRequest for SnifferParameters {
    const ID: u8 = 22;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ZdiagsInitStats;

impl SyncRequest for ZdiagsInitStats {
    const ID: u8 = 23;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ZdiagsClearStats {
    clearnv: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ZdiagsClearStatsReply {
    sysclock: u32,
}

impl SyncReply for ZdiagsClearStatsReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ZdiagsClearStats {
    const ID: u8 = 24;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsClearStatsReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ZdiagsGetStats {
    attributeid: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ZdiagsGetStatsReply {
    attributevalue: u32,
}

impl SyncReply for ZdiagsGetStatsReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ZdiagsGetStats {
    const ID: u8 = 25;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsGetStatsReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ZdiagsRestoreStatsNv;

impl SyncRequest for ZdiagsRestoreStatsNv {
    const ID: u8 = 26;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ZdiagsSaveStatsToNv;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ZdiagsSaveStatsToNvReply {
    sysclock: u32,
}

impl SyncReply for ZdiagsSaveStatsToNvReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ZdiagsSaveStatsToNv {
    const ID: u8 = 27;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsSaveStatsToNvReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvReadExt {
    id: u16,
    offset: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct OsalNvReadExtReply {
    status: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncReply for OsalNvReadExtReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for OsalNvReadExt {
    const ID: u8 = 28;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvReadExtReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalNvWriteExt {
    id: u16,
    offset: u16,
    len: u16,
    value: Vec<u8>,
}

impl SyncRequest for OsalNvWriteExt {
    const ID: u8 = 29;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvCreate {
    sysid: u8,
    itemid: u16,
    subid: u16,
    len: u32,
}

impl SyncRequest for NvCreate {
    const ID: u8 = 48;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvDelete {
    sysid: u8,
    itemid: u16,
    subid: u16,
}

impl SyncRequest for NvDelete {
    const ID: u8 = 49;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvLength {
    sysid: u8,
    itemid: u16,
    subid: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NvLengthReply {
    len: u8,
}

impl SyncReply for NvLengthReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for NvLength {
    const ID: u8 = 50;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvRead {
    sysid: u8,
    itemid: u16,
    subid: u16,
    offset: u16,
    len: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NvReadReply {
    status: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncReply for NvReadReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for NvRead {
    const ID: u8 = 51;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvWrite {
    sysid: u8,
    itemid: u16,
    subid: u16,
    offset: u16,
    len: u8,
    value: Vec<u8>,
}

impl SyncRequest for NvWrite {
    const ID: u8 = 52;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvUpdate {
    sysid: u8,
    itemid: u16,
    subid: u16,
    len: u8,
    value: Vec<u8>,
}

impl SyncRequest for NvUpdate {
    const ID: u8 = 53;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct NvCompact {
    threshold: u16,
}

impl SyncRequest for NvCompact {
    const ID: u8 = 54;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ResetInd {
    reason: u8,
    transportrev: u8,
    productid: u8,
    majorrel: u8,
    minorrel: u8,
    hwrev: u8,
}

impl AsyncRequest for ResetInd {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct OsalTimerExpired {
    id: u8,
}

impl AsyncRequest for OsalTimerExpired {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct JammerInd {
    jammerind: u8,
}

impl AsyncRequest for JammerInd {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}
