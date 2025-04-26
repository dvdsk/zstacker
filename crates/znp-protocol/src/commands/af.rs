use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use super::{SubSystem, SyncReply, SyncRequest, basic_reply};

#[derive(Debug, Clone, Serialize_repr)]
#[repr(u8)]
pub enum LatencyRequirement {
    NoRequirement = 0,
    FastBeacons = 1,
    SlowBeacons = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterId(pub u16);

#[derive(Debug, Clone, Serialize)]
pub struct Register {
    pub endpoint: u8,
    pub app_prof_id: u16,
    pub app_device_id: u16,
    pub app_dev_ver: u8,
    pub latency_req: LatencyRequirement,
    pub in_clusters: Vec<ClusterId>,
    pub out_clusters: Vec<ClusterId>,
}

impl SyncRequest for Register {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = RegisterReply;
}

basic_reply! {Register, RegisterReply}

// #[derive(Debug, Clone, Serialize)]
// pub struct DataRequest {
//     pub dstaddr: u16,
//     pub destendpoint: u8,
//     pub srcendpoint: u8,
//     pub clusterid: u16,
//     pub transid: u8,
//     pub options: u8,
//     pub radius: u8,
//     pub len: u8,
//     pub data: Vec<u8>,
// }
//
// impl SyncRequest for DataRequest {
//     const ID: u8 = 1;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DataRequestReply;
// }
//
// basic_reply! { DataRequest, DataRequestReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct DataRequestExt {
//     pub dstaddrmode: u8,
//     pub dstaddr: IeeeAddr,
//     pub destendpoint: u8,
//     pub dstpanid: u16,
//     pub srcendpoint: u8,
//     pub clusterid: u16,
//     pub transid: u8,
//     pub options: u8,
//     pub radius: u8,
//     pub len: u16,
//     pub data: Vec<u8>,
// }
//
// impl SyncRequest for DataRequestExt {
//     const ID: u8 = 2;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DataRequestExtReply;
// }
//
// basic_reply! { DataRequestExt, DataRequestExtReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct DataRequestSrcRtg {
//     pub dstaddr: u16,
//     pub destendpoint: u8,
//     pub srcendpoint: u8,
//     pub clusterid: u16,
//     pub transid: u8,
//     pub options: u8,
//     pub radius: u8,
//     pub relaylist: Vec<u16>,
//     pub len: u8,
//     pub data: Vec<u8>,
// }
//
// impl SyncRequest for DataRequestSrcRtg {
//     const ID: u8 = 3;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DataRequestSrcRtgReply;
// }
//
// basic_reply! { DataRequestSrcRtg, DataRequestSrcRtgReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct Delete {
//     pub endpoint: u8,
// }
//
// impl SyncRequest for Delete {
//     const ID: u8 = 4;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DeleteReply;
// }
//
// basic_reply! { Delete, DeleteReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct InterPanCtl {
//     pub cmd: u8,
//     pub data: Vec<u8>,
// }
//
// impl SyncRequest for InterPanCtl {
//     const ID: u8 = 16;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = InterPanCtlReply;
// }
//
// basic_reply! { InterPanCtl, InterPanCtlReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct DataStore {
//     pub index: u16,
//     pub length: u8,
//     pub data: Vec<u8>,
// }
//
// impl SyncRequest for DataStore {
//     const ID: u8 = 17;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DataStoreReply;
// }
//
// basic_reply! { DataStore, DataStoreReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct DataRetrieve {
//     pub timestamp: u32,
//     pub index: u16,
//     pub length: u8,
// }
//
// #[derive(Debug, Clone, Deserialize)]
// pub struct DataRetrieveReply {
//     pub status: u8,
//     pub length: u8,
//     pub data: Vec<u8>,
// }
//
// impl SyncReply for DataRetrieveReply {
//     type Request = DataRetrieve;
// }
//
// impl SyncRequest for DataRetrieve {
//     const ID: u8 = 18;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = DataRetrieveReply;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct ApsfConfigSet {
//     pub endpoint: u8,
//     pub framedelay: u8,
//     pub windowsize: u8,
// }
//
// impl SyncRequest for ApsfConfigSet {
//     const ID: u8 = 19;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = ApsfConfigSetReply;
// }
//
// basic_reply! { ApsfConfigSet, ApsfConfigSetReply }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct ApsfConfigGet {
//     pub endpoint: u8,
// }
//
// #[derive(Debug, Clone, Deserialize)]
// pub struct ApsfConfigGetReply {
//     pub framedelay: u8,
//     pub windowsize: u8,
//     pub nomean: u8,
// }
//
// impl SyncReply for ApsfConfigGetReply {
//     type Request = ApsfConfigGet;
// }
//
// impl SyncRequest for ApsfConfigGet {
//     const ID: u8 = 20;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
//     type Reply = ApsfConfigGetReply;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct DataConfirm {
//     pub status: u8,
//     pub endpoint: u8,
//     pub transid: u8,
// }
//
// impl AsyncReply for DataConfirm {
//     const ID: u8 = 128;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct IncomingMsg {
//     pub groupid: u16,
//     pub clusterid: u16,
//     pub srcaddr: u16,
//     pub srcendpoint: u8,
//     pub dstendpoint: u8,
//     pub wasbroadcast: u8,
//     pub linkquality: u8,
//     pub securityuse: u8,
//     pub timestamp: u32,
//     pub transseqnumber: u8,
//     pub len: u8,
//     pub data: Vec<u8>,
// }
//
// impl AsyncRequest for IncomingMsg {
//     const ID: u8 = 129;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct IncomingMsgExt {
//     pub groupid: u16,
//     pub clusterid: u16,
//     pub srcaddrmode: u8,
//     pub srcaddr: IeeeAddr,
//     pub srcendpoint: u8,
//     pub srcpanid: u16,
//     pub dstendpoint: u8,
//     pub wasbroadcast: u8,
//     pub linkquality: u8,
//     pub securityuse: u8,
//     pub timestamp: u32,
//     pub transseqnumber: u8,
//     pub len: u16,
//     pub data: Vec<u8>,
// }
//
// impl AsyncRequest for IncomingMsgExt {
//     const ID: u8 = 130;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
// }
//
// #[derive(Debug, Clone, Serialize)]
// pub struct ReflectError {
//     pub status: u8,
//     pub endpoint: u8,
//     pub transid: u8,
//     pub dstaddrmode: u8,
//     pub dstaddr: u16,
// }
//
// impl AsyncRequest for ReflectError {
//     const ID: u8 = 131;
//     const SUBSYSTEM: SubSystem = SubSystem::Af;
// }
