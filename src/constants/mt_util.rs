#[derive(Clone, Copy)]
pub enum MtUtilCommandId {
    UtilGetDeviceInfo = 0x00,
    UtilGetNvInfo = 0x01,
    UtilSetPanid = 0x02,
    UtilSetChannels = 0x03,
    UtilSetSeclevel = 0x04,
    UtilSetPrecfgkey = 0x05,
    UtilCallbackSubCmd = 0x06,
    UtilKeyEvent = 0x07,
    UtilTimeAlive = 0x09,
    UtilLedControl = 0x0A,
    UtilLoopback = 0x10,
    UtilDataReq = 0x11,
    UtilSrcMatchEnable = 0x20,
    UtilSrcMatchAddEntry = 0x21,
    UtilSrcMatchDelEntry = 0x22,
    UtilSrcMatchCheckSrcAddr = 0x23,
    UtilSrcMatchAckAllPending = 0x24,
    UtilSrcMatchCheckAllPending = 0x25,
    UtilAddrmgrExtAddrLookup = 0x40,
    UtilAddrmgrNwkAddrLookup = 0x41,
    UtilApsmeLinkKeyDataGet = 0x44,
    UtilApsmeLinkKeyNvIdGet = 0x45,
    UtilApsmeRequestKeyCmd = 0x4B,
    UtilAssocCount = 0x48,
    UtilAssocFindDevice = 0x49,
    UtilAssocGetWithAddress = 0x4A,
    UtilBindAddEntry = 0x4D,
    UtilZclKeyEstInitEst = 0x80,
    UtilZclKeyEstSign = 0x81,
    UtilSrngGen = 0x4C,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtUtilCallbackId {
    UtilSyncReq = 0xE0,
    UtilZclKeyEstablishInd = 0xE1,
    // TODO - implement ParseByte
}
