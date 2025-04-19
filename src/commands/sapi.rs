use serde::{Deserialize, Serialize};

use super::{
    AsyncRequest, IeeeAddr, Status, SubSystem, SyncReply, SyncRequest,
};

#[derive(Debug, Clone, Serialize)]
pub struct SystemReset {}

impl AsyncRequest for SystemReset {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct StartRequest {}

impl SyncRequest for StartRequest {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = StartRequestReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartRequestReply;
impl SyncReply for StartRequestReply {
    type Request = StartRequest;
}

#[derive(Debug, Clone, Serialize)]
pub struct BindDevice {
    pub action: u8,
    pub commandid: u16,
    pub destination: IeeeAddr,
}

impl SyncRequest for BindDevice {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = BindDeviceReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BindDeviceReply;

impl SyncReply for BindDeviceReply {
    type Request = BindDevice;
}

#[derive(Debug, Clone, Serialize)]
pub struct AllowBind {
    pub timeout: u8,
}

impl SyncRequest for AllowBind {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = AllowBindReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllowBindReply;

impl SyncReply for AllowBindReply {
    type Request = AllowBind;
}

#[derive(Debug, Clone, Serialize)]
pub struct SendDataRequest {
    pub destination: u16,
    pub commandid: u16,
    pub handle: u8,
    pub txoptions: u8,
    pub radius: u8,
    pub payloadlen: u8,
    pub payloadvalue: Vec<u8>,
}

impl SyncRequest for SendDataRequest {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = SendDataRequestReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendDataRequestReply;

impl SyncReply for SendDataRequestReply {
    type Request = SendDataRequest;
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadConfiguration {
    pub configid: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ReadConfigurationReply {
    pub status: u8,
    pub configid: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncReply for ReadConfigurationReply {
    type Request = ReadConfiguration;
}

impl SyncRequest for ReadConfiguration {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ReadConfigurationReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct WriteConfiguration {
    pub configid: u8,
    pub len: u8,
    pub value: Vec<u8>,
}

impl SyncRequest for WriteConfiguration {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = WriteConfigurationReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WriteConfigurationReply(pub Status);

impl SyncReply for WriteConfigurationReply {
    type Request = WriteConfiguration;
}

#[derive(Debug, Clone, Serialize)]
pub struct GetDeviceInfo {
    pub param: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDeviceInfoReply {
    pub param: u8,
    pub value: [u8; 8],
}

impl SyncReply for GetDeviceInfoReply {
    type Request = GetDeviceInfo;
}

impl SyncRequest for GetDeviceInfo {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = GetDeviceInfoReply;
}

#[derive(Debug, Clone, Serialize)]
pub struct FindDeviceRequest {
    pub search_key: IeeeAddr,
}

impl SyncRequest for FindDeviceRequest {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = FindDeviceRequestReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct FindDeviceRequestReply;

impl SyncReply for FindDeviceRequestReply {
    type Request = FindDeviceRequest;
}

#[derive(Debug, Clone, Serialize)]
pub struct PermitJoiningRequest {
    pub destination: u16,
    pub timeout: u8,
}

impl SyncRequest for PermitJoiningRequest {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = PermitJoiningRequestReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermitJoiningRequestReply(pub Status);

impl SyncReply for PermitJoiningRequestReply {
    type Request = PermitJoiningRequest;
}

#[derive(Debug, Clone, Serialize)]
pub struct StartConfirm {
    pub status: u8,
}

impl AsyncRequest for StartConfirm {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct BindConfirm {
    pub commandid: u16,
    pub status: u8,
}

impl AsyncRequest for BindConfirm {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct AllowBindConfirm {
    pub source: u16,
}

impl AsyncRequest for AllowBindConfirm {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct SendDataConfirm {
    pub handle: u8,
    pub status: u8,
}

impl AsyncRequest for SendDataConfirm {
    const ID: u8 = 131;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct FindDeviceConfirm {
    pub searchtype: u8,
    pub searchkey: u16,
    pub result: IeeeAddr,
}

impl AsyncRequest for FindDeviceConfirm {
    const ID: u8 = 133;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiveDataIndication {
    pub source: u16,
    pub command: u16,
    pub len: u16,
    pub data: Vec<u8>,
}

impl AsyncRequest for ReceiveDataIndication {
    const ID: u8 = 135;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}
