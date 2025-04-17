#![allow(dead_code)]
use serde::Serialize;

use super::{AsyncRequest, SyncRequest, Status, SubSystem};

#[derive(Debug, Clone, Serialize)]
struct SetThreshold {
    componentid: u8,
    threshold: u8,
}

impl SyncRequest for SetThreshold {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Debug;
    type Reply = Status;
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
