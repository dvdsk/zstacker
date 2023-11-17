/// Based on [TI Docs](https://software-dl.ti.com/simplelink/esd/simplelink_cc26x2_sdk/2.30.00.34/exports/docs/zstack/html/zigbee/znp_interface.html#device-specific-configuration-parameters)
pub enum DeviceSpecificConfigurationItem {
    ZCD_NV_STARTUP_OPTION = 0x0003,
    ZCD_NV_LOGICAL_TYPE = 0x0087,
    ZCD_NV_ZDO_DIRECT_CB = 0x008F,
}

pub enum NetworkSpecificConfigurationItem {
    ZCD_NV_PANID = 0x0083,
}

pub enum NvStartupOptionBitMask {
    ZCD_STARTOPT_CLEAR_NWK_FRAME_COUNTER = 0x80,
    ZCD_STARTOPT_CLEAR_STATE = 0x02,
    ZCD_STARTOPT_CLEAR_CONFIG = 0x01,
}

pub enum NvLogicalType {
    ZG_DEVICETYPE_COORDINATOR = 0x00,
    ZG_DEVICETYPE_ROUTER = 0x01,
    ZG_DEVICETYPE_ENDDEVICE = 0x02,
}

pub enum ResetRequestType {
    HardwareReset = 0x00,
    SoftReset = 0x01,
}
