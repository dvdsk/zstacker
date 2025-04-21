use std::iter;
use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::data_format;
use crate::framing::CommandMeta;

use super::{
    CommandError, CommandType, Pattern, ReplyError, START_OF_FRAME, SubSystem,
};

pub trait SyncRequest: Serialize {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::SREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
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

fn to_frame(
    serialized_data: Vec<u8>,
    meta: CommandMeta,
) -> Result<Vec<u8>, CommandError> {
    let frame = [serialized_data.len() as u8]
        .into_iter()
        .chain(meta.serialize())
        .chain(serialized_data);

    let checksum = frame
        .clone()
        .reduce(|checksum, byte| checksum ^ byte)
        .expect("never empty");

    Ok(iter::once(START_OF_FRAME)
        .chain(frame)
        .chain([checksum])
        .collect())
}

pub trait SyncReply: DeserializeOwned {
    type Request: SyncRequest;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::SRSP,
        sub_system: Self::Request::SUBSYSTEM,
        id: Self::Request::ID,
    };

    fn from_reader(
        reader: &mut impl std::io::Read,
    ) -> Result<Self, ReplyError> {
        super::from_reader_inner(reader).map_err(|cause| ReplyError {
            reply: std::any::type_name::<Self>(),
            cause,
        })
    }

    fn from_data(_data: &[u8]) -> Result<Self, ReplyError> {
        todo!()
    }
}

pub trait AsyncRequest: Serialize {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::AREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
    const TIMEOUT: Duration;
    const HAS_SYNC_STATUS_RPLY: bool;
    type Reply: AsyncReply;

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
pub trait AsyncReply: DeserializeOwned {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandMeta = CommandMeta {
        ty: CommandType::AREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
    type Request: AsyncRequest;

    fn from_data(_data: &[u8]) -> Result<Self, ReplyError> {
        todo!()
    }
}
