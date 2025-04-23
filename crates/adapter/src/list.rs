use zstacker_znp_protocol::commands;

use crate::coordinator::{Coordinator, QueueError};

#[derive(Debug, thiserror::Error)]
pub enum ListError {
    #[error("Could not request link quality from device")]
    RequestLinkQuality(#[source] QueueError),
}

impl Coordinator {
    pub async fn list_addresses_on_network(&mut self) -> Result<(), ListError> {
        // Get router list for self than every node that has not send router list

        // Do for each node including self
        let rsp = self
            .queue_async(commands::zdo::MgmtLqiReq {
                dstaddr: self.short_addr,
                startindex: 0,
            })
            .await
            .map_err(ListError::RequestLinkQuality)?;
        Ok(())
    }
}
