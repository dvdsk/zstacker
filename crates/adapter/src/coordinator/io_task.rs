use futures::future::FutureExt;
use futures_concurrency::future::Race;
use reader::FrameReader;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error};

use tokio::io::AsyncWriteExt;
use tokio_serial::SerialStream;
use zstacker_znp_protocol::framing::CommandMeta;

use super::PendingSend;

pub mod dispatch;
use dispatch::Dispatcher;
mod reader;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("IO error while writing request")]
    WritingIo(#[source] Arc<std::io::Error>),
    #[error("IO error while reading response data")]
    ReadingDataIo(#[source] Arc<std::io::Error>),
    #[error("Timed out reading incoming response (header already read)")]
    ReadingDataTimeout,
    #[error("IO Error while reading frame")]
    ReadingFrameIo(#[source] reader::Error),
    #[error("IO task panicked, panick info: {0:?}")]
    Panicked(String),
}

pub async fn io_task(
    mut serial: SerialStream,
    mut rx: mpsc::Receiver<PendingSend>,
) -> (SerialStream, Result<(), Error>) {
    let mut requests_expecting_reply = Dispatcher::default();

    enum Event {
        Received(Option<PendingSend>),
        ReadMeta(Result<(CommandMeta, Vec<u8>), reader::Error>),
    }

    let mut reader = FrameReader::default();
    loop {
        debug!("lop loop");
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
                debug!("Got new request to send");
                send_pending(
                    &mut serial,
                    pending,
                    &mut requests_expecting_reply,
                )
                .await
            }
            Event::ReadMeta(Ok((meta, data))) => {
                debug!("Read meta: {meta:?}");
                requests_expecting_reply.process_reply(&meta, data);
                Ok(())
            }
            Event::ReadMeta(Err(err)) => Err(Error::ReadingFrameIo(err)),
        };

        if let Err(err) = res {
            error!(
                "Io task ran into error, coordinator needs to be restarted to \
                recover. Error was: {err:?}"
            );
            return (serial, Err(err));
        }
    }
}

async fn send_pending(
    serial: &mut SerialStream,
    pending: PendingSend,
    requests_expecting_reply: &mut Dispatcher,
) -> Result<(), Error> {
    let to_send = pending.to_send.clone();
    requests_expecting_reply.register(pending).expect(
        "Having multiple requests with the same command \
            pending is not supported",
    );
    serial
        .write_all(&to_send)
        .await
        .map_err(Arc::new)
        .map_err(Error::WritingIo)?;
    debug!("send request");
    Ok(())
}
