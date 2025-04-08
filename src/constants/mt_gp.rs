#[derive(Clone, Copy)]
pub enum MtGPCommandId {
    GpDataReq = 0x01,
    GpSecRsp = 0x02,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtGPCallbackId {
    GpDataCnf = 0x05,
    GpSecReq = 0x03,
    GpDataInd = 0x04,
    // TODO - implement ParseByte
}
