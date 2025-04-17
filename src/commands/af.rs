#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{AsyncRequest, SyncRequest, SyncReply, CommandType, IeeeAddr, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
struct Register {
    endpoint: u8,
    appprofid: u16,
    appdeviceid: u16,
    appdevver: u8,
    latencyreq: u8,
    appinclusterlist: Vec<u16>,
}

impl SyncRequest for Register {
    const ID: u8 = 0;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct DataRequest {
    dstaddr: u16,
    destendpoint: u8,
    srcendpoint: u8,
    clusterid: u16,
    transid: u8,
    options: u8,
    radius: u8,
    len: u8,
    data: Vec<u8>,
}

impl SyncRequest for DataRequest {
    const ID: u8 = 1;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct DataRequestExt {
    dstaddrmode: u8,
    dstaddr: IeeeAddr,
    destendpoint: u8,
    dstpanid: u16,
    srcendpoint: u8,
    clusterid: u16,
    transid: u8,
    options: u8,
    radius: u8,
    len: u16,
    data: Vec<u8>,
}

impl SyncRequest for DataRequestExt {
    const ID: u8 = 2;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct DataRequestSrcRtg {
    dstaddr: u16,
    destendpoint: u8,
    srcendpoint: u8,
    clusterid: u16,
    transid: u8,
    options: u8,
    radius: u8,
    relaylist: Vec<u16>,
    len: u8,
    data: Vec<u8>,
}

impl SyncRequest for DataRequestSrcRtg {
    const ID: u8 = 3;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct Delete {
    endpoint: u8,
}

impl SyncRequest for Delete {
    const ID: u8 = 4;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct InterPanCtl {
    cmd: u8,
    data: Vec<u8>,
}

impl SyncRequest for InterPanCtl {
    const ID: u8 = 16;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct DataStore {
    index: u16,
    length: u8,
    data: Vec<u8>,
}

impl SyncRequest for DataStore {
    const ID: u8 = 17;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct DataRetrieve {
    timestamp: u32,
    index: u16,
    length: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct DataRetrieveReply {
    status: u8,
    length: u8,
    data: Vec<u8>,
}

impl SyncReply for DataRetrieveReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for DataRetrieve {
    const ID: u8 = 18;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = DataRetrieveReply;
}

#[derive(Debug, Clone, Serialize)]
struct ApsfConfigSet {
    endpoint: u8,
    framedelay: u8,
    windowsize: u8,
}

impl SyncRequest for ApsfConfigSet {
    const ID: u8 = 19;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct ApsfConfigGet {
    endpoint: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ApsfConfigGetReply {
    framedelay: u8,
    windowsize: u8,
    nomean: u8,
}

impl SyncReply for ApsfConfigGetReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ApsfConfigGet {
    const ID: u8 = 20;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
    type Reply = ApsfConfigGetReply;
}

#[derive(Debug, Clone, Serialize)]
struct DataConfirm {
    status: u8,
    endpoint: u8,
    transid: u8,
}

impl AsyncRequest for DataConfirm {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
struct IncomingMsg {
    groupid: u16,
    clusterid: u16,
    srcaddr: u16,
    srcendpoint: u8,
    dstendpoint: u8,
    wasbroadcast: u8,
    linkquality: u8,
    securityuse: u8,
    timestamp: u32,
    transseqnumber: u8,
    len: u8,
    data: Vec<u8>,
}

impl AsyncRequest for IncomingMsg {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
struct IncomingMsgExt {
    groupid: u16,
    clusterid: u16,
    srcaddrmode: u8,
    srcaddr: IeeeAddr,
    srcendpoint: u8,
    srcpanid: u16,
    dstendpoint: u8,
    wasbroadcast: u8,
    linkquality: u8,
    securityuse: u8,
    timestamp: u32,
    transseqnumber: u8,
    len: u16,
    data: Vec<u8>,
}

impl AsyncRequest for IncomingMsgExt {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}

#[derive(Debug, Clone, Serialize)]
struct ReflectError {
    status: u8,
    endpoint: u8,
    transid: u8,
    dstaddrmode: u8,
    dstaddr: u16,
}

impl AsyncRequest for ReflectError {
    const ID: u8 = 131;
    const SUBSYSTEM: SubSystem = SubSystem::Af;
}
