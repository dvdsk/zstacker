use serde_repr::Deserialize_repr;

/// The status parameter that is returned from the `ZNP` device
///
/// From: `Z-Stack ZNP Interface Specification.pdf` revision 1.1 (11/11/2016)
/// url: https://community.silabs.com/s/contentversion/0681M00000EWPKrQAP
/// viewed: 2025-04-15
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum Status {
    ZSuccess = 0x00,
    Zfailure = 0x01,
    ZinvalidParameter = 0x02,
    NvItemUninit = 0x09,
    NvOperFailed = 0x0a,
    NvBadItemLen = 0x0c,
    ZmemError = 0x10,
    ZbufferFull = 0x11,
    ZunsupportedMode = 0x12,
    ZmacMemError = 0x13,
    ZdoInvalidRequestType = 0x80,
    ZdoInvalidEndpoint = 0x82,
    ZdoUnsupported = 0x84,
    ZdoTimeout = 0x85,
    ZdoNoMatch = 0x86,
    ZdoTableFull = 0x87,
    ZdoNoBindEntry = 0x88,
    ZsecNoKey = 0xa1,
    ZsecMaxFrmCount = 0xa3,
    ZapsFail = 0xb1,
    ZapsTableFull = 0xb2,
    ZapsIllegalRequest = 0xb3,
    ZapsInvalidBinding = 0xb4,
    ZapsUnsupportedAttrib = 0xb5,
    ZapsNotSupported = 0xb6,
    ZapsNoAck = 0xb7,
    ZapsDuplicateEntry = 0xb8,
    ZapsNoBoundDevice = 0xb9,
    ZnwkInvalidParam = 0xc1,
    ZnwkInvalidRequest = 0xc2,
    ZnwkNotPermitted = 0xc3,
    ZnwkStartupFailure = 0xc4,
    ZnwkTableFull = 0xc7,
    ZnwkUnknownDevice = 0xc8,
    ZnwkUnsupportedAttribute = 0xc9,
    ZnwkNoNetworks = 0xca,
    ZnwkLeaveUnconfirmed = 0xcb,
    ZnwkNoAck = 0xcc,
    ZnwkNoRoute = 0xcd,
    ZMacNoACK = 0xe9,
}
