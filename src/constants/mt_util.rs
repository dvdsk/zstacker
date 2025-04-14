// source: https://github.com/Koenkk/zigbee-herdsman/blob/4911b7ebbaf0e01d85cf1ebad6eece771484710d/src/adapter/z-stack/znp/definition.ts#L2195

#[derive(Clone, Copy)]
pub enum UtilCommandId {
    GetDeviceInfo = 0x00,
    GetNvInfo = 0x01,
    SetPanid = 0x02,
    SetChannels = 0x03,
    SetSeclevel = 0x04,
    SetPrecfgkey = 0x05,
    CallbackSubCmd = 0x06,
    KeyEvent = 0x07,
    TimeAlive = 0x09,
    LedControl = 0x0A,
    Loopback = 0x10,
    DataReq = 0x11,
    SrcMatchEnable = 0x20,
    SrcMatchAddEntry = 0x21,
    SrcMatchDelEntry = 0x22,
    SrcMatchCheckSrcAddr = 0x23,
    SrcMatchAckAllPending = 0x24,
    SrcMatchCheckAllPending = 0x25,
    AddrmgrExtAddrLookup = 0x40,
    AddrmgrNwkAddrLookup = 0x41,
    ApsmeLinkKeyDataGet = 0x44,
    ApsmeLinkKeyNvIdGet = 0x45,
    ApsmeRequestKeyCmd = 0x4B,
    AssocCount = 0x48,
    AssocFindDevice = 0x49,
    AssocGetWithAddress = 0x4A,
    BindAddEntry = 0x4D,
    ZclKeyEstInitEst = 0x80,
    ZclKeyEstSign = 0x81,
    SrngGen = 0x4C,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtUtilCallbackId {
    UtilSyncReq = 0xE0,
    UtilZclKeyEstablishInd = 0xE1,
    // TODO - implement ParseByte
}
