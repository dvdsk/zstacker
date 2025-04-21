use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio_serial::SerialStream;
use zstacker_znp_protocol::commands::START_OF_FRAME;
use zstacker_znp_protocol::framing::{CommandMeta, CommandMetaError};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error(
        "Frame should start with start of frame token ({})",
        START_OF_FRAME
    )]
    ExpectedStartOfFrame,
    #[error("Could not read from serial")]
    Io(#[source] Arc<std::io::Error>),
    #[error("Could not read from serial")]
    Deserialize(#[source] CommandMetaError),
}

#[derive(Debug, Default)]
pub struct CommandMetaReader {
    buffer: [u8; 4],
    read: usize,
}

type Length = u8;
impl CommandMetaReader {
    pub async fn read(
        &mut self,
        serial: &mut SerialStream,
    ) -> Result<(Length, CommandMeta), Error> {
        for byte in self.buffer.iter_mut().skip(self.read) {
            *byte = serial
                .read_u8()
                .await
                .map_err(Arc::new)
                .map_err(Error::Io)?;
            self.read += 1
        }

        self.read = 0;
        let [START_OF_FRAME, length, meta_bytes @ ..] = self.buffer else {
            return Err(Error::ExpectedStartOfFrame);
        };

        let meta =
            CommandMeta::deserialize(meta_bytes).map_err(Error::Deserialize)?;
        Ok((length, meta))
    }
}
