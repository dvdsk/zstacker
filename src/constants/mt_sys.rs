use crate::api::ParseByte;

#[derive(Clone, Copy)]
pub enum MtSysCommandId {
    SysResetReq = 0x00,
    SysPing = 0x01,
    SysVersion = 0x02,
    SysSetExtaddr = 0x03,
    SysGetExtaddr = 0x04,
    SysRamRead = 0x05,
    SysRamWrite = 0x06,
    SysOsalNvRead = 0x08,
    SysOsalNvWrite = 0x09,
    SysOsalNvItemInit = 0x07,
    SysOsalNvDelete = 0x12,
    SysOsalNvLength = 0x13,
    SysOsalStartTimer = 0x0A,
    SysOsalStopTimer = 0x0B,
    SysRandom = 0x0C,
    SysADCRead = 0x0D,
    SysGIO = 0x0E,
    SysStackTune = 0x0F,
    SysSetTime = 0x10,
    SysGetTime = 0x11,
    SysSetTxPower = 0x14,
    SysZDiagsInitStats = 0x17,
    SysZDiagsClearStats = 0x18,
    SysZDiagsGetStats = 0x19,
    SysZDiagsRestoreStatsNv = 0x1A,
    SysZDiagsSaveStatsToNv = 0x1B,
    SysNvCreate = 0x30,
    SysNvDelete = 0x31,
    SysNvLength = 0x32,
    SysNvRead = 0x33,
    SysNvWrite = 0x34,
    SysNvUpdate = 0x35,
    SysNvCompact = 0x36,
}

// TODO - use derive(ParseByte)
impl ParseByte<Self> for MtSysCommandId {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtSysCommandId::SysResetReq),
            0x01 => Some(MtSysCommandId::SysPing),
            0x02 => Some(MtSysCommandId::SysVersion),
            0x03 => Some(MtSysCommandId::SysSetExtaddr),
            0x04 => Some(MtSysCommandId::SysGetExtaddr),
            0x05 => Some(MtSysCommandId::SysRamRead),
            0x06 => Some(MtSysCommandId::SysRamWrite),
            0x08 => Some(MtSysCommandId::SysOsalNvRead),
            0x09 => Some(MtSysCommandId::SysOsalNvWrite),
            0x07 => Some(MtSysCommandId::SysOsalNvItemInit),
            0x12 => Some(MtSysCommandId::SysOsalNvDelete),
            0x13 => Some(MtSysCommandId::SysOsalNvLength),
            0x0A => Some(MtSysCommandId::SysOsalStartTimer),
            0x0B => Some(MtSysCommandId::SysOsalStopTimer),
            0x0C => Some(MtSysCommandId::SysRandom),
            0x0D => Some(MtSysCommandId::SysADCRead),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MtSysCallbackId {
    SysResetInd = 0x80,
    SysOsalTimerExpired = 0x81,
    // TODO - implement ParseByte
}
