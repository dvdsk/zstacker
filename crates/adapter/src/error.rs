use zstacker_znp_protocol::commands::DeviceState;

use crate::coordinator::QueueError;

#[derive(Debug, thiserror::Error)]
pub enum StartUpError {
    #[error("Could not get adaptor info")]
    GetDeviceInfo(#[source] QueueError),
    #[error("Could not connect to adaptor")]
    GetPing(#[source] QueueError),
    #[error("Could not get adaptor version")]
    GetVersion(#[source] QueueError),
    #[error(
        "Device is not running as coordinator. Instead its state is: {0:?}. \
        Run zigbee2mqtt with this adaptor to let it configure it"
    )]
    NotRunningAsCoordinator(DeviceState),
    #[error("Could not register endpoint")]
    RegisterEndpoints(#[source] RegisterEndpointsError),
    #[error("Could not request device to start in the network")]
    RequestStartup(#[source] QueueError),
    #[error("Device did not recover existing network")]
    NetworkRecoverFailed,
    #[error("Could not add device to green power group")]
    AddingToGreenPowerGroup(#[source] QueueError),
    #[error("Could not reset the adaptor")]
    ResetFailed(#[source] QueueError),
    #[error("Could not configure the device to use maximum tx power")]
    SetMaxTxPower(#[source] QueueError),
    #[error("Device returned an error when configuring max tx power")]
    SetMaxTxPowerFailure,
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterEndpointsError {
    #[error("Error sending command or receiving reply")]
    Io(QueueError),
    #[error("Device returned status failed")]
    Failed,
}
