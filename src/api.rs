use std::io::{Read, Write};

use tracing::info;

use crate::commands::{self, SyncRequest, CommandError, ReplyError};

pub fn start_coordinator(
    adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    begin_startup(adaptor)?;
    register_endpoints(adaptor)?;
    add_to_green_power_group(adaptor)?;
    Ok(())
}

pub fn check_connection_to_adapter(
    adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    let res = adaptor
        .send_command(commands::sys::Ping)
        .map_err(StartUpError::GetPing)?;
    info!(
        "Connected to adapter with capabilities: {:?}",
        res.capabilities
    );
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum StartUpError {
    #[error("Could not get adaptor info")]
    GetDeviceInfo(#[source] SendCommandError),
    #[error("Could not connect to adaptor")]
    GetPing(#[source] SendCommandError),
    #[error("Could not get adaptor version")]
    GetVersion(#[source] SendCommandError),
}

fn begin_startup(
    adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    let device_info = adaptor
        .send_command(crate::commands::util::GetDeviceInfo)
        .map_err(StartUpError::GetDeviceInfo)?;
    tracing::debug!("device_info: {device_info:?}");
    let version = adaptor
        .send_command(crate::commands::sys::Version)
        .map_err(StartUpError::GetVersion)?;
    tracing::debug!("device_version: {version:?}");
    Ok(())
}

fn add_to_green_power_group(
    _adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    todo!()
}

fn register_endpoints(
    _adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    todo!()
}

#[derive(Debug)]
pub struct Adaptor<S> {
    pub serial: S,
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

impl<S: Serial> Adaptor<S> {
    fn send_command<C: SyncRequest>(
        &mut self,
        cmd: C,
    ) -> Result<C::Reply, SendCommandError> {
        use crate::commands::SyncReply;
        let frame = cmd.to_frame().map_err(SendCommandError::Serializing)?;
        self.serial
            .write(&frame)
            .map_err(SendCommandError::Writing)?;
        C::Reply::from_reader(&mut self.serial)
            .map_err(SendCommandError::Reading)
    }
}

pub trait Serial: Send + Read + Write {}
