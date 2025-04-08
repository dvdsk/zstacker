#[derive(Clone, Copy)]
pub enum MtMacCommandId {
    MacResetReq = 0x01,
    MacInit = 0x02,
    MacStartReq = 0x03,
    MacSyncReq = 0x04,
    MacAssociateReq = 0x06,
    MacDataReq = 0x05,
    MacAssociateRsp = 0x50,
    MacDisassociateReq = 0x07,
    MacGetReq = 0x08,
    MacSetReq = 0x09,
    MacScanReq = 0x0C,
    MacOrphanRsp = 0x51,
    MacPollReq = 0x0D,
    MacPurgeReq = 0x0E,
    MacSetRxGainReq = 0x0F,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtMacCallbackId {
    MacSyncLossInd = 0x80,
    MacAssociateInd = 0x81,
    MacAssociateCnf = 0x82,
    MacBeaconNotifyInd = 0x83,
    MacDataCnf = 0x84,
    MacDataInd = 0x85,
    MacDisassociateInd = 0x86,
    MacDisassociateCnf = 0x87,
    MacOrphanInd = 0x8A,
    MacPollCnf = 0x8B,
    MacScanCnf = 0x8C,
    MacCommStatusInd = 0x8D,
    MacStartCnf = 0x8E,
    MacRxEnableCnf = 0x8F,
    MacPurgeCnf = 0x9A,
    // TODO - implement ParseByte
}
