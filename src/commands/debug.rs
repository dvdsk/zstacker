#![allow(dead_code)]
use serde::Serialize;

use super::{Command, CommandType, Status, Subsystem};

#[derive(Debug, Clone, Serialize)]
struct SetThreshold {
    componentid: u8,
    threshold: u8,
}

impl Command for SetThreshold {
    const ID: u8 = 0;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Debug;
    type Reply = Status;
}

#[derive(Debug, Clone, Serialize)]
struct Msg {
    length: u8,
    string: Vec<u8>,
}

impl Command for Msg {
    const ID: u8 = 128;
    const TYPE: CommandType = CommandType::AREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Debug;
    type Reply = ();
}
