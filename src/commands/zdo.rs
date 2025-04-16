#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{Command, CommandType, IeeeAddr, Status, Subsystem};

#[derive(Debug, Clone, Serialize)]
struct NwkAddrReq {
    // ieeeaddr: IeeeAddr,
    // reqtype: u8,
    // startindex: u8,
}

impl Command for NwkAddrReq {
    const ID: u8 = 0;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct IeeeAddrReq {
    // shortaddr: u16,
    // reqtype: u8,
    // startindex: u8,
}

impl Command for IeeeAddrReq {
    const ID: u8 = 1;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NodeDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl Command for NodeDescReq {
    const ID: u8 = 2;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct PowerDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl Command for PowerDescReq {
    const ID: u8 = 3;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SimpleDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
    // endpoint: u8,
}

impl Command for SimpleDescReq {
    const ID: u8 = 4;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ActiveEpReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
}

impl Command for ActiveEpReq {
    const ID: u8 = 5;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MatchDescReq {
    // dstaddr: u16,
    // nwkaddrofinterest: u16,
    // profileid: u16,
    // inclusterlist: Vec<u16>,
}

impl Command for MatchDescReq {
    const ID: u8 = 6;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ComplexDescReq {
    dstaddr: u16,
    nwkaddrofinterest: u16,
}

impl Command for ComplexDescReq {
    const ID: u8 = 7;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct UserDescReq {
    dstaddr: u16,
    nwkaddrofinterest: u16,
}

impl Command for UserDescReq {
    const ID: u8 = 8;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct EndDeviceAnnce {
    nwkaddr: u16,
    ieeeaddr: IeeeAddr,
    capability: u8,
}

impl Command for EndDeviceAnnce {
    const ID: u8 = 10;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct UserDescSet {
    dstaddr: u16,
    nwkaddrofinterest: u16,
    descriptor_len: u8,
    userdescriptor: Vec<u8>,
}

impl Command for UserDescSet {
    const ID: u8 = 11;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ServerDiscReq {
    // servermask: u16,
}

impl Command for ServerDiscReq {
    const ID: u8 = 12;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct EndDeviceBindReq {
    dstaddr: u16,
    localcoord: u16,
    localieee: IeeeAddr,
    endpoint: u8,
    profileid: u16,
    inclusterlist: Vec<u16>,
}

impl Command for EndDeviceBindReq {
    const ID: u8 = 32;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct BindReq {
    // dstaddr: u16,
    // srcaddr: IeeeAddr,
    // srcendpoint: u8,
    // clusterid: u16,
    // dstaddrmode: u8,
    // dstaddress: IeeeAddr,
    // dstendpoint: u8,
}

impl Command for BindReq {
    const ID: u8 = 33;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct UnbindReq {
    // dstaddr: u16,
    // srcaddr: IeeeAddr,
    // srcendpoint: u8,
    // clusterid: u16,
    // dstaddrmode: u8,
    // dstaddress: IeeeAddr,
    // dstendpoint: u8,
}

impl Command for UnbindReq {
    const ID: u8 = 34;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SetLinkKey {
    shortaddr: u16,
    ieeeaddr: IeeeAddr,
    linkkey: Vec<u8>,
}

impl Command for SetLinkKey {
    const ID: u8 = 35;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct RemoveLinkKey {
    ieeeaddr: IeeeAddr,
}

impl Command for RemoveLinkKey {
    const ID: u8 = 36;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct GetLinkKey {
    ieeeaddr: IeeeAddr,
}

#[derive(Debug, Clone, Deserialize)]
struct GetLinkKeyReply {
    status: u8,
    ieeeaddr: IeeeAddr,
    linkkeydata: [u8; 16],
}

impl Command for GetLinkKey {
    const ID: u8 = 37;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = GetLinkKeyReply;
}

#[derive(Debug, Clone, Serialize)]
struct NwkDiscoveryReq {
    scanchannels: u32,
    scanduration: u8,
}

impl Command for NwkDiscoveryReq {
    const ID: u8 = 38;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct JoinReq {
    logicalchannel: u8,
    panid: u16,
    extendedpanid: IeeeAddr,
    chosenparent: u16,
    parentdepth: u8,
    stackprofile: u8,
}

impl Command for JoinReq {
    const ID: u8 = 39;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtNwkDiscReq {
    dstaddr: u16,
    scanchannels: u32,
    scanduration: u8,
    startindex: u8,
}

impl Command for MgmtNwkDiscReq {
    const ID: u8 = 48;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtLqiReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl Command for MgmtLqiReq {
    const ID: u8 = 49;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtRtgReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl Command for MgmtRtgReq {
    const ID: u8 = 50;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtBindReq {
    // dstaddr: u16,
    // startindex: u8,
}

impl Command for MgmtBindReq {
    const ID: u8 = 51;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtLeaveReq {
    // dstaddr: u16,
    // deviceaddress: IeeeAddr,
    // removechildrenRejoin: u8,
}

impl Command for MgmtLeaveReq {
    const ID: u8 = 52;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtDirectJoinReq {
    dstaddr: u16,
    deviceaddr: IeeeAddr,
    capinfo: u8,
}

impl Command for MgmtDirectJoinReq {
    const ID: u8 = 53;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtPermitJoinReq {
    // addrmode: u8,
    // dstaddr: u16,
    // duration: u8,
    // tcsignificance: u8,
}

impl Command for MgmtPermitJoinReq {
    const ID: u8 = 54;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MgmtNwkUpdateReq {
    // dstaddr: u16,
    // dstaddrmode: u8,
    // channelmask: u32,
    // scanduration: u8,
    //     // TODO: below two have various combinations of present/not present depending on scanduration
    // scancount: u8,
    // nwkmanageraddr: u16,
}

impl Command for MgmtNwkUpdateReq {
    const ID: u8 = 55; // the spec says 0x38 but TI used 0x37 see https://github.com/Koenkk/zigbee-herdsman/issues/1237
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MsgCbRegister {
    clusterid: u16,
}

impl Command for MsgCbRegister {
    const ID: u8 = 62;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct MsgCbRemove {
    clusterid: u16,
}

impl Command for MsgCbRemove {
    const ID: u8 = 63;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct StartupFromApp {
    startdelay: u16,
}

impl Command for StartupFromApp {
    const ID: u8 = 64;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct AutoFindDestination {
    endpoint: u8,
}

impl Command for AutoFindDestination {
    const ID: u8 = 65;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct NwkAddrRsp {
    // status: u8,
    // Parse the ieeeaddr as it is needed for ZNP waitFor (see zStackAdapter.requestNetworkAddress())
    // ieeeaddr: IeeeAddr,
    // nwkaddr: u16,
    // startindex: u8,
    // assocdevlist: Vec<u16>,
}

impl Command for NwkAddrRsp {
    const ID: u8 = 128;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct IeeeAddrRsp {
    // status: u8,
    // ieeeaddr: IeeeAddr,
    // nwkaddr: u16,
    // startindex: u8,
    // assocdevlist: Vec<u16>,
}

impl Command for IeeeAddrRsp {
    const ID: u8 = 129;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for NodeDescRsp {
    const ID: u8 = 130;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct PowerDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // currentpowermode_avaipowersrc: u8,
    // currentpowersrc_currentpowersrclevel: u8,
}

impl Command for PowerDescRsp {
    const ID: u8 = 131;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for SimpleDescRsp {
    const ID: u8 = 132;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ActiveEpRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // activeeplist: Vec<u8>,
}

impl Command for ActiveEpRsp {
    const ID: u8 = 133;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MatchDescRsp {
    // srcaddr: u16,
    // status: u8,
    // nwkaddr: u16,
    // matchlength: u8,
    // matchlist: Vec<u8>,
}

impl Command for MatchDescRsp {
    const ID: u8 = 134;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ComplexDescRsp {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
    complexlength: u8,
    complexdesclist: Vec<u8>,
}

impl Command for ComplexDescRsp {
    const ID: u8 = 135;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct UserDescRsp {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
    userlength: u8,
    userdescriptor: Vec<u8>,
}

impl Command for UserDescRsp {
    const ID: u8 = 136;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct UserDescConf {
    srcaddr: u16,
    status: u8,
    nwkaddr: u16,
}

impl Command for UserDescConf {
    const ID: u8 = 137;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ServerDiscRsp {
    // srcaddr: u16,
    // status: u8,
    // servermask: u16,
}

impl Command for ServerDiscRsp {
    const ID: u8 = 138;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

/// https://github.com/Koenkk/zigbee2mqtt/issues/3363 {
#[derive(Debug, Clone, Serialize)]
struct Unknown {}

impl Command for Unknown {
    const ID: u8 = 159;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct EndDeviceBindRsp {
    srcaddr: u16,
    status: u8,
}

impl Command for EndDeviceBindRsp {
    const ID: u8 = 160;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct BindRsp {
    // srcaddr: u16,
    // status: u8,
}

impl Command for BindRsp {
    const ID: u8 = 161;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct UnbindRsp {
    // srcaddr: u16,
    // status: u8,
}

impl Command for UnbindRsp {
    const ID: u8 = 162;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for MgmtNwkDiscRsp {
    const ID: u8 = 176;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();

    fn data_to_vec(&self) -> Result<Vec<u8>, crate::data_format::Error>
    where
        Self: Sized,
    {
        unimplemented!("only received never send")
    }
}

#[derive(Debug, Clone, Serialize)]
struct MgmtLqiRsp {
    // srcaddr: u16,
    // status: u8,
    // neighbortableentries: u8,
    // startindex: u8,
    // neighborlqilistcount: u8,
    // neighborlqilist: compile_error!("needs custom derive with NeighborLqi type"),
}

impl Command for MgmtLqiRsp {
    const ID: u8 = 177;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtRtgRsp {
    // srcaddr: u16,
    // status: u8,
    // routingtableentries: u8,
    // startindex: u8,
    // routingtablelistcount: u8,
    // routingtablelist: RoutingTable,
}

impl Command for MgmtRtgRsp {
    const ID: u8 = 178;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtBindRsp {
    // srcaddr: u16,
    // status: u8,
    // bindingtableentries: u8,
    // startindex: u8,
    // bindingtablelistcount: u8,
    // bindingtablelist: BindTable,
}

impl Command for MgmtBindRsp {
    const ID: u8 = 179;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtLeaveRsp {
    // srcaddr: u16,
    // status: u8,
}

impl Command for MgmtLeaveRsp {
    const ID: u8 = 180;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtDirectJoinRsp {
    srcaddr: u16,
    status: u8,
}

impl Command for MgmtDirectJoinRsp {
    const ID: u8 = 181;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtPermitJoinRsp {
    // srcaddr: u16,
    // status: u8,
}

impl Command for MgmtPermitJoinRsp {
    const ID: u8 = 182;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MgmtNwkUpdateNotify {
    // srcaddr: u16,
    // status: u8,
    // scannedchannels: u32,
    // totaltrans: u16,
    // transfails: u16,
    // energylength: u8,
    // energyvalues: Vec<u8>,
}

impl Command for MgmtNwkUpdateNotify {
    const ID: u8 = 184;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct StateChangeInd {
    state: u8,
}

impl Command for StateChangeInd {
    const ID: u8 = 192;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct EndDeviceAnnceInd {
    // srcaddr: u16,
    // nwkaddr: u16,
    // ieeeaddr: IeeeAddr,
    // capabilities: u8,
}

impl Command for EndDeviceAnnceInd {
    const ID: u8 = 193;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct MatchDescRspSent {
    nwkaddr: u16,
    inclusterlist: Vec<u16>,
}

impl Command for MatchDescRspSent {
    const ID: u8 = 194;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct StatusErrorRsp {
    srcaddr: u16,
    status: u8,
}

impl Command for StatusErrorRsp {
    const ID: u8 = 195;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct SrcRtgInd {
    dstaddr: u16,
    relaylist: Vec<u16>,
}

impl Command for SrcRtgInd {
    const ID: u8 = 196;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct BeaconNotifyInd {
    //// FIXME
    // beaconlist: Vec<u8>,
}

impl Command for BeaconNotifyInd {
    const ID: u8 = 197;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct JoinCnf {
    status: u8,
    deviceaddress: u16,
    parentaddress: u16,
}

impl Command for JoinCnf {
    const ID: u8 = 198;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct NwkDiscoveryCnf {
    status: u8,
}

impl Command for NwkDiscoveryCnf {
    const ID: u8 = 199;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ConcentratorIndCb {
    srcaddr: u16,
    extaddr: IeeeAddr,
    pkt_cost: u8,
}

impl Command for ConcentratorIndCb {
    const ID: u8 = 200;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct LeaveInd {
    srcaddr: u16,
    extaddr: IeeeAddr,
    request: u8,
    removechildren: u8,
    rejoin: u8,
}

impl Command for LeaveInd {
    const ID: u8 = 201;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct SetRejoinParametersReq {
    backoffduration: u32,
    scanduration: u32,
}

impl Command for SetRejoinParametersReq {
    const ID: u8 = 204;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for MsgCbIncoming {
    const ID: u8 = 255;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct EndDeviceTimeoutReq {
    parentaddr: u16,
    reqrimeout: u16,
}

impl Command for EndDeviceTimeoutReq {
    const ID: u8 = 13;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SendData {
    shortaddr: u16,
    transseq: u8,
    cmd: u16,
    len: u8,
    buf: Vec<u8>,
}

impl Command for SendData {
    const ID: u8 = 40;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct NwkAddrOfInterestReq {
    shortaddr: u16,
    nwkaddr: u16,
    cmd: u8,
}

impl Command for NwkAddrOfInterestReq {
    const ID: u8 = 41;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct SecAddLinkKey {
    shortaddr: u16,
    extaddr: IeeeAddr,
    linkkey: Vec<u8>,
}

impl Command for SecAddLinkKey {
    const ID: u8 = 66;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for SecEntryLookupExt {
    const ID: u8 = 67;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = SecEntryLookupExtReply;
}

#[derive(Debug, Clone, Serialize)]
struct SecDeviceRemove {
    extaddr: IeeeAddr,
}

impl Command for SecDeviceRemove {
    const ID: u8 = 68;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtRouteDisc {
    dst_addr: u16,
    options: u8,
    radius: u8,
}

impl Command for ExtRouteDisc {
    const ID: u8 = 69;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtRouteCheck {
    dstaddr: u16,
    rtstatus: u8,
    options: u8,
}

impl Command for ExtRouteCheck {
    const ID: u8 = 70;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtRemoveGroup {
    endpoint: u8,
    groupid: u16,
}

impl Command for ExtRemoveGroup {
    const ID: u8 = 71;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtRemoveAllGroup {
    endpoint: u8,
}

impl Command for ExtRemoveAllGroup {
    const ID: u8 = 72;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtFindAllGroupsEndpoint {
    endpoint: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ExtFindAllGroupsEndpointReply {
    grouplist: Vec<u16>,
}

impl Command for ExtFindAllGroupsEndpoint {
    const ID: u8 = 73;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ExtFindAllGroupsEndpointReply;
}

#[derive(Debug, Clone, Serialize)]
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

impl Command for ExtFindGroup {
    const ID: u8 = 74;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ExtFindGroupReply;
}

#[derive(Debug, Clone, Serialize)]
struct ExtAddGroup {
    endpoint: u8,
    groupid: u16,
    namelen: u8,
    groupname: Vec<u8>,
}

impl Command for ExtAddGroup {
    const ID: u8 = 75;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtCountAllGroups {}

impl Command for ExtCountAllGroups {
    const ID: u8 = 76;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtRxIdle {
    setflag: u8,
    setvalue: u8,
}

impl Command for ExtRxIdle {
    const ID: u8 = 77;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtUpdateNwkKey {
    dstaddr: u16,
    keyseqnum: u8,
    key: Vec<u8>,
}

impl Command for ExtUpdateNwkKey {
    const ID: u8 = 78;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtSwitchNwkKey {
    dstaddr: u16,
    keyseqnum: u8,
}

impl Command for ExtSwitchNwkKey {
    const ID: u8 = 79;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ExtNwkInfo {}

#[derive(Debug, Clone, Deserialize)]
struct ExtNwkInfoReply {
    shortaddr: u16,
    devstate: u8,
    panid: u16,
    parentaddr: u16,
    extendedpanid: IeeeAddr,
    parentextaddr: IeeeAddr,
    channel: u8,
}

impl Command for ExtNwkInfo {
    const ID: u8 = 80;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ExtNwkInfoReply;
}

#[derive(Debug, Clone, Serialize)]
struct ExtSecApsRemoveReq {
    parentaddr: u16,
    nwkaddr: u16,
    extaddr: IeeeAddr,
}

impl Command for ExtSecApsRemoveReq {
    const ID: u8 = 81;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ForceConcentratorChange {}

impl Command for ForceConcentratorChange {
    const ID: u8 = 82;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ExtSetParams {
    usemulticast: u8,
}

impl Command for ExtSetParams {
    const ID: u8 = 83;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct TcDeviceInd {
    nwkaddr: u16,
    extaddr: IeeeAddr,
    parentaddr: u16,
}

impl Command for TcDeviceInd {
    const ID: u8 = 202;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct PermitJoinInd {
    duration: u8,
}

impl Command for PermitJoinInd {
    const ID: u8 = 203;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Zdo;
    type Reply = ();
}
