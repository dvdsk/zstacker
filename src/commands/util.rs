use serde::{Deserialize, Serialize};

use super::{
    Command, CommandType, DeviceState, DeviceType, IeeeAddr, ShortAddr,
    Subsystem,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetDeviceInfo;

impl Command for GetDeviceInfo {
    const ID: u8 = 0;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::UtilInterface;
    type Reply = DeviceInfo;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceInfo {
    status: u8,
    ieee_addr: IeeeAddr,
    short_addr: ShortAddr,
    device_type: DeviceType,
    device_state: DeviceState,
    assoc_devices: Vec<u16>,
}
