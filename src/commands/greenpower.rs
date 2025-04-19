use serde::{Deserialize, Serialize};

use super::{IeeeAddr, Status, SubSystem, SyncReply, SyncRequest};

#[derive(Debug, Clone, Serialize)]
pub struct SecReq {
    pub application_id: u8,
    pub src_id: u32,
    pub gdp_ieee_addr: IeeeAddr,
    pub endpoint: u8,
    pub gpdf_security_level: u8,
    pub gpdf_security_frame_counter: u8,
    pub dgp_stub_handle: u8,
}

impl SyncRequest for SecReq {
    const ID: u8 = 3;
    const SUBSYSTEM: SubSystem = SubSystem::GreenPower;
    type Reply = SecReqReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecReqReply(pub Status);

impl SyncReply for SecReqReply {
    type Request = SecReq;
}
