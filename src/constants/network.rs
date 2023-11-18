#[derive(Clone, Copy)]
pub enum ScanChannels {
    None = 0x00000000,
    AllChannels = 0x07FFF800,
    Channel11 = 0x00000800,
    Channel12 = 0x00001000,
    Channel13 = 0x00002000,
    Channel14 = 0x00004000,
    Channel15 = 0x00008000,
    Channel16 = 0x00010000,
    Channel17 = 0x00020000,
    Channel18 = 0x00040000,
    Channel19 = 0x00080000,
    Channel20 = 0x00100000,
    Channel21 = 0x00200000,
    Channel22 = 0x00400000,
    Channel23 = 0x00800000,
    Channel24 = 0x01000000,
    Channel25 = 0x02000000,
    Channel26 = 0x04000000,
}

/// Values from Zigbee spec Section 2.2.4.1.3 APSDE-DATA.indication Table 2-4 DstAddrMode.
#[derive(Clone, Copy)]
pub enum DestinationAddressMode {
    AddrNotPresent = 0x00,
    AddrGroup = 0x01,
    Addr16Bit = 0x02,
    Addr64Bit = 0x03,
    ZStackAddrBroadcast = 0xF,
}

/// Values from Zigbee spec Section 3.6.6 Broadcast Communication Table 3-76.
#[derive(Clone, Copy)]
pub enum BroadcastAddress {
    ShortAddressReservedF8 = 0xFFF8,
    ShortAddressReservedF9 = 0xFFF9,
    ShortAddressReservedFA = 0xFFFA,
    LowPowerRoutersOnly = 0xFFFB,
    AllRoutersAndCoordinator = 0xFFFC,
    MacRxOnWhenIdleIsTrue = 0xFFFD,
    ShortAddressReservedFE = 0xFFFE,
    AllDevicesInPan = 0xFFFF,
}