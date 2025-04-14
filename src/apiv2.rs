use std::io::{Read, Write};

use crate::commands::Command;
use crate::data_format;

#[derive(Debug, thiserror::Error)]
pub enum StartUpError {
    #[error("Could not get device info")]
    GetDeviceInfo(#[source] AdaptorError),
}

pub fn start(adaptor: &mut Adaptor<impl Serial>) -> Result<(), StartUpError> {
    begin_startup(adaptor)?;
    register_endpoints(adaptor)?;
    add_to_green_power_group(adaptor)?;
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
        .get_device_info()
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

impl<S: Serial> Adaptor<S> {
    fn get_device_info(
        &mut self,
    ) -> Result<crate::commands::util::DeviceInfo, AdaptorError> {
        self.send_command(crate::commands::util::GetDeviceInfo)
            .map_err(AdaptorError::SendCommand)
    }
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
