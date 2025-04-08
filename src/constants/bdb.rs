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
    BdbCommissioningSuccess = 0x00,
    BdbCommissioningInProgress = 0x01,
    BdbCommissioningNoNetwork = 0x02,
    BdbCommissioningTlTargetFailure = 0x03,
    BdbCommissioningTlNotAaCapable = 0x04,
    BdbCommissioningTlNoScanResponse = 0x05,
    BdbCommissioningTlNotPermitted = 0x06,
    BdbCommissioningTclkExFailure = 0x07,
    BdbCommissioningFormationFailure = 0x08,
    BdbCommissioningFbTargetInProgress = 0x09,
    BdbCommissioningFbInitiatorInProgress = 0x0A,
    BdbCommissioningFbNoIdentifyQueryResponse = 0x0B,
    BdbCommissioningFbBindingTableFull = 0x0C,
    BdbCommissioningNetworkRestored = 0x0D,
    BdbCommissioningFailure = 0x0E,
}

#[derive(Clone, Copy)]
pub enum CommissioningNotificationMode {
    BdbCommissioningInitialization = 0x00,
    BdbCommissioningNwkSteering = 0x01,
    BdbCommissioningFormation = 0x02,
    BdbCommissioningFindingBinding = 0x03,
    BdbCommissioningTouchlink = 0x04,
    BdbCommissioningParentLost = 0x05,
}

#[derive(Clone, Copy)]
pub enum CommissioningNotificationRemainingModes {
    BdbCommissioningModeInitiatorTl = 0x01,
    BdbCommissioningModeNwkSteering = 0x02,
    BdbCommissioningModeNwkFormation = 0x04,
    BdbCommissioningModeFindingBinding = 0x08,
    BdbCommissioningModeInitialization = 0x10,
    BdbCommissioningModeParentLost = 0x20,
}
