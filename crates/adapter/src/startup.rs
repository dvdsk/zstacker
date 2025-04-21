use std::thread;
use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::time;
use tokio_serial::SerialStream;
use tracing::info;

use zstacker_znp_protocol::commands::sys::ResetType;
use zstacker_znp_protocol::commands::util::DeviceInfo;
use zstacker_znp_protocol::commands::{
    self, AsyncRequest, DeviceState, SyncRequest,
};

use crate::coordinator::Coordinator;
use crate::error::{RegisterEndpointsError, SendCommandError, StartUpError};

type Endpoint = commands::af::Register;

pub async fn start_coordinator(
    mut adaptor: Adaptor,
    endpoints: Vec<Endpoint>,
) -> Result<Coordinator, StartUpError> {
    let device_info = start_as_coordinator_if_needed(&mut adaptor).await?;
    register_endpoints(&mut adaptor, endpoints)
        .await
        .map_err(StartUpError::RegisterEndpoints)?;
    add_to_green_power_group(&mut adaptor).await?;

    Ok(Coordinator::start(device_info, adaptor))
}

pub async fn reset_device(adaptor: &mut Adaptor) -> Result<(), StartUpError> {
    let reset_frame = commands::sys::ResetReq {
        ty: ResetType::Soft,
    }
    .to_frame()
    .expect("reset cmd should serialize");

    // Reset device multiple times to make sure one arrives
    adaptor
        .serial
        .write_all(&reset_frame)
        .await
        .map_err(StartUpError::ResetFailed)?;
    time::sleep(Duration::from_millis(100)).await;
    adaptor
        .serial
        .write_all(&reset_frame)
        .await
        .map_err(StartUpError::ResetFailed)?;
    time::sleep(Duration::from_millis(100)).await;
    adaptor
        .serial
        .write_all(&reset_frame)
        .await
        .map_err(StartUpError::ResetFailed)?;

    // Give the device time to reset
    time::sleep(Duration::from_secs(1)).await;
    Ok(())
}

pub async fn check_connection_to_adapter(
    adaptor: &mut Adaptor,
) -> Result<(), StartUpError> {
    let res = adaptor
        .send_command(commands::sys::Ping)
        .await
        .map_err(StartUpError::GetPing)?;
    info!(
        "Connected to adapter with capabilities: {:?}",
        res.capabilities
    );
    Ok(())
}

async fn start_as_coordinator_if_needed(
    adaptor: &mut Adaptor,
) -> Result<DeviceInfo, StartUpError> {
    use commands::zdo::StartupFromAppReply;
    loop {
        let device_info = adaptor
            .send_command(commands::util::GetDeviceInfo)
            .await
            .map_err(StartUpError::GetDeviceInfo)?;
        tracing::debug!("device_info: {device_info:?}");
        match device_info.device_state {
            DeviceState::StartedAsZBCoordinator => {
                let version = adaptor
                    .send_command(commands::sys::Version)
                    .await
                    .map_err(StartUpError::GetVersion)?;
                tracing::debug!("device_version: {version:?}");
                return Ok(device_info);
            }
            DeviceState::StartingAsZBCoordinator => {
                thread::sleep(Duration::from_millis(50))
            }
            _ => {
                let rsp = adaptor
                    .send_command(commands::zdo::StartupFromApp {
                        startdelay: 0,
                    })
                    .await
                    .map_err(StartUpError::RequestStartup)?;
                match rsp {
                    StartupFromAppReply::RestoredNetworkState => (),
                    _ => {
                        return Err(StartUpError::NetworkRecoverFailed);
                    }
                }
            }
        }
    }
}

async fn add_to_green_power_group(
    adaptor: &mut Adaptor,
) -> Result<(), StartUpError> {
    let _ = adaptor
        .send_command(commands::zdo::ExtFindGroup {
            endpoint: 242,
            groupid: 2948,
        })
        .await
        .map_err(StartUpError::AddingToGreenPowerGroup)?;

    Ok(())
}

async fn register_endpoints(
    adaptor: &mut Adaptor,
    endpoints: Vec<Endpoint>,
) -> Result<(), RegisterEndpointsError> {
    // Note, z2m checks if the endpoint is already registered first
    // no idea why, let's see
    for register_cmd in endpoints {
        let reply = adaptor
            .send_command(register_cmd)
            .await
            .map_err(RegisterEndpointsError::Io)?;
        if reply.is_err() {
            return Err(RegisterEndpointsError::Failed);
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Adaptor {
    pub serial: SerialStream,
}

impl Adaptor {
    pub async fn send_command<C: SyncRequest>(
        &mut self,
        cmd: C,
    ) -> Result<C::Reply, SendCommandError> {
        use commands::SyncReply;
        let frame = cmd.to_frame().map_err(SendCommandError::Serializing)?;
        self.serial
            .write_all(&frame)
            .await
            .map_err(SendCommandError::Writing)?;
        C::Reply::from_reader(&mut self.serial)
            .map_err(SendCommandError::Reading)
    }
}
