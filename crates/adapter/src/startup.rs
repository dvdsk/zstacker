use std::thread;
use std::time::Duration;

use tokio::time;
use tracing::{debug, info};

use zstacker_znp_protocol::commands::sys::ResetType;
use zstacker_znp_protocol::commands::util::DeviceInfo;
use zstacker_znp_protocol::commands::{self, DeviceState};

use crate::coordinator::{Adaptor, Coordinator};
use crate::error::{RegisterEndpointsError, StartUpError};

type Endpoint = commands::af::Register;

pub async fn start_coordinator(
    mut adaptor: Adaptor,
    endpoints: Vec<Endpoint>,
) -> Result<Coordinator, StartUpError> {
    reset_device(&mut adaptor).await?;
    debug!("device reset");
    let device_info = start_as_coordinator_if_needed(&mut adaptor).await?;
    debug!("device started as coordinator");
    register_endpoints(&mut adaptor, endpoints)
        .await
        .map_err(StartUpError::RegisterEndpoints)?;
    debug!("needed endpoints registered on device");
    add_to_green_power_group(&mut adaptor).await?;
    debug!("added device to green power group");
    Ok(Coordinator::start(device_info, adaptor))
}

pub async fn reset_device(coord: &mut Adaptor) -> Result<(), StartUpError> {
    let reset_cmd = commands::sys::ResetReq {
        ty: ResetType::Soft,
    };

    // Reset device multiple times to make sure one arrives
    coord
        .queue_async(reset_cmd)
        .await
        .map_err(StartUpError::ResetFailed)?;
    time::sleep(Duration::from_millis(100)).await;
    coord
        .queue_async(reset_cmd)
        .await
        .map_err(StartUpError::ResetFailed)?;
    time::sleep(Duration::from_millis(100)).await;
    coord
        .queue_async(reset_cmd)
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
        .queue_sync(commands::sys::Ping)
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
            .queue_sync(commands::util::GetDeviceInfo)
            .await
            .map_err(StartUpError::GetDeviceInfo)?;
        tracing::debug!("device_info: {device_info:?}");
        match device_info.device_state {
            DeviceState::StartedAsZBCoordinator => {
                let version = adaptor
                    .queue_sync(commands::sys::Version)
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
                    .queue_sync(commands::zdo::StartupFromApp { startdelay: 0 })
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
        .queue_sync(commands::zdo::ExtFindGroup {
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
    // Note, `z2m` checks if the endpoint is already registered first
    // no idea why, let's see
    for register_cmd in endpoints {
        let reply = adaptor
            .queue_sync(register_cmd)
            .await
            .map_err(RegisterEndpointsError::Io)?;
        if reply.is_err() {
            return Err(RegisterEndpointsError::Failed);
        }
    }
    Ok(())
}
