#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::{AsyncRequest, Status, SubSystem, SyncReply, SyncRequest};

#[derive(Debug, Clone, Serialize)]
struct SetThreshold {
    componentid: u8,
    threshold: u8,
}

impl SyncRequest for SetThreshold {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Debug;
    type Reply = SetThresholdReply;
}

#[derive(Debug, Clone, Deserialize)]
struct SetThresholdReply(Status);

impl SyncReply for SetThresholdReply {
    type Request = SetThreshold;
}

#[derive(Debug, Clone, Serialize)]
struct Msg {
    length: u8,
    string: Vec<u8>,
}

impl AsyncRequest for Msg {
    const ID: u8 = 128;
    const SUBSYSTEM: SubSystem = SubSystem::Debug;
}
