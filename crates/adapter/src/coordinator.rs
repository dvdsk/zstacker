use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tokio_serial::SerialStream;
use tokio_util::time::FutureExt as _;
use tracing::error;
use zstacker_znp_protocol::commands::util::DeviceInfo;
use zstacker_znp_protocol::commands::{AsyncReply, ReplyError, SyncReply};
use zstacker_znp_protocol::commands::{
    AsyncRequest, CommandError, IeeeAddr, ShortAddr, SyncRequest,
};
use zstacker_znp_protocol::framing::CommandMeta;

use crate::startup::Adaptor;

type Data = Vec<u8>;
type IoAwnserer = oneshot::Sender<Data>;

mod io_task;

struct PendingSend {
    awnser_to: IoAwnserer,
    to_send: Vec<u8>,
    reply_meta: CommandMeta,
    status_reply: bool,
}

#[derive(Debug)]
pub struct Coordinator {
    pub short_addr: ShortAddr,
    pub ieee_addr: IeeeAddr,

    to_io_task: mpsc::Sender<PendingSend>,
    io_task:
        Option<task::JoinHandle<(SerialStream, Result<(), io_task::Error>)>>,

    // These are only some after a critical error forces the IO task to stop
    io_task_error: Option<io_task::Error>,
    recovered_serial: Option<SerialStream>,
}

impl Coordinator {
    pub(crate) fn start(device_info: DeviceInfo, adaptor: Adaptor) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            short_addr: device_info.short_addr,
            ieee_addr: device_info.ieee_addr,
            to_io_task: tx,
            io_task: Some(task::spawn(io_task::io_task(adaptor.serial, rx))),
            io_task_error: None,
            recovered_serial: None,
        }
    }

    /// May wait until there is space in the receive buffer
    pub async fn queue_sync<R: SyncRequest>(
        &mut self,
        req: R,
    ) -> Result<R::Reply, QueueError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.to_io_task
            .send(PendingSend {
                awnser_to: tx,
                to_send: req.to_frame().map_err(QueueError::Serializing)?,
                reply_meta: R::Reply::META,
                status_reply: false,
            })
            .await
            .expect("Io-task is dropped after queue_requests");
        match rx.timeout(Duration::from_millis(50)).await {
            Err(_) => Err(QueueError::ReplyNotImmediate),
            Ok(Err(_)) => Err(self.io_task_error().await),
            Ok(Ok(data)) => {
                R::Reply::from_data(&data).map_err(QueueError::Derializing)
            }
        }
    }

    /// May wait until there is space in the receive buffer
    pub async fn queue_async<R: AsyncRequest>(
        &mut self,
        req: R,
    ) -> Result<R::Reply, QueueError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.to_io_task
            .send(PendingSend {
                awnser_to: tx,
                to_send: req.to_frame().map_err(QueueError::Serializing)?,
                reply_meta: R::Reply::META,
                status_reply: R::HAS_SYNC_STATUS_RPLY,
            })
            .await
            .expect("io-task is dropped after queue_requests");
        match rx.timeout(R::TIMEOUT).await {
            Err(_) => Err(QueueError::TimedOut {
                timeout: R::TIMEOUT,
            }),
            Ok(Err(_)) => Err(self.io_task_error().await),
            Ok(Ok(data)) => {
                R::Reply::from_data(&data).map_err(QueueError::Derializing)
            }
        }
    }

    async fn io_task_error(&mut self) -> QueueError {
        if let Some(err) = self.io_task_error.clone() {
            return QueueError::IoTask(err);
        }

        let Some(io_task) = self.io_task.take() else {
            unreachable!(
                "after we take the io_task we always set the io_task_error"
            );
        };

        assert!(
            io_task.is_finished(),
            "a requests awnser rx is only dropped when the io_task returns"
        );

        match io_task.await {
            Ok((serial, Err(err))) => {
                self.io_task_error = Some(err.clone());
                self.recovered_serial = Some(serial);
                QueueError::IoTask(err)
            }
            Ok((_, Ok(()))) => {
                unreachable!(
                    "Io task only ends without error if Self is dropped"
                )
            }
            Err(join_error) => {
                let err = io_task::Error::Panicked(join_error.to_string());
                self.io_task_error = Some(err.clone());
                QueueError::IoTask(err)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("Error while serializing the request")]
    Serializing(#[source] CommandError),
    #[error("Error deseralizing the request")]
    Derializing(#[source] ReplyError),
    #[error("IoTask ran into an error and has ended")]
    IoTask(#[source] io_task::Error),
    #[error("Async request did not get awnserd within: {timeout:?}")]
    TimedOut { timeout: Duration },
    #[error("Sync request was not awnserd immediatly")]
    ReplyNotImmediate,
}

#[derive(Debug, thiserror::Error)]
pub enum IoHandleError {
    #[error("Io error while reading data")]
    IoErrReadingData(#[source] std::io::Error),
}
