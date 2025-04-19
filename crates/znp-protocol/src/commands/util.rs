use serde::{Deserialize, Serialize};

use super::{
    DeviceState, DeviceType, IeeeAddr, ShortAddr, SubSystem, SyncReply,
    SyncRequest,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetDeviceInfo;

impl SyncRequest for GetDeviceInfo {
    const ID: u8 = 0;
    const SUBSYSTEM: SubSystem = SubSystem::Util;
    type Reply = DeviceInfo;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceInfo {
    pub status: u8,
    pub ieee_addr: IeeeAddr,
    pub short_addr: ShortAddr,
    #[serde(deserialize_with = "device_type_from_u8")]
    pub can_operate_as: Vec<DeviceType>, // bits 1-0
    pub device_state: DeviceState,
    pub assoc_devices: Vec<u16>,
}

impl SyncReply for DeviceInfo {
    type Request = GetDeviceInfo;
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
