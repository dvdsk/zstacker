#[derive(Clone, Copy)]
pub enum MtAFCommandId {
    AF_REGISTER = 0x00,
    AF_DATA_REQUEST = 0x01,
    AF_DATA_REQUEST_EXT = 0x02,
    AF_DATA_REQUEST_SRC_RTG = 0x03,
    AF_INTER_PAN_CTL = 0x10,
    AF_DATA_STORE = 0x11,
    AF_DATA_RETRIEVE = 0x12,
    AF_APSF_CONFIG_SET = 0x13,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtAFCallbackId {
    AF_DATA_CONFIRM = 0x80,
    AF_REFLECT_ERROR = 0x83,
    AF_INCOMING_MSG = 0x81,
    AF_INCOMING_MSG_EXT = 0x82,
    // TODO - implement ParseByte
}
