use std::thread;
use std::time::Duration;

use tracing::{debug, info, instrument};

use zstacker_znp_protocol::commands::sys::{ResetInd, ResetType};
use zstacker_znp_protocol::commands::util::DeviceInfo;
use zstacker_znp_protocol::commands::{self, DeviceState};

use crate::coordinator::{Adaptor, Coordinator};
use crate::error::{RegisterEndpointsError, StartUpError};

type Endpoint = commands::af::Register;

#[instrument(skip(adaptor))]
pub async fn start_coordinator(
    mut adaptor: Adaptor,
    endpoints: Vec<Endpoint>,
) -> Result<Coordinator, StartUpError> {
    reset_device(&mut adaptor).await?;
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

#[instrument(skip(adaptor))]
pub async fn reset_device(adaptor: &mut Adaptor) -> Result<(), StartUpError> {
    let ResetInd {
        product_id,
        major_rel,
        minor_rel,
        ..
    } = adaptor
        .queue_async(commands::sys::ResetReq {
            ty: ResetType::Soft,
        })
        .await
        .map_err(StartUpError::ResetFailed)?;

    info!("device id: {product_id}, version: {major_rel}.{minor_rel}");

    Ok(())
}

#[instrument(skip(adaptor))]
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

#[instrument(skip(adaptor))]
async fn start_as_coordinator_if_needed(
    adaptor: &mut Adaptor,
) -> Result<DeviceInfo, StartUpError> {
    use commands::zdo::StartupFromAppReply;
    loop {
        let device_info = adaptor
            .queue_sync(commands::util::GetDeviceInfo)
            .await
            .map_err(StartUpError::GetDeviceInfo)?;
        match device_info.device_state {
            DeviceState::StartedAsZBCoordinator => {
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

#[instrument(skip(adaptor))]
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

#[instrument(skip(adaptor))]
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
