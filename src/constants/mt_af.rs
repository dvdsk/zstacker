#[derive(Clone, Copy)]
pub enum MtAFCommandId {
    AfRegister = 0x00,
    AfDataRequest = 0x01,
    AfDataRequestExt = 0x02,
    AfDataRequestSrcRtg = 0x03,
    AfInterPanCtl = 0x10,
    AfDataStore = 0x11,
    AfDataRetrieve = 0x12,
    AfApsfConfigSet = 0x13,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtAFCallbackId {
    AfDataConfirm = 0x80,
    AfReflectError = 0x83,
    AfIncomingMsg = 0x81,
    AfIncomingMsgExt = 0x82,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum LatencyReq {
    NoLatency = 0x00,
    FastBeacons = 0x01,
    SlowBeacons = 0x02,
    // TODO - implement ParseByte
}
