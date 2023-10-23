#[derive(Clone, Copy)]
pub enum MtGPCommandId {
    GP_DATA_REQ = 0x01,
    GP_SEC_RSP = 0x02,
    // TODO - implement ParseByte
}


#[derive(Clone, Copy)]
pub enum MtGPCallbackId {
    GP_DATA_CNF = 0x05,
    GP_SEC_REQ = 0x03,
    GP_DATA_IND = 0x04,
    // TODO - implement ParseByte
}