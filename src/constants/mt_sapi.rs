#[derive(Clone, Copy)]
pub enum MtSAPICommandId {
    ZbSystemReset = 0x09,
    ZbStartRequest = 0x00,
    ZbPermitJoiningRequest = 0x08,
    ZbBindDevice = 0x01,
    ZbAllowBind = 0x02,
    ZbSendDataRequest = 0x03,
    ZbReadConfiguration = 0x04,
    ZbWriteConfiguration = 0x05,
    ZbGetDeviceInfo = 0x06,
    ZbFindDeviceRequest = 0x07,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtSAPICallbackId {
    ZbStartConfirm = 0x80,
    ZbBindConfirm = 0x81,
    ZbAllowBindConfirm = 0x82,
    ZbSendDataConfirm = 0x83,
    ZbReceiveDataIndication = 0x87,
    ZbFindDeviceConfirm = 0x85,
    // TODO - implement ParseByte
}
