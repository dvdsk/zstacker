#[derive(Clone, Copy)]
pub enum MtSAPICommandId {
    ZB_SYSTEM_RESET = 0x09,
    ZB_START_REQUEST = 0x00,
    ZB_PERMIT_JOINING_REQUEST = 0x08,
    ZB_BIND_DEVICE = 0x01,
    ZB_ALLOW_BIND = 0x02,
    ZB_SEND_DATA_REQUEST = 0x03,
    ZB_READ_CONFIGURATION = 0x04,
    ZB_WRITE_CONFIGURATION = 0x05,
    ZB_GET_DEVICE_INFO = 0x06,
    ZB_FIND_DEVICE_REQUEST = 0x07,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtSAPICallbackId {
    ZB_START_CONFIRM = 0x80,
    ZB_BIND_CONFIRM = 0x81,
    ZB_ALLOW_BIND_CONFIRM = 0x82,
    ZB_SEND_DATA_CONFIRM = 0x83,
    ZB_RECEIVE_DATA_INDICATION = 0x87,
    ZB_FIND_DEVICE_CONFIRM = 0x85,
    // TODO - implement ParseByte
}
