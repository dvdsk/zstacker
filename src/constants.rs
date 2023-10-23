use crate::api::ParseByte;

pub const SOF: u8 = 0xFE;

#[derive(Clone, Copy)]
pub enum MtCommandType {
    POLL = 0x00,
    SREQ = 0x20,
    AREQ = 0x40,
    SRSP = 0x60,
}

// TODO - use derive(ParseByte)
impl ParseByte<Self> for MtCommandType {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtCommandType::POLL),
            0x20 => Some(MtCommandType::SREQ),
            0x40 => Some(MtCommandType::AREQ),
            0x60 => Some(MtCommandType::SRSP),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MtCommandSubsystem {
    Reserved = 0x00,
    SYSInterface = 0x01,
    MACInterface = 0x02,
    NWKInterface = 0x03,
    AFInterface = 0x04,
    ZDOInterface = 0x05,
    SAPIInterface = 0x06,
    UTILInterface = 0x07,
    DEBUGInterface = 0x08,
    APPInterface = 0x09,
    APPConfig = 0x0F,
    GreenPower = 0x15,
}

// TODO - use derive(ParseByte)
impl ParseByte<Self> for MtCommandSubsystem {
    fn parse_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(MtCommandSubsystem::Reserved),
            0x01 => Some(MtCommandSubsystem::SYSInterface),
            0x02 => Some(MtCommandSubsystem::MACInterface),
            0x03 => Some(MtCommandSubsystem::NWKInterface),
            0x04 => Some(MtCommandSubsystem::AFInterface),
            0x05 => Some(MtCommandSubsystem::ZDOInterface),
            0x06 => Some(MtCommandSubsystem::SAPIInterface),
            0x07 => Some(MtCommandSubsystem::UTILInterface),
            0x08 => Some(MtCommandSubsystem::DEBUGInterface),
            0x09 => Some(MtCommandSubsystem::APPInterface),
            0x0F => Some(MtCommandSubsystem::APPConfig),
            0x15 => Some(MtCommandSubsystem::GreenPower),
            _ => None,
        }
    }
}

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

#[derive(Clone, Copy)]
pub enum MtUtilCommandId {
    UTIL_GET_DEVICE_INFO = 0x00,
    UTIL_GET_NV_INFO = 0x01,
    UTIL_SET_PANID = 0x02,
    UTIL_SET_CHANNELS = 0x03,
    UTIL_SET_SECLEVEL = 0x04,
    UTIL_SET_PRECFGKEY = 0x05,
    UTIL_CALLBACK_SUB_CMD = 0x06,
    UTIL_KEY_EVENT = 0x07,
    UTIL_TIME_ALIVE = 0x09,
    UTIL_LED_CONTROL = 0x0A,
    UTIL_LOOPBACK = 0x10,
    UTIL_DATA_REQ = 0x11,
    UTIL_SRC_MATCH_ENABLE = 0x20,
    UTIL_SRC_MATCH_ADD_ENTRY = 0x21,
    UTIL_SRC_MATCH_DEL_ENTRY = 0x22,
    UTIL_SRC_MATCH_CHECK_SRC_ADDR = 0x23,
    UTIL_SRC_MATCH_ACK_ALL_PENDING = 0x24,
    UTIL_SRC_MATCH_CHECK_ALL_PENDING = 0x25,
    UTIL_ADDRMGR_EXT_ADDR_LOOKUP = 0x40,
    UTIL_ADDRMGR_NWK_ADDR_LOOKUP = 0x41,
    UTIL_APSME_LINK_KEY_DATA_GET = 0x44,
    UTIL_APSME_LINK_KEY_NV_ID_GET = 0x45,
    UTIL_APSME_REQUEST_KEY_CMD = 0x4B,
    UTIL_ASSOC_COUNT = 0x48,
    UTIL_ASSOC_FIND_DEVICE = 0x49,
    UTIL_ASSOC_GET_WITH_ADDRESS = 0x4A,
    UTIL_BIND_ADD_ENTRY = 0x4D,
    UTIL_ZCL_KEY_EST_INIT_EST = 0x80,
    UTIL_ZCL_KEY_EST_SIGN = 0x81,
    UTIL_SRNG_GEN = 0x4C,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtUtilCallbackId {
    UTIL_SYNC_REQ = 0xE0,
    UTIL_ZCL_KEY_ESTABLISH_IND = 0xE1,
    // TODO - implement ParseByte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mt_command_subystem() {
        assert_eq!(MtCommandSubsystem::Reserved as u8, 0x00);
        assert_eq!(MtCommandSubsystem::SYSInterface as u8, 0x01);
        assert_eq!(MtCommandSubsystem::MACInterface as u8, 0x02);
        assert_eq!(MtCommandSubsystem::NWKInterface as u8, 0x03);
        assert_eq!(MtCommandSubsystem::AFInterface as u8, 0x04);
        assert_eq!(MtCommandSubsystem::ZDOInterface as u8, 0x05);
        assert_eq!(MtCommandSubsystem::SAPIInterface as u8, 0x06);
        assert_eq!(MtCommandSubsystem::UTILInterface as u8, 0x07);
        assert_eq!(MtCommandSubsystem::DEBUGInterface as u8, 0x08);
        assert_eq!(MtCommandSubsystem::APPInterface as u8, 0x09);
        assert_eq!(MtCommandSubsystem::APPConfig as u8, 0x0F);
        assert_eq!(MtCommandSubsystem::GreenPower as u8, 0x15);
    }

    #[test]
    fn test_mt_command_type() {
        assert_eq!(MtCommandType::POLL as u8, 0x00);
        assert_eq!(MtCommandType::SREQ as u8, 0x20);
        assert_eq!(MtCommandType::AREQ as u8, 0x40);
        assert_eq!(MtCommandType::SRSP as u8, 0x60);
    }
}
