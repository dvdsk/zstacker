//! Pretty inefficient but it is cancel safe

use std::sync::Arc;
use std::time::Duration;

use tokio::io::AsyncReadExt;
use tokio_serial::SerialStream;
use tokio_util::time::FutureExt;
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
    #[error("Timed out reading next serial byte")]
    Timeout,
}

#[derive(Debug)]
pub enum FrameReader {
    ReadingMeta(MetaReader),
    ReadingData(DataReader),
}

impl Default for FrameReader {
    fn default() -> Self {
        Self::ReadingMeta(MetaReader::default())
    }
}

#[derive(Debug, Default)]
pub struct MetaReader {
    buffer: [u8; 4],
    n_read: usize,
}

#[derive(Debug)]
pub struct DataReader {
    meta: CommandMeta,
    data_length: usize,
    bytes_read: Vec<u8>,
}

type Data = Vec<u8>;
impl FrameReader {
    /// # Cancel safety
    ///
    /// This is cancel safe
    pub async fn read(
        &mut self,
        serial: &mut SerialStream,
    ) -> Result<(CommandMeta, Data), Error> {
        let data_reader = match self {
            Self::ReadingMeta(meta_reader) => {
                let data_reader = meta_reader.read(serial).await?;
                *self = Self::ReadingData(data_reader);
                let Self::ReadingData(reader) = self else {
                    unreachable!()
                };
                reader
            }
            Self::ReadingData(data_reader) => data_reader,
        };

        let (meta, data) = data_reader.read(serial).await?;
        *self = Self::ReadingMeta(MetaReader::default());
        Ok((meta, data))
    }
}

impl MetaReader {
    pub async fn read(
        &mut self,
        serial: &mut SerialStream,
    ) -> Result<DataReader, Error> {
        for byte in self.buffer.iter_mut().skip(self.n_read) {
            *byte = serial
                .read_u8()
                .timeout(Duration::from_millis(50))
                .await
                .map_err(|_| Error::Timeout)?
                .map_err(Arc::new)
                .map_err(Error::Io)?;
            self.n_read += 1
        }

        self.n_read = 0;
        let [START_OF_FRAME, length, meta_bytes @ ..] = self.buffer else {
            return Err(Error::ExpectedStartOfFrame);
        };

        let meta =
            CommandMeta::deserialize(meta_bytes).map_err(Error::Deserialize)?;
        Ok(DataReader {
            meta,
            data_length: length as usize,
            bytes_read: Vec::new(),
        })
    }
}

impl DataReader {
    pub async fn read(
        &mut self,
        serial: &mut SerialStream,
    ) -> Result<(CommandMeta, Data), Error> {
        let left_to_read = self.data_length - self.bytes_read.len();
        for _ in 0..left_to_read {
            self.bytes_read.push(
                serial
                    .read_u8()
                    .timeout(Duration::from_millis(50))
                    .await
                    .map_err(|_| Error::Timeout)?
                    .map_err(Arc::new)
                    .map_err(Error::Io)?,
            );
        }

        Ok((self.meta.clone(), self.bytes_read.clone()))
    }
}
