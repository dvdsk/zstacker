/// Based on [TI Docs](https://software-dl.ti.com/simplelink/esd/simplelink_cc26x2_sdk/2.30.00.34/exports/docs/zstack/html/zigbee/znp_interface.html#device-specific-configuration-parameters)
pub enum DeviceSpecificConfigurationItem {
    ZcdNvStartupOption = 0x0003,
    ZcdNvLogicalType0x0087,
    ZcdNvZdoDirectCb = 0x008F,
}

pub enum NetworkSpecificConfigurationItem {
    ZcdNvPanid = 0x0083,
}

pub enum NvStartupOptionBitMask {
    ZcdStartoptClearNwkFrameCounter = 0x80,
    ZcdStartoptClearState = 0x02,
    ZcdStartoptClearConfig = 0x01,
}

pub enum NvLogicalType {
    ZgDevicetypeCoordinator = 0x00,
    ZgDevicetypeRouter = 0x01,
    ZgDevicetypeEnddevice = 0x02,
}

pub enum ResetRequestType {
    HardwareReset = 0x00,
    SoftReset = 0x01,
}
