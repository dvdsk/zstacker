use std::io::{Read, Write};

use tracing::info;

use crate::commands::{self, Command};
use crate::data_format;

#[derive(Debug, thiserror::Error)]
pub enum StartUpError {
    #[error("Could not get device info")]
    GetDeviceInfo(#[source] SendCommandError),
    #[error("Could not get device info")]
    GetPing(#[source] SendCommandError),
}

pub fn start(adaptor: &mut Adaptor<impl Serial>) -> Result<(), StartUpError> {
    check_connection_to_adapter(adaptor)?;
    begin_startup(adaptor)?;
    register_endpoints(adaptor)?;
    add_to_green_power_group(adaptor)?;
    Ok(())
}

fn check_connection_to_adapter(
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

fn begin_startup(
    adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    let device_info = adaptor
        .send_command(crate::commands::util::GetDeviceInfo)
        .map_err(StartUpError::GetDeviceInfo)?;
    println!("device_info: {device_info:?}");
    Ok(())
}

pub struct Adaptor<S> {
    serial: S,
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
    #[error("Could not read reply to command from serial")]
    Reading(#[source] std::io::Error),
    #[error("Could not serialize command")]
    Serializing(#[source] data_format::Error),
    #[error("Could not deserialize reply")]
    Deserializing(#[source] data_format::Error),
}

impl<S: Serial> Adaptor<S> {
    fn send_command<C: Command>(
        &mut self,
        cmd: C,
    ) -> Result<C::Reply, SendCommandError> {
        let frame = cmd.to_frame().map_err(SendCommandError::Serializing)?;
        self.serial
            .write(&frame)
            .map_err(SendCommandError::Writing)?;
        data_format::from_reader(&mut self.serial)
            .map_err(SendCommandError::Deserializing)
    }
}

pub trait Serial: Send + Read + Write {}
