use serde::{Deserialize, Serialize};

use super::{
    AsyncRequest, IeeeAddr, Status, SubSystem, SyncReply, SyncRequest,
};

#[derive(Debug, Clone, Serialize)]
pub struct Register {
    pub endpoint: u8,
    pub appprofid: u16,
    pub appdeviceid: u16,
    pub appdevver: u8,
    pub latencyreq: u8,
    pub appinclusterlist: Vec<u16>,
}

impl SyncRequest for Register {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = RegisterReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterReply(pub Status);

impl SyncReply for RegisterReply {
    type Request = Register;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataRequest {
    pub dstaddr: u16,
    pub destendpoint: u8,
    pub srcendpoint: u8,
    pub clusterid: u16,
    pub transid: u8,
    pub options: u8,
    pub radius: u8,
    pub len: u8,
    pub data: Vec<u8>,
}

impl SyncRequest for DataRequest {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataRequestReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRequestReply(pub Status);

impl SyncReply for DataRequestReply {
    type Request = DataRequest;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataRequestExt {
    pub dstaddrmode: u8,
    pub dstaddr: IeeeAddr,
    pub destendpoint: u8,
    pub dstpanid: u16,
    pub srcendpoint: u8,
    pub clusterid: u16,
    pub transid: u8,
    pub options: u8,
    pub radius: u8,
    pub len: u16,
    pub data: Vec<u8>,
}

impl SyncRequest for DataRequestExt {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataRequestExtReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRequestExtReply(pub Status);

impl SyncReply for DataRequestExtReply {
    type Request = DataRequestExt;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataRequestSrcRtg {
    pub dstaddr: u16,
    pub destendpoint: u8,
    pub srcendpoint: u8,
    pub clusterid: u16,
    pub transid: u8,
    pub options: u8,
    pub radius: u8,
    pub relaylist: Vec<u16>,
    pub len: u8,
    pub data: Vec<u8>,
}

impl SyncRequest for DataRequestSrcRtg {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataRequestSrcRtgReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRequestSrcRtgReply(pub Status);

impl SyncReply for DataRequestSrcRtgReply {
    type Request = DataRequestSrcRtg;
}

#[derive(Debug, Clone, Serialize)]
pub struct Delete {
    pub endpoint: u8,
}

impl SyncRequest for Delete {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DeleteReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteReply(pub Status);

impl SyncReply for DeleteReply {
    type Request = Delete;
}

#[derive(Debug, Clone, Serialize)]
pub struct InterPanCtl {
    pub cmd: u8,
    pub data: Vec<u8>,
}

impl SyncRequest for InterPanCtl {
    const ID: u8 = 16;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = InterPanCtlReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct InterPanCtlReply(pub Status);

impl SyncReply for InterPanCtlReply {
    type Request = InterPanCtl;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataStore {
    pub index: u16,
    pub length: u8,
    pub data: Vec<u8>,
}

impl SyncRequest for DataStore {
    const ID: u8 = 17;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataStoreReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataStoreReply(pub Status);

impl SyncReply for DataStoreReply {
    type Request = DataStore;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataRetrieve {
    pub timestamp: u32,
    pub index: u16,
    pub length: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRetrieveReply {
    pub status: u8,
    pub length: u8,
    pub data: Vec<u8>,
}

impl SyncReply for DataRetrieveReply {
    type Request = DataRetrieve;
}

impl SyncRequest for DataRetrieve {
    const ID: u8 = 18;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataRetrieveReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct ApsfConfigSet {
    pub endpoint: u8,
    pub framedelay: u8,
    pub windowsize: u8,
}

impl SyncRequest for ApsfConfigSet {
    const ID: u8 = 19;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = ApsfConfigSetReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApsfConfigSetReply(pub Status);

impl SyncReply for ApsfConfigSetReply {
    type Request = ApsfConfigSet;
}

#[derive(Debug, Clone, Serialize)]
pub struct ApsfConfigGet {
    pub endpoint: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApsfConfigGetReply {
    pub framedelay: u8,
    pub windowsize: u8,
    pub nomean: u8,
}

impl SyncReply for ApsfConfigGetReply {
    type Request = ApsfConfigGet;
}

impl SyncRequest for ApsfConfigGet {
    const ID: u8 = 20;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = ApsfConfigGetReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct DataConfirm {
    pub status: u8,
    pub endpoint: u8,
    pub transid: u8,
}

impl AsyncRequest for DataConfirm {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
pub struct IncomingMsg {
    pub groupid: u16,
    pub clusterid: u16,
    pub srcaddr: u16,
    pub srcendpoint: u8,
    pub dstendpoint: u8,
    pub wasbroadcast: u8,
    pub linkquality: u8,
    pub securityuse: u8,
    pub timestamp: u32,
    pub transseqnumber: u8,
    pub len: u8,
    pub data: Vec<u8>,
}

impl AsyncRequest for IncomingMsg {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
pub struct IncomingMsgExt {
    pub groupid: u16,
    pub clusterid: u16,
    pub srcaddrmode: u8,
    pub srcaddr: IeeeAddr,
    pub srcendpoint: u8,
    pub srcpanid: u16,
    pub dstendpoint: u8,
    pub wasbroadcast: u8,
    pub linkquality: u8,
    pub securityuse: u8,
    pub timestamp: u32,
    pub transseqnumber: u8,
    pub len: u16,
    pub data: Vec<u8>,
}

impl AsyncRequest for IncomingMsgExt {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
pub struct ReflectError {
    pub status: u8,
    pub endpoint: u8,
    pub transid: u8,
    pub dstaddrmode: u8,
    pub dstaddr: u16,
}

impl AsyncRequest for ReflectError {
    const ID: u8 = 131;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}
