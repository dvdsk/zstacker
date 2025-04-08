#[derive(Clone, Copy)]
pub enum MtDebugCommandId {
    DebugSetThreshold = 0x00,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtDebugAsyncCommandId {
    DebugMsg = 0x00,
    // TODO - implement ParseByte
}
