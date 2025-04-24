use std::iter;
use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::data_format;
use crate::framing::CommandMeta;

use super::{
    CommandError, CommandType, Pattern, ReplyError, START_OF_FRAME, SubSystem,
};

pub trait SyncRequest: Serialize + std::fmt::Debug {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::SREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
    const TIMEOUT: Duration = Duration::from_millis(50);
    type Reply: SyncReply;

    fn reply_pattern(&self) -> Pattern {
        Pattern::default()
    }

    fn data_to_vec(&self) -> Result<Vec<u8>, data_format::Error>
    where
        Self: Sized,
    {
        data_format::to_vec(self)
    }

    fn to_frame(&self) -> Result<Vec<u8>, CommandError>
    where
        Self: Sized,
    {
        let serialized_data =
            self.data_to_vec().map_err(|error| CommandError {
                command: std::any::type_name::<Self>(),
                cause: error,
            })?;
        to_frame(serialized_data, Self::META)
    }
}

mod to_frame {
    use super::*;

    pub fn to_frame(
        serialized_data: Vec<u8>,
        meta: CommandMeta,
    ) -> Result<Vec<u8>, CommandError> {
        let frame_body = [serialized_data.len() as u8]
            .into_iter()
            .chain(meta.serialize())
            .chain(serialized_data);

        let checksum = frame_body
            .clone()
            .reduce(|checksum, byte| checksum ^ byte)
            .expect("never empty");

        Ok(iter::once(START_OF_FRAME)
            .chain(frame_body)
            .chain([checksum])
            .collect())
    }
}
#[cfg(feature = "mocking")]
pub use to_frame::to_frame;
#[cfg(not(feature = "mocking"))]
pub(crate) use to_frame::to_frame;

pub trait SyncReply: DeserializeOwned + std::fmt::Debug {
    type Request: SyncRequest;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::SRSP,
        sub_system: Self::Request::SUBSYSTEM,
        id: Self::Request::ID,
    };

    fn from_data(data: &[u8]) -> Result<Self, ReplyError> {
        use crate::commands::ReplyErrorCause as E;
        let mut data = std::io::Cursor::new(data);
        data_format::from_reader(&mut data)
            .map_err(E::Deserialize)
            .map_err(|cause| ReplyError {
                reply: std::any::type_name::<Self>(),
                cause,
            })
    }
}

/// These map to two separate mt command types:
/// - `Self::HAS_SYNC_STATUS_RPLY` is false: an AREQ send by the host
/// - `Self::HAS_SYNC_STATUS_RPLY` is true: an SREQ send by the host then
/// immediately answered with a status SRSP from the device. Then at some
/// later time an AREQ from the device
pub trait AsyncRequest: Serialize + std::fmt::Debug {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: if Self::HAS_SYNC_STATUS_RPLY {
            CommandType::SREQ
        } else {
            CommandType::AREQ
        },
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };

    const TIMEOUT: Duration;
    const HAS_SYNC_STATUS_RPLY: bool;
    type Reply: AsyncReply;

    fn status_reply_meta() -> Option<CommandMeta> {
        Self::HAS_SYNC_STATUS_RPLY.then(|| CommandMeta {
            ty: CommandType::SRSP,
            sub_system: Self::SUBSYSTEM,
            id: Self::ID,
        })
    }

    fn reply_pattern(&self) -> Pattern {
        Pattern::default()
    }

    fn data_to_vec(&self) -> Result<Vec<u8>, data_format::Error>
    where
        Self: Sized,
    {
        data_format::to_vec(self)
    }

    fn to_frame(&self) -> Result<Vec<u8>, CommandError>
    where
        Self: Sized,
    {
        let serialized_data =
            self.data_to_vec().map_err(|error| CommandError {
                command: std::any::type_name::<Self>(),
                cause: error,
            })?;
        to_frame(serialized_data, Self::META)
    }
}

/// Send by the device in response to a state change.
///
/// # Note
/// that state change could very well be caused by an AsyncRequest
pub trait AsyncNotify {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
}

/// Only send by the device in response to an AsyncRequest
pub trait AsyncReply: DeserializeOwned + std::fmt::Debug {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::AREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
    type Request: AsyncRequest;

    fn from_data(data: &[u8]) -> Result<Self, ReplyError> {
        use crate::commands::ReplyErrorCause as E;
        let mut data = std::io::Cursor::new(data);
        data_format::from_reader(&mut data)
            .map_err(E::Deserialize)
            .map_err(|cause| ReplyError {
                reply: std::any::type_name::<Self>(),
                cause,
            })
    }
}
