use crate::api::ParseByte;
mod command_subsystem;
mod command_type;

pub const SOF: u8 = 0xFE;
pub use crate::constants::command_subsystem::MtCommandSubsystem;
pub use crate::constants::command_type::MtCommandType;

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

#[derive(Clone, Copy)]
pub enum MtZdoCommandId {
    ZDO_NWK_ADDR_REQ = 0x00,
    ZDO_IEEE_ADDR_REQ = 0x01,
    ZDO_NODE_DESC_REQ = 0x02,
    ZDO_POWER_DESC_REQ = 0x03,
    ZDO_SIMPLE_DESC_REQ = 0x04,
    ZDO_ACTIVE_EP_REQ = 0x05,
    ZDO_MATCH_DESC_REQ = 0x06,
    ZDO_COMPLEX_DESC_REQ = 0x07,
    ZDO_USER_DESC_REQ = 0x08,
    ZDO_END_DEVICE_ANNCE = 0x0A,
    ZDO_USER_DESC_SET = 0x0B,
    ZDO_SERVER_DISC_REQ = 0x0C,
    ZDO_END_DEVICE_BIND_REQ = 0x20,
    ZDO_BIND_REQ = 0x21,
    ZDO_UNBIND_REQ = 0x22,
    ZDO_MGMT_NWK_DISC_REQ = 0x30,
    ZDO_MGMT_LQI_REQ = 0x31,
    ZDO_MGMT_RTG_REQ = 0x32,
    ZDO_MGMT_BIND_REQ = 0x33,
    ZDO_MGMT_LEAVE_REQ = 0x34,
    ZDO_MGMT_DIRECT_JOIN_REQ = 0x35,
    ZDO_MGMT_PERMIT_JOIN_REQ = 0x36,
    ZDO_MGMT_NWK_UPDATE_REQ = 0x37,
    ZDO_MSG_CB_REGISTER = 0x3E,
    ZDO_MSG_CB_REMOVE = 0x3F,
    ZDO_STARTUP_FROM_APP = 0x40,
    ZDO_STARTUP_FROM_APP_EX = 0x54,
    ZDO_SET_LINK_KEY = 0x23,
    ZDO_REMOVE_LINK_KEY = 0x24,
    ZDO_GET_LINK_KEY = 0x25,
    ZDO_NWK_DISCOVERY_REQ = 0x26,
    ZDO_JOIN_REQ = 0x27,
    // TODO - validate
    // ZDO_SET_REJOIN_PARAMETERS = 0x26,
    ZDO_SEC_ADD_LINK_KEY = 0x42,
    ZDO_SEC_ENTRY_LOOKUP_EXT = 0x43,
    ZDO_SEC_DEVICE_REMOVE = 0x44,
    ZDO_EXT_ROUTE_DISC = 0x45,
    ZDO_EXT_ROUTE_CHECK = 0x46,
    ZDO_EXT_REMOVE_GROUP = 0x47,
    ZDO_EXT_REMOVE_ALL_GROUP = 0x48,
    ZDO_EXT_FIND_ALL_GROUPS_ENDPOINT = 0x49,
    ZDO_EXT_FIND_GROUP = 0x4A,
    ZDO_EXT_ADD_GROUP = 0x4B,
    ZDO_EXT_COUNT_ALL_GROUPS = 0x4C,
    ZDO_EXT_RX_IDLE = 0x4D,
    ZDO_EXT_UPDATE_NWK_KEY = 0x4E,
    ZDO_EXT_SWITCH_NWK_KEY = 0x4F,
    ZDO_EXT_NWK_INFO = 0x50,
    ZDO_EXT_SEC_APS_REMOVE_REQ = 0x51,
    ZDO_FORCE_CONCENTRATOR_CHANGE = 0x52,
    ZDO_EXT_SET_PARAMS = 0x53,
    ZDO_NWK_ADDR_OF_INTEREST_REQ = 0x29,
    // TODO - implement ParseByte
}

#[derive(Clone, Copy)]
pub enum MtZdoCallbackId {
    ZDO_NWK_ADDR_RSP = 0x80,
    ZDO_IEEE_ADDR_RSP = 0x81,
    ZDO_NODE_DESC_RSP = 0x82,
    ZDO_POWER_DESC_RSP = 0x83,
    ZDO_SIMPLE_DESC_RSP = 0x84,
    ZDO_ACTIVE_EP_RSP = 0x85,
    ZDO_MATCH_DESC_RSP = 0x86,
    ZDO_COMPLEX_DESC_RSP = 0x87,
    ZDO_USER_DESC_RSP = 0x88,
    ZDO_USER_DESC_CONF = 0x89,
    ZDO_SERVER_DISC_RSP = 0x8A,
    ZDO_END_DEVICE_BIND_RSP = 0xA0,
    ZDO_BIND_RSP = 0xA1,
    ZDO_UNBIND_RSP = 0xA2,
    ZDO_MGMT_NWK_DISC_RSP = 0xB0,
    ZDO_MGMT_LQI_RSP = 0xB1,
    ZDO_MGMT_RTG_RSP = 0xB2,
    ZDO_MGMT_BIND_RSP = 0xB3,
    ZDO_MGMT_LEAVE_RSP = 0xB4,
    ZDO_MGMT_DIRECT_JOIN_RSP = 0xB5,
    ZDO_MGMT_PERMIT_JOIN_RSP = 0xB6,
    ZDO_STATE_CHANGE_IND = 0xC0,
    ZDO_END_DEVICE_ANNCE_IND = 0xC1,
    ZDO_MATCH_DESC_RSP_SENT = 0xC2,
    ZDO_STATUS_ERROR_RSP = 0xC3,
    ZDO_SRC_RTG_IND = 0xC4,
    ZDO_BEACON_NOTIFY_IND = 0xC5,
    ZDO_JOIN_CNF = 0xC6,
    ZDO_NWK_DISCOVERY_CNF = 0xC7,
    ZDO_LEAVE_IND = 0xC9,
    ZDO_MSG_CB_INCOMING = 0xFF,
    ZDO_TC_DEV_IND = 0xCA,
    ZDO_PERMIT_JOIN_IND = 0xCB,
    // TODO - implement ParseByte
}
