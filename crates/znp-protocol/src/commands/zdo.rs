use serde::{Deserialize, Serialize};

use super::{AsyncRequest, IeeeAddr, SubSystem, SyncReply, SyncRequest};

use super::BasicStatus as Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwkAddrReq {
    pub ieeeaddr: IeeeAddr,
    pub reqtype: u8,
    pub startindex: u8,
}

impl SyncRequest for NwkAddrReq {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IeeeAddrReq {
    pub shortaddr: u16,
    pub reqtype: u8,
    pub startindex: u8,
}

impl SyncRequest for IeeeAddrReq {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
}

impl SyncRequest for NodeDescReq {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
}

impl SyncRequest for PowerDescReq {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
    pub endpoint: u8,
}

impl SyncRequest for SimpleDescReq {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEpReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
}

impl SyncRequest for ActiveEpReq {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
    pub profileid: u16,
    pub inclusterlist: Vec<u16>,
}

impl SyncRequest for MatchDescReq {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
}

impl SyncRequest for ComplexDescReq {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDescReq {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
}

impl SyncRequest for UserDescReq {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndDeviceAnnce {
    pub nwkaddr: u16,
    pub ieeeaddr: IeeeAddr,
    pub capability: u8,
}

impl SyncRequest for EndDeviceAnnce {
    const ID: u8 = 10;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDescSet {
    pub dstaddr: u16,
    pub nwkaddrofinterest: u16,
    pub descriptor_len: u8,
    pub userdescriptor: Vec<u8>,
}

impl SyncRequest for UserDescSet {
    const ID: u8 = 11;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDiscReq {
    pub servermask: u16,
}

impl SyncRequest for ServerDiscReq {
    const ID: u8 = 12;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndDeviceBindReq {
    pub dstaddr: u16,
    pub localcoord: u16,
    pub localieee: IeeeAddr,
    pub endpoint: u8,
    pub profileid: u16,
    pub inclusterlist: Vec<u16>,
}

impl SyncRequest for EndDeviceBindReq {
    const ID: u8 = 32;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindReq {
    pub dstaddr: u16,
    pub srcaddr: IeeeAddr,
    pub srcendpoint: u8,
    pub clusterid: u16,
    pub dstaddrmode: u8,
    pub dstaddress: IeeeAddr,
    pub dstendpoint: u8,
}

impl SyncRequest for BindReq {
    const ID: u8 = 33;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindReq {
    pub dstaddr: u16,
    pub srcaddr: IeeeAddr,
    pub srcendpoint: u8,
    pub clusterid: u16,
    pub dstaddrmode: u8,
    pub dstaddress: IeeeAddr,
    pub dstendpoint: u8,
}

impl SyncRequest for UnbindReq {
    const ID: u8 = 34;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLinkKey {
    pub shortaddr: u16,
    pub ieeeaddr: IeeeAddr,
    pub linkkey: Vec<u8>,
}

impl SyncRequest for SetLinkKey {
    const ID: u8 = 35;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveLinkKey {
    pub ieeeaddr: IeeeAddr,
}

impl SyncRequest for RemoveLinkKey {
    const ID: u8 = 36;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLinkKey {
    pub ieeeaddr: IeeeAddr,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetLinkKeyReply {
    pub status: u8,
    pub ieeeaddr: IeeeAddr,
    pub linkkeydata: [u8; 16],
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
pub struct NwkDiscoveryReq {
    pub scanchannels: u32,
    pub scanduration: u8,
}

impl SyncRequest for NwkDiscoveryReq {
    const ID: u8 = 38;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinReq {
    pub logicalchannel: u8,
    pub panid: u16,
    pub extendedpanid: IeeeAddr,
    pub chosenparent: u16,
    pub parentdepth: u8,
    pub stackprofile: u8,
}

impl SyncRequest for JoinReq {
    const ID: u8 = 39;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtNwkDiscReq {
    pub dstaddr: u16,
    pub scanchannels: u32,
    pub scanduration: u8,
    pub startindex: u8,
}

impl SyncRequest for MgmtNwkDiscReq {
    const ID: u8 = 48;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtLqiReq {
    pub dstaddr: u16,
    pub startindex: u8,
}

impl SyncRequest for MgmtLqiReq {
    const ID: u8 = 49;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtRtgReq {
    pub dstaddr: u16,
    pub startindex: u8,
}

impl SyncRequest for MgmtRtgReq {
    const ID: u8 = 50;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtBindReq {
    pub dstaddr: u16,
    pub startindex: u8,
}

impl SyncRequest for MgmtBindReq {
    const ID: u8 = 51;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtLeaveReq {
    pub dstaddr: u16,
    pub deviceaddress: IeeeAddr,
    pub removechildrenRejoin: u8,
}

impl SyncRequest for MgmtLeaveReq {
    const ID: u8 = 52;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtDirectJoinReq {
    pub dstaddr: u16,
    pub deviceaddr: IeeeAddr,
    pub capinfo: u8,
}

impl SyncRequest for MgmtDirectJoinReq {
    const ID: u8 = 53;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtPermitJoinReq {
    pub addrmode: u8,
    pub dstaddr: u16,
    pub duration: u8,
    pub tcsignificance: u8,
}

impl SyncRequest for MgmtPermitJoinReq {
    const ID: u8 = 54;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtNwkUpdateReq {
    pub dstaddr: u16,
    pub dstaddrmode: u8,
    pub channelmask: u32,
    pub scanduration: u8,
    // TODO: below two have various combinations of present/not present depending on scanduration
    pub scancount: u8,
    pub nwkmanageraddr: u16,
}

impl SyncRequest for MgmtNwkUpdateReq {
    const ID: u8 = 55; // the spec says 0x38 but TI used 0x37 see https://github.com/Koenkk/zigbee-herdsman/issues/1237
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgCbRegister {
    pub clusterid: u16,
}

impl SyncRequest for MsgCbRegister {
    const ID: u8 = 62;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgCbRemove {
    pub clusterid: u16,
}

impl SyncRequest for MsgCbRemove {
    const ID: u8 = 63;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupFromApp {
    pub startdelay: u16,
}

impl SyncRequest for StartupFromApp {
    const ID: u8 = 64;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoFindDestination {
    pub endpoint: u8,
}

impl AsyncRequest for AutoFindDestination {
    const ID: u8 = 65;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwkAddrRsp {
    pub status: u8,
    // Parse the ieeeaddr as it is needed for ZNP waitFor (see zStackAdapter.requestNetworkAddress())
    pub ieeeaddr: IeeeAddr,
    pub nwkaddr: u16,
    pub startindex: u8,
    pub assocdevlist: Vec<u16>,
}

impl SyncReply for NwkAddrRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IeeeAddrRsp {
    pub status: u8,
    pub ieeeaddr: IeeeAddr,
    pub nwkaddr: u16,
    pub startindex: u8,
    pub assocdevlist: Vec<u16>,
}

impl SyncReply for IeeeAddrRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub logicaltype_cmplxdescavai_userdescavai: u8,
    pub apsflags_freqband: u8,
    pub maccapflags: u8,
    pub manufacturercode: u16,
    pub maxbuffersize: u8,
    pub maxintransfersize: u16,
    pub servermask: u16,
    pub maxouttransfersize: u16,
    pub descriptorcap: u8,
}

impl SyncReply for NodeDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub currentpowermode_avaipowersrc: u8,
    pub currentpowersrc_currentpowersrclevel: u8,
}

impl SyncReply for PowerDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub len: u8,
    pub endpoint: u8,
    pub profileid: u16,
    pub deviceid: u16,
    pub deviceversion: u8,
    pub inclusterlist: Vec<u16>,
}

impl SyncReply for SimpleDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEpRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub activeeplist: Vec<u8>,
}

impl SyncReply for ActiveEpRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub matchlength: u8,
    pub matchlist: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub complexlength: u8,
    pub complexdesclist: Vec<u8>,
}

impl SyncReply for ComplexDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDescRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
    pub userlength: u8,
    pub userdescriptor: Vec<u8>,
}

impl SyncReply for UserDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDescConf {
    pub srcaddr: u16,
    pub status: u8,
    pub nwkaddr: u16,
}

impl AsyncRequest for UserDescConf {
    const ID: u8 = 137;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDiscRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub servermask: u16,
}

impl SyncReply for ServerDiscRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

/// https://github.com/Koenkk/zigbee2mqtt/issues/3363 {
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unknown {}

impl AsyncRequest for Unknown {
    const ID: u8 = 159;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndDeviceBindRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for EndDeviceBindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for BindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for UnbindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtNwkDiscRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub networkcount: u8,
    pub startindex: u8,
    pub networklist: Vec<Network>,
}

/// See: Z-Stack Monitor and Test API section 3.12.2.15 revision 1.14
/// Note this is not serialized like this in the wire format
/// Can not use serde to get this over the wire. Overwrite the default
/// `Command::data_to_vec` implementation instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    // PAN ID of the neighbor device
    pub pan_id: u16,
    // The current logical channel occupied by the network.
    pub logical_channel: u8,
    pub stack_profile: u8,     // 4th byte bits 3-0
    pub zigbee_version: u8,    // 4th byte bits 7-4
    pub beacon_order: u8,      // 4th byte bits 3-0
    pub super_frame_order: u8, // 5th byte bits 7-4
    pub permit_joining: bool,
}

impl SyncReply for MgmtNwkDiscRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtLqiRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub neighbortableentries: u8,
    pub startindex: u8,
    pub neighborlqilistcount: u8,
    pub neighborlqilist:
        compile_error!("needs custom derive with NeighborLqi type"),
}

impl SyncReply for MgmtLqiRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtRtgRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub routingtableentries: u8,
    pub startindex: u8,
    pub routingtablelistcount: u8,
    pub routingtablelist: RoutingTable,
}

impl SyncReply for MgmtRtgRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtBindRsp {
    pub srcaddr: u16,
    pub status: u8,
    pub bindingtableentries: u8,
    pub startindex: u8,
    pub bindingtablelistcount: u8,
    pub bindingtablelist: BindTable,
}

impl SyncReply for MgmtBindRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtLeaveRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for MgmtLeaveRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtDirectJoinRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for MgmtDirectJoinRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtPermitJoinRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for MgmtPermitJoinRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgmtNwkUpdateNotify {
    pub srcaddr: u16,
    pub status: u8,
    pub scannedchannels: u32,
    pub totaltrans: u16,
    pub transfails: u16,
    pub energylength: u8,
    pub energyvalues: Vec<u8>,
}

impl AsyncRequest for MgmtNwkUpdateNotify {
    const ID: u8 = 184;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangeInd {
    pub state: u8,
}

impl AsyncRequest for StateChangeInd {
    const ID: u8 = 192;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndDeviceAnnceInd {
    pub srcaddr: u16,
    pub nwkaddr: u16,
    pub ieeeaddr: IeeeAddr,
    pub capabilities: u8,
}

impl AsyncRequest for EndDeviceAnnceInd {
    const ID: u8 = 193;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchDescRspSent {
    pub nwkaddr: u16,
    pub inclusterlist: Vec<u16>,
}

impl SyncReply for MatchDescRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusErrorRsp {
    pub srcaddr: u16,
    pub status: u8,
}

impl SyncReply for StatusErrorRsp {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrcRtgInd {
    pub dstaddr: u16,
    pub relaylist: Vec<u16>,
}

impl AsyncRequest for SrcRtgInd {
    const ID: u8 = 196;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconNotifyInd {
    //// FIXME
    pub beaconlist: Vec<u8>,
}

impl AsyncRequest for BeaconNotifyInd {
    const ID: u8 = 197;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinCnf {
    pub status: u8,
    pub deviceaddress: u16,
    pub parentaddress: u16,
}

impl AsyncRequest for JoinCnf {
    const ID: u8 = 198;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwkDiscoveryCnf {
    pub status: u8,
}

impl AsyncRequest for NwkDiscoveryCnf {
    const ID: u8 = 199;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentratorIndCb {
    pub srcaddr: u16,
    pub extaddr: IeeeAddr,
    pub pkt_cost: u8,
}

impl AsyncRequest for ConcentratorIndCb {
    const ID: u8 = 200;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveInd {
    pub srcaddr: u16,
    pub extaddr: IeeeAddr,
    pub request: u8,
    pub removechildren: u8,
    pub rejoin: u8,
}

impl AsyncRequest for LeaveInd {
    const ID: u8 = 201;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRejoinParametersReq {
    pub backoffduration: u32,
    pub scanduration: u32,
}

impl SyncRequest for SetRejoinParametersReq {
    const ID: u8 = 204;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgCbIncoming {
    pub srcaddr: u16,
    pub wasbroadcast: u8,
    pub clusterid: u16,
    pub securityuse: u8,
    pub seqnum: u8,
    pub macdstaddr: u16,
    // FIXME
    pub msgdata: Vec<u8>,
}

impl AsyncRequest for MsgCbIncoming {
    const ID: u8 = 255;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndDeviceTimeoutReq {
    pub parentaddr: u16,
    pub reqrimeout: u16,
}

impl SyncRequest for EndDeviceTimeoutReq {
    const ID: u8 = 13;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendData {
    pub shortaddr: u16,
    pub transseq: u8,
    pub cmd: u16,
    pub len: u8,
    pub buf: Vec<u8>,
}

impl SyncRequest for SendData {
    const ID: u8 = 40;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NwkAddrOfInterestReq {
    pub shortaddr: u16,
    pub nwkaddr: u16,
    pub cmd: u8,
}

impl SyncRequest for NwkAddrOfInterestReq {
    const ID: u8 = 41;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecAddLinkKey {
    pub shortaddr: u16,
    pub extaddr: IeeeAddr,
    pub linkkey: Vec<u8>,
}

impl SyncRequest for SecAddLinkKey {
    const ID: u8 = 66;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecEntryLookupExt {
    pub extaddr: IeeeAddr,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecEntryLookupExtReply {
    pub status: u8,
    pub ami: u16,
    pub keynvid: u16,
    pub authenticateoption: u8,
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
pub struct SecDeviceRemove {
    pub extaddr: IeeeAddr,
}

impl SyncRequest for SecDeviceRemove {
    const ID: u8 = 68;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtRouteDisc {
    pub dst_addr: u16,
    pub options: u8,
    pub radius: u8,
}

impl SyncRequest for ExtRouteDisc {
    const ID: u8 = 69;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtRouteCheck {
    pub dstaddr: u16,
    pub rtstatus: u8,
    pub options: u8,
}

impl SyncRequest for ExtRouteCheck {
    const ID: u8 = 70;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtRemoveGroup {
    pub endpoint: u8,
    pub groupid: u16,
}

impl SyncRequest for ExtRemoveGroup {
    const ID: u8 = 71;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtRemoveAllGroup {
    pub endpoint: u8,
}

impl SyncRequest for ExtRemoveAllGroup {
    const ID: u8 = 72;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtFindAllGroupsEndpoint {
    pub endpoint: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtFindAllGroupsEndpointReply {
    pub grouplist: Vec<u16>,
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
pub struct ExtFindGroup {
    pub endpoint: u8,
    pub groupid: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtFindGroupReply {
    pub status: u8,
    pub groupid: u16,
    pub namelen: u8,
    pub groupname: Vec<u8>,
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
pub struct ExtAddGroup {
    pub endpoint: u8,
    pub groupid: u16,
    pub namelen: u8,
    pub groupname: Vec<u8>,
}

impl SyncRequest for ExtAddGroup {
    const ID: u8 = 75;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtCountAllGroups {}

impl SyncRequest for ExtCountAllGroups {
    const ID: u8 = 76;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtRxIdle {
    pub setflag: u8,
    pub setvalue: u8,
}

impl SyncRequest for ExtRxIdle {
    const ID: u8 = 77;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtUpdateNwkKey {
    pub dstaddr: u16,
    pub keyseqnum: u8,
    pub key: Vec<u8>,
}

impl SyncRequest for ExtUpdateNwkKey {
    const ID: u8 = 78;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtSwitchNwkKey {
    pub dstaddr: u16,
    pub keyseqnum: u8,
}

impl SyncRequest for ExtSwitchNwkKey {
    const ID: u8 = 79;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtNwkInfo {}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtNwkInfoRsp {
    pub shortaddr: u16,
    pub devstate: u8,
    pub panid: u16,
    pub parentaddr: u16,
    pub extendedpanid: IeeeAddr,
    pub parentextaddr: IeeeAddr,
    pub channel: u8,
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
pub struct ExtSecApsRemoveReq {
    pub parentaddr: u16,
    pub nwkaddr: u16,
    pub extaddr: IeeeAddr,
}

impl SyncRequest for ExtSecApsRemoveReq {
    const ID: u8 = 81;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceConcentratorChange {}

impl SyncRequest for ForceConcentratorChange {
    const ID: u8 = 82;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtSetParams {
    pub usemulticast: u8,
}

impl SyncRequest for ExtSetParams {
    const ID: u8 = 83;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcDeviceInd {
    pub nwkaddr: u16,
    pub extaddr: IeeeAddr,
    pub parentaddr: u16,
}

impl AsyncRequest for TcDeviceInd {
    const ID: u8 = 202;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermitJoinInd {
    pub duration: u8,
}

impl AsyncRequest for PermitJoinInd {
    const ID: u8 = 203;
    const SUBSYSTEM: SubSystem = SubSystem::Zdo;
}
