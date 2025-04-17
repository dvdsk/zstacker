#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{AsyncRequest, SyncRequest, SyncReply, IeeeAddr, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
struct SystemReset {}

impl AsyncRequest for SystemReset {
    const ID: u8 = 9;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct StartRequest {}

impl SyncRequest for StartRequest {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct BindDevice {
    action: u8,
    commandid: u16,
    destination: IeeeAddr,
}

impl SyncRequest for BindDevice {
    const ID: u8 = 1;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct AllowBind {
    timeout: u8,
}

impl SyncRequest for AllowBind {
    const ID: u8 = 2;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct SendDataRequest {
    destination: u16,
    commandid: u16,
    handle: u8,
    txoptions: u8,
    radius: u8,
    payloadlen: u8,
    payloadvalue: Vec<u8>,
}

impl SyncRequest for SendDataRequest {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct ReadConfiguration {
    configid: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct ReadConfigurationReply {
    status: u8,
    configid: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncReply for ReadConfigurationReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for ReadConfiguration {
    const ID: u8 = 4;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ReadConfigurationReply;
}

#[derive(Debug, Clone, Serialize)]
struct WriteConfiguration {
    configid: u8,
    len: u8,
    value: Vec<u8>,
}

impl SyncRequest for WriteConfiguration {
    const ID: u8 = 5;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct GetDeviceInfo {
    param: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct GetDeviceInfoReply {
    param: u8,
    value: [u8; 8],
}

impl SyncReply for GetDeviceInfoReply {
    const CMD0: u8 = 0; // placeholder
    const CMD1: u8 = 0; // placeholder
}

impl SyncRequest for GetDeviceInfo {
    const ID: u8 = 6;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = GetDeviceInfoReply;
}

#[derive(Debug, Clone, Serialize)]
struct FindDeviceRequest {
    search_key: IeeeAddr,
}

impl SyncRequest for FindDeviceRequest {
    const ID: u8 = 7;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = ();
}

#[derive(Debug, Clone, Serialize)]
struct PermitJoiningRequest {
    destination: u16,
    timeout: u8,
}

impl SyncRequest for PermitJoiningRequest {
    const ID: u8 = 8;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct StartConfirm {
    status: u8,
}

impl AsyncRequest for StartConfirm {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct BindConfirm {
    commandid: u16,
    status: u8,
}

impl AsyncRequest for BindConfirm {
    const ID: u8 = 129;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct AllowBindConfirm {
    source: u16,
}

impl AsyncRequest for AllowBindConfirm {
    const ID: u8 = 130;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct SendDataConfirm {
    handle: u8,
    status: u8,
}

impl AsyncRequest for SendDataConfirm {
    const ID: u8 = 131;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct FindDeviceConfirm {
    searchtype: u8,
    searchkey: u16,
    result: IeeeAddr,
}

impl AsyncRequest for FindDeviceConfirm {
    const ID: u8 = 133;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}

#[derive(Debug, Clone, Serialize)]
struct ReceiveDataIndication {
    source: u16,
    command: u16,
    len: u16,
    data: Vec<u8>,
}

impl AsyncRequest for ReceiveDataIndication {
    const ID: u8 = 135;
    const SUBSYSTEM: SubSystem = SubSystem::Sapi;
}
