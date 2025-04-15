#![allow(dead_code)]
use serde::{Serialize, Deserialize};

use super::{Command, CommandType, Status, Subsystem, IeeeAddr};

#[derive(Debug, Clone, Serialize)]
struct ResetReq {
	ty: u8,
}

impl Command for ResetReq {
	const ID: u8 = 0;
	const TYPE: CommandType = CommandType::AREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct Ping {
}

#[derive(Debug, Clone, Deserialize)]
struct PingReply {
	capabilities: u16,
}

impl Command for Ping {
	const ID: u8 = 1;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = PingReply;
}

#[derive(Debug, Clone, Serialize)]
struct Version {
}

#[derive(Debug, Clone, Deserialize)]
struct VersionReply {
	transportrev: u8,
	product: u8,
	majorrel: u8,
	minorrel: u8,
	maintrel: u8,
	revision: u32,
}

impl Command for Version {
	const ID: u8 = 2;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = VersionReply;
}

#[derive(Debug, Clone, Serialize)]
struct SetExtAddr {
	extaddress: IeeeAddr,
}

impl Command for SetExtAddr {
	const ID: u8 = 3;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct GetExtAddr {
}

#[derive(Debug, Clone, Deserialize)]
struct GetExtAddrReply {
	extaddress: IeeeAddr,
}

impl Command for GetExtAddr {
	const ID: u8 = 4;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = GetExtAddrReply;
}

#[derive(Debug, Clone, Serialize)]
struct RamRead {
	address: u16,
	len: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct RamReadReply {
	status: u8,
	len: u8,
	value: Vec<u8>,
}

impl Command for RamRead {
	const ID: u8 = 5;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = RamReadReply;
}

#[derive(Debug, Clone, Serialize)]
struct RamWrite {
	address: u16,
	len: u8,
	value: Vec<u8>,
}

impl Command for RamWrite {
	const ID: u8 = 6;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvItemInit {
	id: u16,
	len: u16,
	initlen: u8,
	initvalue: Vec<u8>,
}

impl Command for OsalNvItemInit {
	const ID: u8 = 7;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvRead {
	id: u16,
	offset: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct OsalNvReadReply {
	status: u8,
	len: u8,
	value: Vec<u8>,
}

impl Command for OsalNvRead {
	const ID: u8 = 8;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = OsalNvReadReply;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvWrite {
	id: u16,
	offset: u8,
	len: u8,
	value: Vec<u8>,
}

impl Command for OsalNvWrite {
	const ID: u8 = 9;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct OsalStartTimer {
	id: u8,
	timeout: u16,
}

impl Command for OsalStartTimer {
	const ID: u8 = 10;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct OsalStopTimer {
	id: u8,
}

impl Command for OsalStopTimer {
	const ID: u8 = 11;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct Random {
}

#[derive(Debug, Clone, Deserialize)]
struct RandomReply {
	value: u16,
}

impl Command for Random {
	const ID: u8 = 12;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = RandomReply;
}

#[derive(Debug, Clone, Serialize)]
struct AdcRead {
	channel: u8,
	resolution: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct AdcReadReply {
	value: u16,
}

impl Command for AdcRead {
	const ID: u8 = 13;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = AdcReadReply;
}

#[derive(Debug, Clone, Serialize)]
struct Gpio {
	operation: u8,
	value: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct GpioReply {
	value: u8,
}

impl Command for Gpio {
	const ID: u8 = 14;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = GpioReply;
}

#[derive(Debug, Clone, Serialize)]
struct StackTune {
	operation: u8,
	value: i8,
}

#[derive(Debug, Clone, Deserialize)]
struct StackTuneReply {
	value: u8,
}

impl Command for StackTune {
	const ID: u8 = 15;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = StackTuneReply;
}

#[derive(Debug, Clone, Serialize)]
struct SetTime {
	utc: u32,
	hour: u8,
	minute: u8,
	second: u8,
	month: u8,
	day: u8,
	year: u16,
}

impl Command for SetTime {
	const ID: u8 = 16;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct GetTime {
}

#[derive(Debug, Clone, Deserialize)]
struct GetTimeReply {
	utc: u32,
	hour: u8,
	minute: u8,
	second: u8,
	month: u8,
	day: u8,
	year: u16,
}

impl Command for GetTime {
	const ID: u8 = 17;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = GetTimeReply;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvDelete {
	id: u16,
	len: u16,
}

impl Command for OsalNvDelete {
	const ID: u8 = 18;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvLength {
	id: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct OsalNvLengthReply {
	length: u16,
}

impl Command for OsalNvLength {
	const ID: u8 = 19;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = OsalNvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
struct SetTxPower {
	level: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct SetTxPowerReply {
	txpower: u8,
}

impl Command for SetTxPower {
	const ID: u8 = 20;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = SetTxPowerReply;
}

#[derive(Debug, Clone, Serialize)]
struct JammerParameters {
	jmrcntievents: u16,
	jmrhinoiselvl: u8,
	jmrdetectperiod: u32,
}

impl Command for JammerParameters {
	const ID: u8 = 21;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SnifferParameters {
	param: u8,
}

impl Command for SnifferParameters {
	const ID: u8 = 22;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ZdiagsInitStats {
}

impl Command for ZdiagsInitStats {
	const ID: u8 = 23;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ZdiagsClearStats {
	clearnv: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ZdiagsClearStatsReply {
	sysclock: u32,
}

impl Command for ZdiagsClearStats {
	const ID: u8 = 24;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ZdiagsClearStatsReply;
}

#[derive(Debug, Clone, Serialize)]
struct ZdiagsGetStats {
	attributeid: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct ZdiagsGetStatsReply {
	attributevalue: u32,
}

impl Command for ZdiagsGetStats {
	const ID: u8 = 25;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ZdiagsGetStatsReply;
}

#[derive(Debug, Clone, Serialize)]
struct ZdiagsRestoreStatsNv {
}

impl Command for ZdiagsRestoreStatsNv {
	const ID: u8 = 26;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ZdiagsSaveStatsToNv {
}

#[derive(Debug, Clone, Deserialize)]
struct ZdiagsSaveStatsToNvReply {
	sysclock: u32,
}

impl Command for ZdiagsSaveStatsToNv {
	const ID: u8 = 27;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ZdiagsSaveStatsToNvReply;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvReadExt {
	id: u16,
	offset: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct OsalNvReadExtReply {
	status: u8,
	len: u8,
	value: Vec<u8>,
}

impl Command for OsalNvReadExt {
	const ID: u8 = 28;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = OsalNvReadExtReply;
}

#[derive(Debug, Clone, Serialize)]
struct OsalNvWriteExt {
	id: u16,
	offset: u16,
	len: u16,
	value: Vec<u8>,
}

impl Command for OsalNvWriteExt {
	const ID: u8 = 29;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NvCreate {
	sysid: u8,
	itemid: u16,
	subid: u16,
	len: u32,
}

impl Command for NvCreate {
	const ID: u8 = 48;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NvDelete {
	sysid: u8,
	itemid: u16,
	subid: u16,
}

impl Command for NvDelete {
	const ID: u8 = 49;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NvLength {
	sysid: u8,
	itemid: u16,
	subid: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct NvLengthReply {
	len: u8,
}

impl Command for NvLength {
	const ID: u8 = 50;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = NvLengthReply;
}

#[derive(Debug, Clone, Serialize)]
struct NvRead {
	sysid: u8,
	itemid: u16,
	subid: u16,
	offset: u16,
	len: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct NvReadReply {
	status: u8,
	len: u8,
	value: Vec<u8>,
}

impl Command for NvRead {
	const ID: u8 = 51;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = NvReadReply;
}

#[derive(Debug, Clone, Serialize)]
struct NvWrite {
	sysid: u8,
	itemid: u16,
	subid: u16,
	offset: u16,
	len: u8,
	value: Vec<u8>,
}

impl Command for NvWrite {
	const ID: u8 = 52;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NvUpdate {
	sysid: u8,
	itemid: u16,
	subid: u16,
	len: u8,
	value: Vec<u8>,
}

impl Command for NvUpdate {
	const ID: u8 = 53;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NvCompact {
	threshold: u16,
}

impl Command for NvCompact {
	const ID: u8 = 54;
	const TYPE: CommandType = CommandType::SREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ResetInd {
	reason: u8,
	transportrev: u8,
	productid: u8,
	majorrel: u8,
	minorrel: u8,
	hwrev: u8,
}

impl Command for ResetInd {
	const ID: u8 = 128;
	const TYPE: CommandType = CommandType::AREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct OsalTimerExpired {
	id: u8,
}

impl Command for OsalTimerExpired {
	const ID: u8 = 129;
	const TYPE: CommandType = CommandType::AREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct JammerInd {
	jammerind: u8,
}

impl Command for JammerInd {
	const ID: u8 = 130;
	const TYPE: CommandType = CommandType::AREQ;
	const SUBSYSTEM: Subsystem = Subsystem::Sys;
	type Reply = ();
}

