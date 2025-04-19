use super::{
    basic_reply, AsyncRequest, IeeeAddr, SubSystem, SyncReply, SyncRequest
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ResetReq {
    pub ty: u8,
}

impl AsyncRequest for ResetReq {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub struct Ping;

#[derive(Debug, Clone, Deserialize)]
pub struct PingReply {
    #[serde(deserialize_with = "capabilities_from_u16")]
    pub capabilities: Vec<Capability>,
}

impl SyncReply for PingReply {
    type Request = Ping;
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
pub struct Version;

#[derive(Debug, Clone, Deserialize)]
pub struct VersionReply {
    pub transportrev: u8,
    pub product: u8,
    pub majorrel: u8,
    pub minorrel: u8,
    pub maintrel: u8,
    pub revision: u32,
}

impl SyncReply for VersionReply {
    type Request = Version;
}

impl SyncRequest for Version {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = VersionReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct SetExtAddr {
    pub extaddress: IeeeAddr,
}

impl SyncRequest for SetExtAddr {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = SetExtAddrReply;
}

basic_reply! { SetExtAddr, SetExtAddrReply }

#[derive(Debug, Clone, Serialize)]
pub struct GetExtAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct GetExtAddrReply {
    pub extaddress: IeeeAddr,
}

impl SyncReply for GetExtAddrReply {
    type Request = GetExtAddr;
}

impl SyncRequest for GetExtAddr {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GetExtAddrReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct RamRead {
    pub address: u16,
    pub len: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RamReadReply {
    pub status: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncReply for RamReadReply {
    type Request = RamRead;
}

impl SyncRequest for RamRead {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = RamReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct RamWrite {
    pub address: u16,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncRequest for RamWrite {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = RamWriteReply;
}

basic_reply! { RamWrite, RamWriteReply }

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvItemInit {
    pub id: u16,
    pub len: u16,
    pub initlen: u8,
    pub initvalue: Vec<u8>,
}

impl SyncRequest for OsalNvItemInit {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvItemInitReply;
}

basic_reply! { OsalNvItemInit, OsalNvItemInitReply }

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvRead {
    pub id: u16,
    pub offset: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OsalNvReadReply {
    pub status: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncReply for OsalNvReadReply {
    type Request = OsalNvRead;
}

impl SyncRequest for OsalNvRead {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvWrite {
    pub id: u16,
    pub offset: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncRequest for OsalNvWrite {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvWriteReply;
}

basic_reply! { OsalNvWrite, OsalNvWriteReply }

#[derive(Debug, Clone, Serialize)]
pub struct OsalStartTimer {
    pub id: u8,
    pub timeout: u16,
}

impl SyncRequest for OsalStartTimer {
    const ID: u8 = 10;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalStartTimerReply;
}

basic_reply! { OsalStartTimer, OsalStartTimerReply }

#[derive(Debug, Clone, Serialize)]
pub struct OsalStopTimer {
    pub id: u8,
}

impl SyncRequest for OsalStopTimer {
    const ID: u8 = 11;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalStopTimerReply;
}

basic_reply! { OsalStopTimer, OsalStopTimerReply }

#[derive(Debug, Clone, Serialize)]
pub struct Random;

#[derive(Debug, Clone, Deserialize)]
pub struct RandomReply {
    pub value: u16,
}

impl SyncReply for RandomReply {
    type Request = Random;
}

impl SyncRequest for Random {
    const ID: u8 = 12;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = RandomReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct AdcRead {
    pub channel: u8,
    pub resolution: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdcReadReply {
    pub value: u16,
}

impl SyncReply for AdcReadReply {
    type Request = AdcRead;
}

impl SyncRequest for AdcRead {
    const ID: u8 = 13;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = AdcReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct Gpio {
    pub operation: u8,
    pub value: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GpioReply {
    pub value: u8,
}

impl SyncReply for GpioReply {
    type Request = Gpio;
}

impl SyncRequest for Gpio {
    const ID: u8 = 14;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GpioReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct StackTune {
    pub operation: u8,
    pub value: i8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StackTuneReply {
    pub value: u8,
}

impl SyncReply for StackTuneReply {
    type Request = StackTune;
}

impl SyncRequest for StackTune {
    const ID: u8 = 15;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = StackTuneReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct SetTime {
    pub utc: u32,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl SyncRequest for SetTime {
    const ID: u8 = 16;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = SetTimeReply;
}

basic_reply! { SetTime, SetTimeReply }

#[derive(Debug, Clone, Serialize)]
pub struct GetTime;

#[derive(Debug, Clone, Deserialize)]
pub struct GetTimeReply {
    pub utc: u32,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub month: u8,
    pub day: u8,
    pub year: u16,
}

impl SyncReply for GetTimeReply {
    type Request = GetTime;
}

impl SyncRequest for GetTime {
    const ID: u8 = 17;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = GetTimeReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvDelete {
    pub id: u16,
    pub len: u16,
}

impl SyncRequest for OsalNvDelete {
    const ID: u8 = 18;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvDeleteReply;
}

basic_reply! { OsalNvDelete, OsalNvDeleteReply }

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvLength {
    pub id: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OsalNvLengthReply {
    pub length: u16,
}

impl SyncReply for OsalNvLengthReply {
    type Request = OsalNvLength;
}

impl SyncRequest for OsalNvLength {
    const ID: u8 = 19;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct SetTxPower {
    pub level: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetTxPowerReply {
    pub txpower: u8,
}

impl SyncReply for SetTxPowerReply {
    type Request = SetTxPower;
}

impl SyncRequest for SetTxPower {
    const ID: u8 = 20;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = SetTxPowerReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct JammerParameters {
    pub jmrcntievents: u16,
    pub jmrhinoiselvl: u8,
    pub jmrdetectperiod: u32,
}

impl SyncRequest for JammerParameters {
    const ID: u8 = 21;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = JammerParametersReply;
}

basic_reply! { JammerParameters, JammerParametersReply }

#[derive(Debug, Clone, Serialize)]
pub struct SnifferParameters {
    pub param: u8,
}

impl SyncRequest for SnifferParameters {
    const ID: u8 = 22;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = SnifferParametersReply;
}

basic_reply! { SnifferParameters, SnifferParametersReply }

#[derive(Debug, Clone, Serialize)]
pub struct ZdiagsInitStats;

impl SyncRequest for ZdiagsInitStats {
    const ID: u8 = 23;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsInitStatsReply;
}

basic_reply! { ZdiagsInitStats, ZdiagsInitStatsReply }

#[derive(Debug, Clone, Serialize)]
pub struct ZdiagsClearStats {
    pub clearnv: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZdiagsClearStatsReply {
    pub sysclock: u32,
}

impl SyncReply for ZdiagsClearStatsReply {
    type Request = ZdiagsClearStats;
}

impl SyncRequest for ZdiagsClearStats {
    const ID: u8 = 24;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsClearStatsReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct ZdiagsGetStats {
    pub attributeid: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZdiagsGetStatsReply {
    pub attributevalue: u32,
}

impl SyncReply for ZdiagsGetStatsReply {
    type Request = ZdiagsGetStats;
}

impl SyncRequest for ZdiagsGetStats {
    const ID: u8 = 25;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsGetStatsReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct ZdiagsRestoreStatsNv;

impl SyncRequest for ZdiagsRestoreStatsNv {
    const ID: u8 = 26;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsRestoreStatsNvReply;
}

basic_reply! { ZdiagsRestoreStatsNv, ZdiagsRestoreStatsNvReply }

#[derive(Debug, Clone, Serialize)]
pub struct ZdiagsSaveStatsToNv;

#[derive(Debug, Clone, Deserialize)]
pub struct ZdiagsSaveStatsToNvReply {
    pub sysclock: u32,
}

impl SyncReply for ZdiagsSaveStatsToNvReply {
    type Request = ZdiagsSaveStatsToNv;
}

impl SyncRequest for ZdiagsSaveStatsToNv {
    const ID: u8 = 27;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = ZdiagsSaveStatsToNvReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvReadExt {
    pub id: u16,
    pub offset: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OsalNvReadExtReply {
    pub status: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncReply for OsalNvReadExtReply {
    type Request = OsalNvReadExt;
}

impl SyncRequest for OsalNvReadExt {
    const ID: u8 = 28;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvReadExtReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct OsalNvWriteExt {
    pub id: u16,
    pub offset: u16,
    pub len: u16,
    pub value: Vec<u8>,
}

impl SyncRequest for OsalNvWriteExt {
    const ID: u8 = 29;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = OsalNvWriteExtReply;
}

basic_reply! { OsalNvWriteExt, OsalNvWriteExtReply }

#[derive(Debug, Clone, Serialize)]
pub struct NvCreate {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
    pub len: u32,
}

impl SyncRequest for NvCreate {
    const ID: u8 = 48;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvCreateReply;
}

basic_reply! { NvCreate, NvCreateReply }

#[derive(Debug, Clone, Serialize)]
pub struct NvDelete {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
}

impl SyncRequest for NvDelete {
    const ID: u8 = 49;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvDeleteReply;
}

basic_reply! { NvDelete, NvDeleteReply }

#[derive(Debug, Clone, Serialize)]
pub struct NvLength {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NvLengthReply {
    pub len: u8,
}

impl SyncReply for NvLengthReply {
    type Request = NvLength;
}

impl SyncRequest for NvLength {
    const ID: u8 = 50;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct NvRead {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
    pub offset: u16,
    pub len: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NvReadReply {
    pub status: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncReply for NvReadReply {
    type Request = NvRead;
}

impl SyncRequest for NvRead {
    const ID: u8 = 51;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvReadReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct NvWrite {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
    pub offset: u16,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncRequest for NvWrite {
    const ID: u8 = 52;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvWriteReply;
}

basic_reply! { NvWrite, NvWriteReply }

#[derive(Debug, Clone, Serialize)]
pub struct NvUpdate {
    pub sysid: u8,
    pub itemid: u16,
    pub subid: u16,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncRequest for NvUpdate {
    const ID: u8 = 53;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvUpdateReply;
}

basic_reply! { NvUpdate, NvUpdateReply }

#[derive(Debug, Clone, Serialize)]
pub struct NvCompact {
    pub threshold: u16,
}

impl SyncRequest for NvCompact {
    const ID: u8 = 54;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
    type Reply = NvCompactReply;
}

basic_reply! { NvCompact, NvCompactReply }

#[derive(Debug, Clone, Serialize)]
pub struct ResetInd {
    pub reason: u8,
    pub transportrev: u8,
    pub productid: u8,
    pub majorrel: u8,
    pub minorrel: u8,
    pub hwrev: u8,
}

impl AsyncRequest for ResetInd {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub struct OsalTimerExpired {
    pub id: u8,
}

impl AsyncRequest for OsalTimerExpired {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}

#[derive(Debug, Clone, Serialize)]
pub struct JammerInd {
    pub jammerind: u8,
}

impl AsyncRequest for JammerInd {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Sys;
}
