use serde::{Deserialize, Serialize};

use super::{
    Command, CommandReply, CommandType, DeviceState, DeviceType, IeeeAddr,
    ShortAddr, Subsystem,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetDeviceInfo;

impl Command for GetDeviceInfo {
    const ID: u8 = 0;
    const TYPE: CommandType = CommandType::SREQ;
    const SUBSYSTEM: Subsystem = Subsystem::Util;
    type Reply = DeviceInfo;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceInfo {
    status: u8,
    ieee_addr: IeeeAddr,
    short_addr: ShortAddr,
    #[serde(deserialize_with = "device_type_from_u8")]
    can_operate_as: Vec<DeviceType>, // bits 1-0
    device_state: DeviceState,
    assoc_devices: Vec<u16>,
}

impl CommandReply for DeviceInfo {
    const CMD0: u8 = 0x67;
    const CMD1: u8 = 0x00;
}

fn device_type_from_u8<'de, D>(
    deserializer: D,
) -> Result<Vec<DeviceType>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use strum::IntoEnumIterator;
    let bitset: u8 = serde::de::Deserialize::deserialize(deserializer)?;
    Ok(DeviceType::iter()
        .filter(|dev_type| bitset & (*dev_type as u8) > 0)
        .collect())
}
