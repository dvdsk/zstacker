#[derive(Clone, Copy)]
pub enum MtAppConfigCommandId {
    AppCnfSetNwkFrameCounter = 0xFF,
    AppCnfSetDefaultRemoteEnddeviceTimeout = 0x01,
    AppCnfSetEnddevicetimeout = 0x02,
    AppCnfSetAllowrejoinTcPolicy = 0x03,
    AppCnfBdbStartCommissioning = 0x05,
    AppCnfBdbSetChannel = 0x08,
    AppCnfBdbAddInstallcode = 0x04,
    AppCnfBdbSetTcRequireKeyExchange = 0x09,
    AppCnfBdbSetJoinusesinstallcodekey = 0x06,
    AppCnfBdbSetActiveDefaultCentralizedKey = 0x07,
    AppCnfBdbZedAttemptRecoverNwk = 0x0A,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtAppConfigCallbackId {
    AppCnfBdbCommissioningNotification = 0x80,
    // TODO - implement ParseByte
}
