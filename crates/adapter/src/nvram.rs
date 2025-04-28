use zstacker_znp_protocol::commands;
use zstacker_znp_protocol::commands::sys::{NvId, OsalNvLengthReply};

use crate::coordinator::{Coordinator, QueueError};

pub mod ids;
pub mod types;

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("Could not ask the coordinator for the length")]
    QueryingLength(#[source] QueueError),
    #[error("There is no item with this ID in nvram")]
    DoesNotExist(NvId),
    #[error("Could not read bytes from nvram")]
    ReadingItem(#[source] QueueError),
    #[error("Coordinator reported status Failure")]
    ReadFailed,
}

impl Coordinator {
    pub async fn read_nvram_item(
        &mut self,
        item_id: NvId,
    ) -> Result<Vec<u8>, ReadError> {
        let reply = self
            .queue_sync(commands::sys::OsalNvLength { item_id })
            .await
            .map_err(ReadError::QueryingLength)?;

        let length = match reply {
            OsalNvLengthReply::ItemExists { length } => length,
            OsalNvLengthReply::ItemDoesNotExist => {
                return Err(ReadError::DoesNotExist(item_id));
            }
        };

        let mut res = Vec::new();
        while res.len() < length.get().into() {
            let reply = self
                .queue_sync(commands::sys::OsalNvReadExt {
                    id: item_id,
                    offset: res.len() as u16,
                })
                .await
                .map_err(ReadError::ReadingItem)?;
            reply
                .status
                .as_result()
                .map_err(|()| ReadError::ReadFailed)?;
            res.extend(reply.bytes);
        }

        Ok(res)
    }
}
