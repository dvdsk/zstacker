#[derive(Clone, Copy)]
pub enum CommissioningMode {
    Initialization = 0x00,
    TouchLink = 0x01,
    NetworkSteering = 0x02,
    NetworkFormation = 0x04,
    FindingAndBinding = 0x08,
}

#[derive(Clone, Copy)]
pub enum DeviceState {
    InitializatedNotStartedAutomatically = 0x00,
    InitializatedNotConnected = 0x01,
    DiscoveringPansToJoin = 0x02,
    JoiningAPan = 0x03,
    ReJoiningAPan = 0x04,
    JoinedButNotYetAuthenticatedByTS = 0x05,
    StartedAsDeviceAfterAuthentication = 0x06,
    DeviceJoinedAuthenticatedAndIsRouter = 0x07,
    StartingAsZBCoordinator = 0x08,
    StartedAsZBCoordinator = 0x09,
    DeviceLostInfoAboutParent = 0x0A,
}

#[derive(Clone, Copy)]
pub enum CommissioningNotificationStatus {
    BDB_COMMISSIONING_SUCCESS = 0x00,
    BDB_COMMISSIONING_IN_PROGRESS = 0x01,
    BDB_COMMISSIONING_NO_NETWORK = 0x02,
    BDB_COMMISSIONING_TL_TARGET_FAILURE = 0x03,
    BDB_COMMISSIONING_TL_NOT_AA_CAPABLE = 0x04,
    BDB_COMMISSIONING_TL_NO_SCAN_RESPONSE = 0x05,
    BDB_COMMISSIONING_TL_NOT_PERMITTED = 0x06,
    BDB_COMMISSIONING_TCLK_EX_FAILURE = 0x07,
    BDB_COMMISSIONING_FORMATION_FAILURE = 0x08,
    BDB_COMMISSIONING_FB_TARGET_IN_PROGRESS = 0x09,
    BDB_COMMISSIONING_FB_INITIATOR_IN_PROGRESS = 0x0A,
    BDB_COMMISSIONING_FB_NO_IDENTIFY_QUERY_RESPONSE = 0x0B,
    BDB_COMMISSIONING_FB_BINDING_TABLE_FULL = 0x0C,
    BDB_COMMISSIONING_NETWORK_RESTORED = 0x0D,
    BDB_COMMISSIONING_FAILURE = 0x0E,
}

#[derive(Clone, Copy)]
pub enum CommissioningNotificationMode {
    BDB_COMMISSIONING_INITIALIZATION = 0x00,
    BDB_COMMISSIONING_NWK_STEERING = 0x01,
    BDB_COMMISSIONING_FORMATION = 0x02,
    BDB_COMMISSIONING_FINDING_BINDING = 0x03,
    BDB_COMMISSIONING_TOUCHLINK = 0x04,
    BDB_COMMISSIONING_PARENT_LOST = 0x05,
}

#[derive(Clone, Copy)]
pub enum CommissioningNotificationRemainingModes {
    BDB_COMMISSIONING_MODE_INITIATOR_TL = 0x01,
    BDB_COMMISSIONING_MODE_NWK_STEERING = 0x02,
    BDB_COMMISSIONING_MODE_NWK_FORMATION = 0x04,
    BDB_COMMISSIONING_MODE_FINDING_BINDING = 0x08,
    BDB_COMMISSIONING_MODE_INITIALIZATION = 0x10,
    BDB_COMMISSIONING_MODE_PARENT_LOST = 0x20,
}
