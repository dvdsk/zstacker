use zstacker_znp_protocol::commands::{CommandError, DeviceState, ReplyError};

#[derive(Debug, thiserror::Error)]
pub enum StartUpError {
    #[error("Could not get adaptor info")]
    GetDeviceInfo(#[source] SendCommandError),
    #[error("Could not connect to adaptor")]
    GetPing(#[source] SendCommandError),
    #[error("Could not get adaptor version")]
    GetVersion(#[source] SendCommandError),
    #[error(
        "Device is not running as coordinator. Instead its state is: {0:?}. \
        Run zigbee2mqtt with this adaptor to let it configure it"
    )]
    NotRunningAsCoordinator(DeviceState),
    #[error("Could not register endpoint")]
    RegisterEndpoints(#[source] RegisterEndpointsError),
}

#[derive(Debug, thiserror::Error)]
pub enum AdaptorError {
    #[error("Could not send command and receive output")]
    SendCommand(#[source] SendCommandError),
}

#[derive(Debug, thiserror::Error)]
pub enum SendCommandError {
    #[error("Could not write encoded command to serial")]
    Writing(#[source] std::io::Error),
    #[error("Could not read reply from serial")]
    Reading(#[source] ReplyError),
    #[error("Could not serialize command")]
    Serializing(#[source] CommandError),
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterEndpointsError{
    #[error("Error sending command or receiving reply")]
    Io(SendCommandError),
    #[error("Device returned status failed")]
    Failed,
}
