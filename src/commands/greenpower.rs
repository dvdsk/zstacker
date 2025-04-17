#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{IeeeAddr, Status, SubSystem, SyncReply, SyncRequest};

#[derive(Debug, Clone, Serialize)]
struct SecReq {
    application_id: u8,
    src_id: u32,
    gdp_ieee_addr: IeeeAddr,
    endpoint: u8,
    gpdf_security_level: u8,
    gpdf_security_frame_counter: u8,
    dgp_stub_handle: u8,
}

impl SyncRequest for SecReq {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::GreenPower;
    type Reply = SecReqReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SecReqReply(Status);

impl SyncReply for SecReqReply {
    type Request = SecReq;
}
