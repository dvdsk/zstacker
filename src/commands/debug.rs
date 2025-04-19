use serde::{Deserialize, Serialize};

use super::{AsyncRequest, Status, SubSystem, SyncReply, SyncRequest};

#[derive(Debug, Clone, Serialize)]
pub struct SetThreshold {
    pub componentid: u8,
    pub threshold: u8,
}

impl SyncRequest for SetThreshold {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Debug;
    type Reply = SetThresholdReply;
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetThresholdReply(pub Status);

impl SyncReply for SetThresholdReply {
    type Request = SetThreshold;
}

#[derive(Debug, Clone, Serialize)]
pub struct Msg {
    pub length: u8,
    pub string: Vec<u8>,
}

impl AsyncRequest for Msg {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Debug;
}
