use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use tracing::info;

use zstacker_znp_protocol::commands::{self, DeviceState, SyncRequest};

use crate::error::{RegisterEndpointsError, SendCommandError, StartUpError};

type Endpoint = commands::af::Register;

pub fn start_coordinator(
    adaptor: &mut Adaptor<impl Serial>,
    endpoints: Vec<Endpoint>,
) -> Result<(), StartUpError> {
    start_as_coordinator_if_needed(adaptor)?;
    register_endpoints(adaptor, endpoints)
        .map_err(StartUpError::RegisterEndpoints)?;
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

fn start_as_coordinator_if_needed(
    adaptor: &mut Adaptor<impl Serial>,
) -> Result<(), StartUpError> {
    loop {
        let device_info = adaptor
            .send_command(commands::util::GetDeviceInfo)
            .map_err(StartUpError::GetDeviceInfo)?;
        tracing::debug!("device_info: {device_info:?}");
        match device_info.device_state {
            DeviceState::StartedAsZBCoordinator => break,
            DeviceState::StartingAsZBCoordinator => {
                thread::sleep(Duration::from_millis(50))
            }
            other => {
                // adaptor.send_command(commands::zdo::Start)
                todo!()
            },
        }
    }
    let version = adaptor
        .send_command(commands::sys::Version)
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
    adaptor: &mut Adaptor<impl Serial>,
    endpoints: Vec<Endpoint>,
) -> Result<(), RegisterEndpointsError> {
    // Note, z2m checks if the endpoint is already registered first
    // no idea why, let's see
    for register_cmd in endpoints {
        let reply = adaptor
            .send_command(register_cmd)
            .map_err(RegisterEndpointsError::Io)?;
        if reply.is_err() {
            return Err(RegisterEndpointsError::Failed);
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Adaptor<S> {
    pub serial: S,
}

impl<S: Serial> Adaptor<S> {
    fn send_command<C: SyncRequest>(
        &mut self,
        cmd: C,
    ) -> Result<C::Reply, SendCommandError> {
        use commands::SyncReply;
        let frame = cmd.to_frame().map_err(SendCommandError::Serializing)?;
        self.serial
            .write(&frame)
            .map_err(SendCommandError::Writing)?;
        C::Reply::from_reader(&mut self.serial)
            .map_err(SendCommandError::Reading)
    }
}

pub trait Serial: Send + Read + Write {}
