#![allow(dead_code)]
use serde::{Serialize, Deserialize};

use super::{AsyncRequest, SyncRequest, SyncReply, IeeeAddr, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
struct ResetReq {
	setdefault: u8,
}


impl SyncRequest for ResetReq {
	const ID: u8 = 1;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = ResetReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct ResetReqReply(Status);
impl SyncReply for ResetReqReply {
    type Request = ResetReq;
}

#[derive(Debug, Clone, Serialize)]
struct Init {
}


impl SyncRequest for Init {
	const ID: u8 = 2;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = InitReply;
}

#[derive(Debug, Clone, Deserialize)]
struct InitReply(Status);
impl SyncReply for InitReply {
    type Request = Init;
}

#[derive(Debug, Clone, Serialize)]
struct StartReq {
	starttime: u32,
	panid: u16,
	logicalchannel: u8,
	channelpage: u8,
	beaconorder: u8,
	superframeorder: u8,
	pancoordinator: u8,
	batterylifeext: u8,
	coordrealignment: u8,
	realignkeysource: Vec<u8>,
	realignsecuritylevel: u8,
	realignkeyidmode: u8,
	realignkeyindex: u8,
	beaconkeysource: Vec<u8>,
	beaconsecuritylevel: u8,
	beaconkeyidmode: u8,
	beaconkeyindex: u8,
}


impl SyncRequest for StartReq {
	const ID: u8 = 3;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = StartReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct StartReqReply(Status);
impl SyncReply for StartReqReply {
    type Request = StartReq;
}

#[derive(Debug, Clone, Serialize)]
struct SyncReq {
	logicalchannel: u8,
	channelpage: u8,
	trackbeacon: u8,
}


impl SyncRequest for SyncReq {
	const ID: u8 = 4;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SyncReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SyncReqReply(Status);
impl SyncReply for SyncReqReply {
    type Request = SyncReq;
}

#[derive(Debug, Clone, Serialize)]
struct DataReq {
	destaddressmode: u8,
	destaddress: IeeeAddr,
	destpanid: u16,
	srcaddressmode: u8,
	handle: u8,
	txoption: u8,
	logicalchannel: u8,
	power: u8,
	keysource: Vec<u8>,
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
	msdulength: u8,
	msdu: Vec<u8>,
}


impl SyncRequest for DataReq {
	const ID: u8 = 5;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = DataReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct DataReqReply(Status);
impl SyncReply for DataReqReply {
    type Request = DataReq;
}

#[derive(Debug, Clone, Serialize)]
struct AssociateReq {
	logicalchannel: u8,
	channelpage: u8,
	coordaddressmode: u8,
	coordaddress: IeeeAddr,
	coordpanid: u16,
	capabilityinformation: u8,
	keysource: Vec<u8>,
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}


impl SyncRequest for AssociateReq {
	const ID: u8 = 6;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = AssociateReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct AssociateReqReply(Status);
impl SyncReply for AssociateReqReply {
    type Request = AssociateReq;
}

#[derive(Debug, Clone, Serialize)]
struct DisassociateReq {
	deviceaddressmode: u8,
	deviceaddress: IeeeAddr,
	devicepanid: u16,
	disassociatereason: u8,
	txindirect: u8,
	keysource: Vec<u8>,
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}


impl SyncRequest for DisassociateReq {
	const ID: u8 = 7;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = DisassociateReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct DisassociateReqReply(Status);
impl SyncReply for DisassociateReqReply {
    type Request = DisassociateReq;
}

#[derive(Debug, Clone, Serialize)]
struct GetReq {
	attribute: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct GetReqReply {
	status: u8,
	data: [u8; 16],
}

impl SyncReply for GetReqReply {
    type Request = GetReq;
}


impl SyncRequest for GetReq {
	const ID: u8 = 8;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
	type Reply = GetReqReply;
}

#[derive(Debug, Clone, Serialize)]
struct SetReq {
	attribute: u8,
	attributevalue: Vec<u8>,
}


impl SyncRequest for SetReq {
	const ID: u8 = 9;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SetReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SetReqReply(Status);

impl SyncReply for SetReqReply {
    type Request = SetReq;
}

#[derive(Debug, Clone, Serialize)]
struct ScanReq {
	scanchannels: u32,
	scantype: u8,
	scanduration: u8,
	channelpage: u8,
	maxresults: u8,
	keysource: Vec<u8>,
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}


impl SyncRequest for ScanReq {
	const ID: u8 = 12;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = ScanReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct ScanReqReply(Status);

impl SyncReply for ScanReqReply {
    type Request = ScanReq;
}

#[derive(Debug, Clone, Serialize)]
struct PollReq {
	coordaddressmode: u8,
	coordaddress: IeeeAddr,
	coordpanid: u16,
	keysource: Vec<u8>,
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}


impl SyncRequest for PollReq {
	const ID: u8 = 13;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = PollReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct PollReqReply(Status);

impl SyncReply for PollReqReply {
    type Request = PollReq;
}

#[derive(Debug, Clone, Serialize)]
struct PurgeReq {
	msduhandle: u8,
}


impl SyncRequest for PurgeReq {
	const ID: u8 = 14;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = PurgeReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct PurgeReqReply(Status);

impl SyncReply for PurgeReqReply {
    type Request = PurgeReq;
}

#[derive(Debug, Clone, Serialize)]
struct SetRxGainReq {
	mode: u8,
}


impl SyncRequest for SetRxGainReq {
	const ID: u8 = 15;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SetRxGainReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SetRxGainReqReply(Status);

impl SyncReply for SetRxGainReqReply {
    type Request = SetRxGainReq;
}

#[derive(Debug, Clone, Serialize)]
struct SecurityGetReq {
	attribute: u8,
	index1: u8,
	index2: u8,
}


impl SyncRequest for SecurityGetReq {
	const ID: u8 = 48;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SecurityGetReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SecurityGetReqReply(Status);

impl SyncReply for SecurityGetReqReply {
    type Request = SecurityGetReq;
}

#[derive(Debug, Clone, Serialize)]
struct SecuritySetReq {
	attribute: u8,
	attributevalue: Vec<u8>,
}


impl SyncRequest for SecuritySetReq {
	const ID: u8 = 49;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SecuritySetReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SecuritySetReqReply(Status);

impl SyncReply for SecuritySetReqReply {
    type Request = SecuritySetReq;
}

#[derive(Debug, Clone, Serialize)]
struct AssociateRsp {
	extaddr: IeeeAddr,
	assocshortaddress: u16,
	assocstatus: u8,
}


impl SyncRequest for AssociateRsp {
	const ID: u8 = 80;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = AssociateRspReply;
}

#[derive(Debug, Clone, Deserialize)]
struct AssociateRspReply(Status);

impl SyncReply for AssociateRspReply {
    type Request = AssociateRsp;
}

#[derive(Debug, Clone, Serialize)]
struct OrphanRsp {
	extaddr: IeeeAddr,
	assocshortaddress: u16,
	associatedmember: u8,
}


impl SyncRequest for OrphanRsp {
	const ID: u8 = 81;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = OrphanRspReply;
}

#[derive(Debug, Clone, Deserialize)]
struct OrphanRspReply(Status);

impl SyncReply for OrphanRspReply {
    type Request = OrphanRsp;
}

#[derive(Debug, Clone, Serialize)]
struct SyncLossInd {
	status: u8,
	panid: u16,
	logicalchannel: u8,
	channelpage: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for SyncLossInd {
	const ID: u8 = 128;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct AssociateInd {
	deviceextendedaddress: IeeeAddr,
	capabilities: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for AssociateInd {
	const ID: u8 = 129;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct AssociateCnf {
	status: u8,
	deviceshortaddress: u16,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for AssociateCnf {
	const ID: u8 = 130;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct BeaconNotifyInd {
	bsn: u8,
	timestamp: u32,
	coordinatoraddressmode: u8,
	coordinatorextendedaddress: IeeeAddr,
	panid: u16,
	superframespec: u16,
	logicalchannel: u8,
	gtspermit: u8,
	linkquality: u8,
	securityfailure: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
	pendingaddrspec: u8,
	addresslist: [u8; 32],
	sdulength: u8,
	nsdu: Vec<u8>,
}

impl AsyncRequest for BeaconNotifyInd {
	const ID: u8 = 131;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct DataCnf {
	status: u8,
	handle: u8,
	timestamp: u32,
	timestamp2: u16,
}

impl AsyncRequest for DataCnf {
	const ID: u8 = 132;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct DataInd {
	srcaddrmode: u8,
	srcaddr: IeeeAddr,
	dstaddrmode: u8,
	dstaddr: IeeeAddr,
	timestamp: u32,
	timestamp2: u16,
	srcpanid: u16,
	dstpanid: u16,
	linkquality: u8,
	correlation: u8,
	rssi: u8,
	dsn: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
	length: u8,
	data: Vec<u8>,
}

impl AsyncRequest for DataInd {
	const ID: u8 = 133;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct DisassociateInd {
	extendedaddress: IeeeAddr,
	disassociatereason: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for DisassociateInd {
	const ID: u8 = 134;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct DisassociateCnf {
	status: u8,
	deviceaddrmode: u8,
	deviceaddr: IeeeAddr,
	devicepanid: u16,
}

impl AsyncRequest for DisassociateCnf {
	const ID: u8 = 135;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct OrphanInd {
	extendedaddr: IeeeAddr,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for OrphanInd {
	const ID: u8 = 138;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct PollCnf {
	status: u8,
}

impl AsyncRequest for PollCnf {
	const ID: u8 = 139;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct ScanCnf {
	status: u8,
	ed: u8,
	scantype: u8,
	channelpage: u8,
	unscannedchannellist: u32,
	resultlistcount: u8,
	resultlistmaxlength: u8,
	resultlist: Vec<u8>,
}

impl AsyncRequest for ScanCnf {
	const ID: u8 = 140;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct CommStatusInd {
	status: u8,
	srcaddrmode: u8,
	srcaddr: IeeeAddr,
	dstaddrmode: u8,
	dstaddr: IeeeAddr,
	devicepanid: u16,
	reason: u8,
	keysource: [u8; 8],
	securitylevel: u8,
	keyidmode: u8,
	keyindex: u8,
}

impl AsyncRequest for CommStatusInd {
	const ID: u8 = 141;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct StartCnf {
	status: u8,
}

impl AsyncRequest for StartCnf {
	const ID: u8 = 142;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct RxEnableCnf {
	status: u8,
}

impl AsyncRequest for RxEnableCnf {
	const ID: u8 = 143;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
struct PurgeCnf {
	status: u8,
	handle: u8,
}

impl AsyncRequest for PurgeCnf {
	const ID: u8 = 144;
	const SUBSYSTEM: SubSystem = SubSystem::Mac;
}
