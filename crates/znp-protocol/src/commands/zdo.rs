#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{
    AsyncRequest, SyncRequest, SyncReply, IeeeAddr, Status,
    SubSystem,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkAddrReq {
    // ieeeaddr: IeeeAddr,
    // reqtype: u8,
    // startindex: u8,
}

impl SyncRequest for NwkAddrReq {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IeeeAddrReq {
    // shortaddr: u16,
    // reqtype: u8,
    // startindex: u8,
}

impl SyncRequest for IeeeAddrReq {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl SyncRequest for NodeDescReq {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PowerDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl SyncRequest for PowerDescReq {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
    // endpoint: u8,
}

impl SyncRequest for SimpleDescReq {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveEpReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl SyncRequest for ActiveEpReq {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MatchDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
    // profileid: u16,
    // inclusterlist: Vec<u16>,
}

impl SyncRequest for MatchDescReq {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComplexDescReq {
    dstaddr: u16,
    nwkaddrofinterest: u16,
}

impl SyncRequest for ComplexDescReq {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDescReq {
    dstaddr: u16,
    nwkaddrofinterest: u16,
}

impl SyncRequest for UserDescReq {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndDeviceAnnce {
    nwkaddr: u16,
    ieeeaddr: IeeeAddr,
    capability: u8,
}

impl SyncRequest for EndDeviceAnnce {
    const ID: u8 = 10;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDescSet {
    dstaddr: u16,
    nwkaddrofinterest: u16,
    descriptor_len: u8,
    userdescriptor: Vec<u8>,
}

impl SyncRequest for UserDescSet {
    const ID: u8 = 11;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerDiscReq {
    // servermask: u16,
}

impl SyncRequest for ServerDiscReq {
    const ID: u8 = 12;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndDeviceBindReq {
    dstaddr: u16,
    localcoord: u16,
    localieee: IeeeAddr,
    endpoint: u8,
    profileid: u16,
    inclusterlist: Vec<u16>,
}

impl SyncRequest for EndDeviceBindReq {
    const ID: u8 = 32;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BindReq {
    // dstaddr: u16,
    // srcaddr: IeeeAddr,
    // srcendpoint: u8,
    // clusterid: u16,
    // dstaddrmode: u8,
    // dstaddress: IeeeAddr,
    // dstendpoint: u8,
}

impl SyncRequest for BindReq {
    const ID: u8 = 33;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnbindReq {
    // dstaddr: u16,
    // srcaddr: IeeeAddr,
    // srcendpoint: u8,
    // clusterid: u16,
    // dstaddrmode: u8,
    // dstaddress: IeeeAddr,
    // dstendpoint: u8,
}

impl SyncRequest for UnbindReq {
    const ID: u8 = 34;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetLinkKey {
    shortaddr: u16,
    ieeeaddr: IeeeAddr,
    linkkey: Vec<u8>,
}

impl SyncRequest for SetLinkKey {
    const ID: u8 = 35;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RemoveLinkKey {
    ieeeaddr: IeeeAddr,
}

impl SyncRequest for RemoveLinkKey {
    const ID: u8 = 36;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetLinkKey {
    ieeeaddr: IeeeAddr,
}

#[derive(Debug, Clone, Deserialize)]
struct GetLinkKeyReply {
    status: u8,
    ieeeaddr: IeeeAddr,
    linkkeydata: [u8; 16],
}

impl SyncReply for GetLinkKeyReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for GetLinkKey {
    const ID: u8 = 37;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = GetLinkKeyReply;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkDiscoveryReq {
    scanchannels: u32,
    scanduration: u8,
}

impl SyncRequest for NwkDiscoveryReq {
    const ID: u8 = 38;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JoinReq {
    logicalchannel: u8,
    panid: u16,
    extendedpanid: IeeeAddr,
    chosenparent: u16,
    parentdepth: u8,
    stackprofile: u8,
}

impl SyncRequest for JoinReq {
    const ID: u8 = 39;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtNwkDiscReq {
    dstaddr: u16,
    scanchannels: u32,
    scanduration: u8,
    startindex: u8,
}

impl SyncRequest for MgmtNwkDiscReq {
    const ID: u8 = 48;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtLqiReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl SyncRequest for MgmtLqiReq {
    const ID: u8 = 49;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtRtgReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl SyncRequest for MgmtRtgReq {
    const ID: u8 = 50;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtBindReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl SyncRequest for MgmtBindReq {
    const ID: u8 = 51;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtLeaveReq {
    // dstaddr: u16,
    // deviceaddress: IeeeAddr,
    // removechildrenRejoin: u8,
}

impl SyncRequest for MgmtLeaveReq {
    const ID: u8 = 52;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtDirectJoinReq {
    dstaddr: u16,
    deviceaddr: IeeeAddr,
    capinfo: u8,
}

impl SyncRequest for MgmtDirectJoinReq {
    const ID: u8 = 53;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtPermitJoinReq {
    // addrmode: u8,
    // dstaddr: u16,
    // duration: u8,
    // tcsignificance: u8,
}

impl SyncRequest for MgmtPermitJoinReq {
    const ID: u8 = 54;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtNwkUpdateReq {
    // dstaddr: u16,
    // dstaddrmode: u8,
    // channelmask: u32,
    // scanduration: u8,
    //     // TODO: below two have various combinations of present/not present depending on scanduration
    // scancount: u8,
    // nwkmanageraddr: u16,
}

impl SyncRequest for MgmtNwkUpdateReq {
    const ID: u8 = 55; // the spec says 0x38 but TI used 0x37 see https://github.com/Koenkk/zigbee-herdsman/issues/1237
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgCbRegister {
    clusterid: u16,
}

impl SyncRequest for MsgCbRegister {
    const ID: u8 = 62;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgCbRemove {
    clusterid: u16,
}

impl SyncRequest for MsgCbRemove {
    const ID: u8 = 63;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StartupFromApp {
    startdelay: u16,
}

impl SyncRequest for StartupFromApp {
    const ID: u8 = 64;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AutoFindDestination {
    endpoint: u8,
}

impl AsyncRequest for AutoFindDestination {
    const ID: u8 = 65;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkAddrRsp {
    // status: u8,
    // Parse the ieeeaddr as it is needed for ZNP waitFor (see zStackAdapter.requestNetworkAddress())
    // ieeeaddr: IeeeAddr,
    // nwkaddr: u16,
    // startindex: u8,
    // assocdevlist: Vec<u16>,
}

impl SyncReply for NwkAddrRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IeeeAddrRsp {
    // status: u8,
    // ieeeaddr: IeeeAddr,
    // nwkaddr: u16,
    // startindex: u8,
    // assocdevlist: Vec<u16>,
}

impl SyncReply for IeeeAddrRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // logicaltype_cmplxdescavai_userdescavai: u8,
    // apsflags_freqband: u8,
    // maccapflags: u8,
    // manufacturercode: u16,
    // maxbuffersize: u8,
    // maxintransfersize: u16,
    // servermask: u16,
    // maxouttransfersize: u16,
    // descriptorcap: u8,
}

impl SyncReply for NodeDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PowerDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // currentpowermode_avaipowersrc: u8,
    // currentpowersrc_currentpowersrclevel: u8,
}

impl SyncReply for PowerDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // len: u8,
    // endpoint: u8,
    // profileid: u16,
    // deviceid: u16,
    // deviceversion: u8,
    // inclusterlist: Vec<u16>,
}

impl SyncReply for SimpleDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveEpRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // activeeplist: Vec<u8>,
}

impl SyncReply for ActiveEpRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Deserialize)]
struct MatchDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // matchlength: u8,
    // matchlist: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComplexDescRsp {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
    complexlength: u8,
    complexdesclist: Vec<u8>,
}

impl SyncReply for ComplexDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDescRsp {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
    userlength: u8,
    userdescriptor: Vec<u8>,
}

impl SyncReply for UserDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserDescConf {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
}

impl AsyncRequest for UserDescConf {
    const ID: u8 = 137;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerDiscRsp {
    // srcaddr: u16,
    // status: u8,
    // servermask: u16,
}

impl SyncReply for ServerDiscRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

/// https://github.com/Koenkk/zigbee2mqtt/issues/3363 {
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Unknown {}

impl AsyncRequest for Unknown {
    const ID: u8 = 159;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndDeviceBindRsp {
    srcaddr: u16,
    status: u8,
}

impl SyncReply for EndDeviceBindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BindRsp {
    // srcaddr: u16,
    // status: u8,
}

impl SyncReply for BindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnbindRsp {
    // srcaddr: u16,
    // status: u8,
}

impl SyncReply for UnbindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtNwkDiscRsp {
    srcaddr: u16,
    status: u8,
    networkcount: u8,
    startindex: u8,
    networklist: Vec<Network>,
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
    super_frame_order: u8, // 5th byte bits 7-4
    permit_joining: bool,
}

impl SyncReply for MgmtNwkDiscRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtLqiRsp {
    // srcaddr: u16,
    // status: u8,
    // neighbortableentries: u8,
    // startindex: u8,
    // neighborlqilistcount: u8,
    // neighborlqilist: compile_error!("needs custom derive with NeighborLqi type"),
}

impl SyncReply for MgmtLqiRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtRtgRsp {
    // srcaddr: u16,
    // status: u8,
    // routingtableentries: u8,
    // startindex: u8,
    // routingtablelistcount: u8,
    // routingtablelist: RoutingTable,
}

impl SyncReply for MgmtRtgRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtBindRsp {
    // srcaddr: u16,
    // status: u8,
    // bindingtableentries: u8,
    // startindex: u8,
    // bindingtablelistcount: u8,
    // bindingtablelist: BindTable,
}

impl SyncReply for MgmtBindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtLeaveRsp {
    // srcaddr: u16,
    // status: u8,
}

impl SyncReply for MgmtLeaveRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtDirectJoinRsp {
    srcaddr: u16,
    status: u8,
}

impl SyncReply for MgmtDirectJoinRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtPermitJoinRsp {
    // srcaddr: u16,
    // status: u8,
}

impl SyncReply for MgmtPermitJoinRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MgmtNwkUpdateNotify {
    // srcaddr: u16,
    // status: u8,
    // scannedchannels: u32,
    // totaltrans: u16,
    // transfails: u16,
    // energylength: u8,
    // energyvalues: Vec<u8>,
}

impl AsyncRequest for MgmtNwkUpdateNotify {
    const ID: u8 = 184;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StateChangeInd {
    state: u8,
}

impl AsyncRequest for StateChangeInd {
    const ID: u8 = 192;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndDeviceAnnceInd {
    // srcaddr: u16,
    // nwkaddr: u16,
    // ieeeaddr: IeeeAddr,
    // capabilities: u8,
}

impl AsyncRequest for EndDeviceAnnceInd {
    const ID: u8 = 193;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MatchDescRspSent {
    nwkaddr: u16,
    inclusterlist: Vec<u16>,
}

impl SyncReply for MatchDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusErrorRsp {
    srcaddr: u16,
    status: u8,
}

impl SyncReply for StatusErrorRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SrcRtgInd {
    dstaddr: u16,
    relaylist: Vec<u16>,
}

impl AsyncRequest for SrcRtgInd {
    const ID: u8 = 196;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BeaconNotifyInd {
    //// FIXME
    // beaconlist: Vec<u8>,
}

impl AsyncRequest for BeaconNotifyInd {
    const ID: u8 = 197;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JoinCnf {
    status: u8,
    deviceaddress: u16,
    parentaddress: u16,
}

impl AsyncRequest for JoinCnf {
    const ID: u8 = 198;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkDiscoveryCnf {
    status: u8,
}

impl AsyncRequest for NwkDiscoveryCnf {
    const ID: u8 = 199;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConcentratorIndCb {
    srcaddr: u16,
    extaddr: IeeeAddr,
    pkt_cost: u8,
}

impl AsyncRequest for ConcentratorIndCb {
    const ID: u8 = 200;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeaveInd {
    srcaddr: u16,
    extaddr: IeeeAddr,
    request: u8,
    removechildren: u8,
    rejoin: u8,
}

impl AsyncRequest for LeaveInd {
    const ID: u8 = 201;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SetRejoinParametersReq {
    backoffduration: u32,
    scanduration: u32,
}

impl SyncRequest for SetRejoinParametersReq {
    const ID: u8 = 204;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgCbIncoming {
    srcaddr: u16,
    wasbroadcast: u8,
    clusterid: u16,
    securityuse: u8,
    seqnum: u8,
    macdstaddr: u16,
    //// FIXME
    // msgdata: Vec<u8>,
}

impl AsyncRequest for MsgCbIncoming {
    const ID: u8 = 255;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndDeviceTimeoutReq {
    parentaddr: u16,
    reqrimeout: u16,
}

impl SyncRequest for EndDeviceTimeoutReq {
    const ID: u8 = 13;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SendData {
    shortaddr: u16,
    transseq: u8,
    cmd: u16,
    len: u8,
    buf: Vec<u8>,
}

impl SyncRequest for SendData {
    const ID: u8 = 40;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NwkAddrOfInterestReq {
    shortaddr: u16,
    nwkaddr: u16,
    cmd: u8,
}

impl SyncRequest for NwkAddrOfInterestReq {
    const ID: u8 = 41;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecAddLinkKey {
    shortaddr: u16,
    extaddr: IeeeAddr,
    linkkey: Vec<u8>,
}

impl SyncRequest for SecAddLinkKey {
    const ID: u8 = 66;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecEntryLookupExt {
    extaddr: IeeeAddr,
}

#[derive(Debug, Clone, Deserialize)]
struct SecEntryLookupExtReply {
    status: u8,
    ami: u16,
    keynvid: u16,
    authenticateoption: u8,
}

impl SyncReply for SecEntryLookupExtReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for SecEntryLookupExt {
    const ID: u8 = 67;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = SecEntryLookupExtReply;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecDeviceRemove {
    extaddr: IeeeAddr,
}

impl SyncRequest for SecDeviceRemove {
    const ID: u8 = 68;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtRouteDisc {
    dst_addr: u16,
    options: u8,
    radius: u8,
}

impl SyncRequest for ExtRouteDisc {
    const ID: u8 = 69;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtRouteCheck {
    dstaddr: u16,
    rtstatus: u8,
    options: u8,
}

impl SyncRequest for ExtRouteCheck {
    const ID: u8 = 70;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtRemoveGroup {
    endpoint: u8,
    groupid: u16,
}

impl SyncRequest for ExtRemoveGroup {
    const ID: u8 = 71;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtRemoveAllGroup {
    endpoint: u8,
}

impl SyncRequest for ExtRemoveAllGroup {
    const ID: u8 = 72;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtFindAllGroupsEndpoint {
    endpoint: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ExtFindAllGroupsEndpointReply {
    grouplist: Vec<u16>,
}

impl SyncReply for ExtFindAllGroupsEndpointReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ExtFindAllGroupsEndpoint {
    const ID: u8 = 73;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = ExtFindAllGroupsEndpointReply;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtFindGroup {
    endpoint: u8,
    groupid: u16,
}

#[derive(Debug, Clone, Deserialize)]
struct ExtFindGroupReply {
    status: u8,
    groupid: u16,
    namelen: u8,
    groupname: Vec<u8>,
}

impl SyncReply for ExtFindGroupReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ExtFindGroup {
    const ID: u8 = 74;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = ExtFindGroupReply;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtAddGroup {
    endpoint: u8,
    groupid: u16,
    namelen: u8,
    groupname: Vec<u8>,
}

impl SyncRequest for ExtAddGroup {
    const ID: u8 = 75;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtCountAllGroups {}

impl SyncRequest for ExtCountAllGroups {
    const ID: u8 = 76;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtRxIdle {
    setflag: u8,
    setvalue: u8,
}

impl SyncRequest for ExtRxIdle {
    const ID: u8 = 77;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtUpdateNwkKey {
    dstaddr: u16,
    keyseqnum: u8,
    key: Vec<u8>,
}

impl SyncRequest for ExtUpdateNwkKey {
    const ID: u8 = 78;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtSwitchNwkKey {
    dstaddr: u16,
    keyseqnum: u8,
}

impl SyncRequest for ExtSwitchNwkKey {
    const ID: u8 = 79;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtNwkInfo {}

#[derive(Debug, Clone, Deserialize)]
struct ExtNwkInfoRsp {
    shortaddr: u16,
    devstate: u8,
    panid: u16,
    parentaddr: u16,
    extendedpanid: IeeeAddr,
    parentextaddr: IeeeAddr,
    channel: u8,
}

impl SyncReply for ExtNwkInfoRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ExtNwkInfo {
    const ID: u8 = 80;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = ExtNwkInfoRsp;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtSecApsRemoveReq {
    parentaddr: u16,
    nwkaddr: u16,
    extaddr: IeeeAddr,
}

impl SyncRequest for ExtSecApsRemoveReq {
    const ID: u8 = 81;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ForceConcentratorChange {}

impl SyncRequest for ForceConcentratorChange {
    const ID: u8 = 82;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtSetParams {
    usemulticast: u8,
}

impl SyncRequest for ExtSetParams {
    const ID: u8 = 83;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TcDeviceInd {
    nwkaddr: u16,
    extaddr: IeeeAddr,
    parentaddr: u16,
}

impl AsyncRequest for TcDeviceInd {
    const ID: u8 = 202;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PermitJoinInd {
    duration: u8,
}

impl AsyncRequest for PermitJoinInd {
    const ID: u8 = 203;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}
