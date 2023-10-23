#[derive(Clone, Copy)]
pub enum MtMacCommandId {
    MAC_RESET_REQ = 0x01,
    MAC_INIT = 0x02,
    MAC_START_REQ = 0x03,
    MAC_SYNC_REQ = 0x04,
    MAC_DATA_REQ = 0x05,
    MAC_ASSOCIATE_REQ = 0x06,
    MAC_ASSOCIATE_RSP = 0x50,
    MAC_DISASSOCIATE_REQ = 0x07,
    MAC_GET_REQ = 0x08,
    MAC_SET_REQ = 0x09,
    MAC_SCAN_REQ = 0x0C,
    MAC_ORPHAN_RSP = 0x51,
    MAC_POLL_REQ = 0x0D,
    MAC_PURGE_REQ = 0x0E,
    MAC_SET_RX_GAIN_REQ = 0x0F,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtMacCallbackId {
    MAC_SYNC_LOSS_IND = 0x80,
    MAC_ASSOCIATE_IND = 0x81,
    MAC_ASSOCIATE_CNF = 0x82,
    MAC_BEACON_NOTIFY_IND = 0x83,
    MAC_DATA_CNF = 0x84,
    MAC_DATA_IND = 0x85,
    MAC_DISASSOCIATE_IND = 0x86,
    MAC_DISASSOCIATE_CNF = 0x87,
    MAC_ORPHAN_IND = 0x8A,
    MAC_POLL_CNF = 0x8B,
    MAC_SCAN_CNF = 0x8C,
    MAC_COMM_STATUS_IND = 0x8D,
    MAC_START_CNF = 0x8E,
    MAC_RX_ENABLE_CNF = 0x8F,
    MAC_PURGE_CNF = 0x9A,
    // TODO - implement ParseByte
}
