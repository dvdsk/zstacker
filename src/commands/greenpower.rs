#![allow(dead_code)]
use serde::Serialize;

use super::{Command, CommandType, IeeeAddr, Status, Subsystem};

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

impl Command for SecReq {
    const ID: u8 = 3;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::GreenPower;
    type Reply = Status;
}
