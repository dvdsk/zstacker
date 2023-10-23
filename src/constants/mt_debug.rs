#[derive(Clone, Copy)]
pub enum MtDebugCommandId {
    DEBUG_SET_THRESHOLD = 0x00,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtDebugAsyncCommandId {
    DEBUG_MSG = 0x00,
    // TODO - implement ParseByte
}
