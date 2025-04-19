use serde::{Deserialize, Serialize};

use super::{basic_reply, AsyncRequest, IeeeAddr, SubSystem, SyncReply, SyncRequest};

#[derive(Debug, Clone, Serialize)]
pub struct ResetReq {
    pub setdefault: u8,
}

impl SyncRequest for ResetReq {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = ResetReqReply;
}

basic_reply! { ResetReq, ResetReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct Init {}

impl SyncRequest for Init {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = InitReply;
}

basic_reply! { Init, InitReply }

#[derive(Debug, Clone, Serialize)]
pub struct StartReq {
    pub starttime: u32,
    pub panid: u16,
    pub logicalchannel: u8,
    pub channelpage: u8,
    pub beaconorder: u8,
    pub superframeorder: u8,
    pub pancoordinator: u8,
    pub batterylifeext: u8,
    pub coordrealignment: u8,
    pub realignkeysource: Vec<u8>,
    pub realignsecuritylevel: u8,
    pub realignkeyidmode: u8,
    pub realignkeyindex: u8,
    pub beaconkeysource: Vec<u8>,
    pub beaconsecuritylevel: u8,
    pub beaconkeyidmode: u8,
    pub beaconkeyindex: u8,
}

impl SyncRequest for StartReq {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = StartReqReply;
}

basic_reply! { StartReq, StartReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct SyncReq {
    pub logicalchannel: u8,
    pub channelpage: u8,
    pub trackbeacon: u8,
}

impl SyncRequest for SyncReq {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SyncReqReply;
}

basic_reply! { SyncReq, SyncReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct DataReq {
    pub destaddressmode: u8,
    pub destaddress: IeeeAddr,
    pub destpanid: u16,
    pub srcaddressmode: u8,
    pub handle: u8,
    pub txoption: u8,
    pub logicalchannel: u8,
    pub power: u8,
    pub keysource: Vec<u8>,
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
    pub msdulength: u8,
    pub msdu: Vec<u8>,
}

impl SyncRequest for DataReq {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = DataReqReply;
}

basic_reply! { DataReq, DataReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct AssociateReq {
    pub logicalchannel: u8,
    pub channelpage: u8,
    pub coordaddressmode: u8,
    pub coordaddress: IeeeAddr,
    pub coordpanid: u16,
    pub capabilityinformation: u8,
    pub keysource: Vec<u8>,
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl SyncRequest for AssociateReq {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = AssociateReqReply;
}

basic_reply! { AssociateReq, AssociateReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct DisassociateReq {
    pub deviceaddressmode: u8,
    pub deviceaddress: IeeeAddr,
    pub devicepanid: u16,
    pub disassociatereason: u8,
    pub txindirect: u8,
    pub keysource: Vec<u8>,
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl SyncRequest for DisassociateReq {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = DisassociateReqReply;
}

basic_reply! { DisassociateReq, DisassociateReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct GetReq {
    pub attribute: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetReqReply {
    pub status: u8,
    pub data: [u8; 16],
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
pub struct SetReq {
    pub attribute: u8,
    pub attributevalue: Vec<u8>,
}

impl SyncRequest for SetReq {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SetReqReply;
}

basic_reply! { SetReq, SetReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct ScanReq {
    pub scanchannels: u32,
    pub scantype: u8,
    pub scanduration: u8,
    pub channelpage: u8,
    pub maxresults: u8,
    pub keysource: Vec<u8>,
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl SyncRequest for ScanReq {
    const ID: u8 = 12;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = ScanReqReply;
}

basic_reply! { ScanReq, ScanReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct PollReq {
    pub coordaddressmode: u8,
    pub coordaddress: IeeeAddr,
    pub coordpanid: u16,
    pub keysource: Vec<u8>,
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl SyncRequest for PollReq {
    const ID: u8 = 13;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = PollReqReply;
}

basic_reply! { PollReq, PollReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct PurgeReq {
    pub msduhandle: u8,
}

impl SyncRequest for PurgeReq {
    const ID: u8 = 14;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = PurgeReqReply;
}

basic_reply! { PurgeReq, PurgeReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct SetRxGainReq {
    pub mode: u8,
}

impl SyncRequest for SetRxGainReq {
    const ID: u8 = 15;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SetRxGainReqReply;
}

basic_reply! { SetRxGainReq, SetRxGainReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct SecurityGetReq {
    pub attribute: u8,
    pub index1: u8,
    pub index2: u8,
}

impl SyncRequest for SecurityGetReq {
    const ID: u8 = 48;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SecurityGetReqReply;
}

basic_reply! { SecurityGetReq, SecurityGetReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct SecuritySetReq {
    pub attribute: u8,
    pub attributevalue: Vec<u8>,
}

impl SyncRequest for SecuritySetReq {
    const ID: u8 = 49;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = SecuritySetReqReply;
}

basic_reply! { SecuritySetReq, SecuritySetReqReply }

#[derive(Debug, Clone, Serialize)]
pub struct AssociateRsp {
    pub extaddr: IeeeAddr,
    pub assocshortaddress: u16,
    pub assocstatus: u8,
}

impl SyncRequest for AssociateRsp {
    const ID: u8 = 80;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = AssociateRspReply;
}

basic_reply! { AssociateRsp, AssociateRspReply }

#[derive(Debug, Clone, Serialize)]
pub struct OrphanRsp {
    pub extaddr: IeeeAddr,
    pub assocshortaddress: u16,
    pub associatedmember: u8,
}

impl SyncRequest for OrphanRsp {
    const ID: u8 = 81;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
    type Reply = OrphanRspReply;
}

basic_reply! { OrphanRsp, OrphanRspReply }

#[derive(Debug, Clone, Serialize)]
pub struct SyncLossInd {
    pub status: u8,
    pub panid: u16,
    pub logicalchannel: u8,
    pub channelpage: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for SyncLossInd {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct AssociateInd {
    pub deviceextendedaddress: IeeeAddr,
    pub capabilities: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for AssociateInd {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct AssociateCnf {
    pub status: u8,
    pub deviceshortaddress: u16,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for AssociateCnf {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct BeaconNotifyInd {
    pub bsn: u8,
    pub timestamp: u32,
    pub coordinatoraddressmode: u8,
    pub coordinatorextendedaddress: IeeeAddr,
    pub panid: u16,
    pub superframespec: u16,
    pub logicalchannel: u8,
    pub gtspermit: u8,
    pub linkquality: u8,
    pub securityfailure: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
    pub pendingaddrspec: u8,
    pub addresslist: [u8; 32],
    pub sdulength: u8,
    pub nsdu: Vec<u8>,
}

impl AsyncRequest for BeaconNotifyInd {
    const ID: u8 = 131;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataCnf {
    pub status: u8,
    pub handle: u8,
    pub timestamp: u32,
    pub timestamp2: u16,
}

impl AsyncRequest for DataCnf {
    const ID: u8 = 132;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataInd {
    pub srcaddrmode: u8,
    pub srcaddr: IeeeAddr,
    pub dstaddrmode: u8,
    pub dstaddr: IeeeAddr,
    pub timestamp: u32,
    pub timestamp2: u16,
    pub srcpanid: u16,
    pub dstpanid: u16,
    pub linkquality: u8,
    pub correlation: u8,
    pub rssi: u8,
    pub dsn: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
    pub length: u8,
    pub data: Vec<u8>,
}

impl AsyncRequest for DataInd {
    const ID: u8 = 133;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct DisassociateInd {
    pub extendedaddress: IeeeAddr,
    pub disassociatereason: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for DisassociateInd {
    const ID: u8 = 134;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct DisassociateCnf {
    pub status: u8,
    pub deviceaddrmode: u8,
    pub deviceaddr: IeeeAddr,
    pub devicepanid: u16,
}

impl AsyncRequest for DisassociateCnf {
    const ID: u8 = 135;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct OrphanInd {
    pub extendedaddr: IeeeAddr,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for OrphanInd {
    const ID: u8 = 138;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct PollCnf {
    pub status: u8,
}

impl AsyncRequest for PollCnf {
    const ID: u8 = 139;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanCnf {
    pub status: u8,
    pub ed: u8,
    pub scantype: u8,
    pub channelpage: u8,
    pub unscannedchannellist: u32,
    pub resultlistcount: u8,
    pub resultlistmaxlength: u8,
    pub resultlist: Vec<u8>,
}

impl AsyncRequest for ScanCnf {
    const ID: u8 = 140;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct CommStatusInd {
    pub status: u8,
    pub srcaddrmode: u8,
    pub srcaddr: IeeeAddr,
    pub dstaddrmode: u8,
    pub dstaddr: IeeeAddr,
    pub devicepanid: u16,
    pub reason: u8,
    pub keysource: [u8; 8],
    pub securitylevel: u8,
    pub keyidmode: u8,
    pub keyindex: u8,
}

impl AsyncRequest for CommStatusInd {
    const ID: u8 = 141;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct StartCnf {
    pub status: u8,
}

impl AsyncRequest for StartCnf {
    const ID: u8 = 142;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct RxEnableCnf {
    pub status: u8,
}

impl AsyncRequest for RxEnableCnf {
    const ID: u8 = 143;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}

#[derive(Debug, Clone, Serialize)]
pub struct PurgeCnf {
    pub status: u8,
    pub handle: u8,
}

impl AsyncRequest for PurgeCnf {
    const ID: u8 = 144;
    const SUBSYSTEM: SubSystem = SubSystem::Mac;
}
