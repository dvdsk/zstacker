use futures::future::FutureExt;
use futures_concurrency::future::Race;
use reader::CommandMetaReader;
use tokio::sync::mpsc;
use tracing::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;
use tokio_util::time::FutureExt as _;
use zstacker_znp_protocol::framing::CommandMeta;

use super::{IoAwnserer, PendingSend};

mod matcher;
mod reader;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("IO error while writing request")]
    WritingIo(#[source] Arc<std::io::Error>),
    #[error("IO error while reading response data")]
    ReadingDataIo(#[source] Arc<std::io::Error>),
    #[error("Timed out reading incoming response (header already read)")]
    ReadingDataTimeout,
    #[error("IO Error while reading metadata")]
    ReadingMetaIo(#[source] reader::Error),
    #[error("IO task panicked, panick info: {0:?}")]
    Panicked(String),
}

pub async fn io_task(
    mut serial: SerialStream,
    mut rx: mpsc::Receiver<PendingSend>,
) -> (SerialStream, Result<(), Error>) {
    let mut requests_expecting_reply: HashMap<_, IoAwnserer> = HashMap::new();
    // Future work: track usage per route too. Prevent overloading route

    enum Event {
        Received(Option<PendingSend>),
        ReadMeta(Result<(u8, CommandMeta), reader::Error>),
    }

    let mut read_buf = vec![0u8; u8::MAX as usize];
    let mut reader = CommandMetaReader::default();
    loop {
        let res = match (
            rx.recv().map(Event::Received),
            reader.read(&mut serial).map(Event::ReadMeta),
        )
            .race()
            .await
        {
            Event::Received(None) => {
                debug!("Coordinator dropped, ending IO task");
                return (serial, Ok(()));
            }
            Event::Received(Some(pending)) => {
                send_pending(
                    &mut serial,
                    pending,
                    &mut requests_expecting_reply,
                )
                .await
            }
            Event::ReadMeta(Ok((length, meta))) => {
                handle_data(
                    &mut serial,
                    &mut requests_expecting_reply,
                    &mut read_buf[..length as usize],
                    meta,
                )
                .await
            }
            Event::ReadMeta(Err(err)) => Err(Error::ReadingMetaIo(err)),
        };

        if let Err(err) = res {
            error!(
                "Io task ran into error, coordinator needs to be restarted to \
                recover. Erro was: {err}"
            );
            return (serial, Err(err));
        }
    }
}

async fn send_pending(
    serial: &mut SerialStream,
    pending: PendingSend,
    requests_expecting_reply: &mut HashMap<CommandMeta, IoAwnserer>,
) -> Result<(), Error> {
    serial
        .write_all(&pending.to_send)
        .await
        .map_err(Arc::new)
        .map_err(Error::WritingIo)?;
    if pending.status_reply {
        todo!()
    }
    requests_expecting_reply
        .insert(pending.reply_meta, pending.awnser_to)
        .expect(
            "Having multiple requests with the same command \
            pending is not supported",
        );
    Ok(())
}

async fn handle_data(
    serial: &mut SerialStream,
    requests_expecting_reply: &mut HashMap<CommandMeta, IoAwnserer>,
    buf: &mut [u8],
    meta: CommandMeta,
) -> Result<(), Error> {
    let read_res = serial
        .read_exact(buf)
        .timeout(Duration::from_millis(10))
        .await;

    match read_res {
        Ok(Ok(_)) => {
            if let Some(answerer) = requests_expecting_reply.remove(&meta) {
                let _ = answerer.send(buf.to_vec());
            }
            Ok(())
        }
        Ok(Err(io_error)) => Err(Error::ReadingDataIo(Arc::new(io_error))),
        Err(_timeout_error) => Err(Error::ReadingDataTimeout),
    }
}
