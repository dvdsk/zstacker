use std::collections::HashMap;
use std::collections::HashSet;

use tracing::debug;
use zstacker_znp_protocol::commands::zdo::{NeighborLqi, RoutingEntry};
use zstacker_znp_protocol::commands::{self, PartialList, ShortAddr};

use crate::coordinator::{Coordinator, QueueError};

#[derive(Debug, thiserror::Error)]
#[error("Could not request routing table from: {device}")]
pub struct ListAddressesError {
    device: ShortAddr,
    #[source]
    cause: QueueError,
}

#[derive(Debug, thiserror::Error)]
pub enum LqiTableError {
    #[error("Could not list nodes on network")]
    ListNodesOnNetwork(#[source] ListAddressesError),
    #[error("Could not request link quality from: {device}")]
    RequestLinkQuality {
        device: ShortAddr,
        #[source]
        cause: QueueError,
    },
}

impl Coordinator {
    async fn routing_table_for(
        &mut self,
        addr: ShortAddr,
    ) -> Result<Vec<RoutingEntry>, QueueError> {
        get_entire_list(Box::new(async |start_index| {
            self.queue_async(commands::zdo::MgmtRtgReq {
                dst_addr: addr,
                start_index,
            })
            .await
            .map(|rsp| rsp.routing_table)
        }))
        .await
    }

    async fn lqi_table_for(
        &mut self,
        addr: ShortAddr,
    ) -> Result<Vec<NeighborLqi>, QueueError> {
        get_entire_list(Box::new(async |start_index| {
            self.queue_async(commands::zdo::MgmtLqiReq {
                dst_addr: addr,
                start_index,
            })
            .await
            .map(|rsp| rsp.neighbor_lqis)
        }))
        .await
    }

    pub async fn list_addresses_on_network(
        &mut self,
    ) -> Result<HashSet<ShortAddr>, ListAddressesError> {
        let coordinator_table = self
            .routing_table_for(self.short_addr)
            .await
            .map_err(|cause| ListAddressesError {
            device: self.short_addr,
            cause,
        })?;

        let mut to_ask: HashSet<_> = coordinator_table
            .iter()
            .map(|entry| entry.destination_address)
            .collect();

        let mut res = HashSet::new();
        while let Some(addr) = to_ask.difference(&res).next().copied() {
            to_ask.remove(&addr);
            res.insert(addr);
            let table =
                self.routing_table_for(addr).await.map_err(|cause| {
                    ListAddressesError {
                        device: self.short_addr,
                        cause,
                    }
                })?;

            to_ask.extend(table.iter().map(|entry| entry.destination_address));
        }

        Ok(to_ask)
    }

    pub async fn lqi_table(
        &mut self,
    ) -> Result<HashMap<ShortAddr, Vec<u8>>, LqiTableError> {
        let to_ask = self
            .list_addresses_on_network()
            .await
            .map_err(LqiTableError::ListNodesOnNetwork)?;

        let mut res = HashMap::new();
        for addr in to_ask {
            debug!("asking: {addr:?} for their link quality");
            let table = self.lqi_table_for(addr).await.map_err(|cause| {
                LqiTableError::RequestLinkQuality {
                    device: addr,
                    cause,
                }
            })?;
            res.insert(
                addr,
                table.into_iter().map(|entry| entry.lqi).collect(),
            );
        }

        Ok(res)
    }
}

async fn get_entire_list<T, E>(
    mut partial_list_getter: impl AsyncFnMut(u8) -> Result<PartialList<T>, E>,
) -> Result<Vec<T>, E> {
    let mut next_start = 0;
    let mut list = Vec::new();
    loop {
        let last_partial = partial_list_getter(next_start).await?;
        next_start = last_partial.next_start();
        list.extend(last_partial.list);

        if list.len() >= last_partial.total_entries as usize {
            break;
        }
    }
    Ok(list)
}
