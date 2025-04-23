//! Pretty inefficient but it is cancel safe

use std::iter;
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
    #[error("Timed out reading after start of frame was received")]
    Timeout,
    #[error("Send checksum does not match calculated one")]
    CheckSumMismatch,
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
    /// length, meta
    buffer: [u8; 3],
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
        if self.n_read == 0 {
            if serial
                .read_u8()
                .await
                .map_err(Arc::new)
                .map_err(Error::Io)?
                != START_OF_FRAME
            {
                return Err(Error::ExpectedStartOfFrame);
            };
        }

        for byte in self.buffer.iter_mut().skip(self.n_read) {
            *byte = serial
                .read_u8()
                .timeout(Duration::from_millis(50))
                .await
                .map_err(|_| Error::Timeout)?
                .map_err(Arc::new)
                .map_err(Error::Io)?;
            self.n_read += 1;
        }

        self.n_read = 0;
        let [data_length, meta_bytes @ ..] = self.buffer;

        let meta =
            CommandMeta::deserialize(meta_bytes).map_err(Error::Deserialize)?;
        Ok(DataReader {
            meta,
            data_length: data_length as usize,
            bytes_read: Vec::new(),
        })
    }
}

impl DataReader {
    pub async fn read(
        &mut self,
        serial: &mut SerialStream,
    ) -> Result<(CommandMeta, Data), Error> {
        const CHECKSUM_LENGTH: usize = 1;
        let left_to_read =
            self.data_length + CHECKSUM_LENGTH - self.bytes_read.len();
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
        let checksum_in_frame = self.bytes_read.pop().unwrap();

        let frame = iter::once(self.data_length as u8)
            .chain(self.meta.serialize())
            .chain(self.bytes_read.iter().copied());
        let calculated_checksum = frame
            .reduce(|checksum, byte| checksum ^ byte)
            .expect("never empty");
        if checksum_in_frame != calculated_checksum {
            return Err(Error::CheckSumMismatch);
        }

        Ok((self.meta.clone(), self.bytes_read.clone()))
    }
}
