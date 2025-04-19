use std::iter;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::data_format;
use crate::framing::CommandInfo;

use super::{CommandError, CommandType, ReplyError, START_OF_FRAME, SubSystem};

pub trait SyncRequest: Serialize {
    const ID: u8;
    const SUBSYSTEM: SubSystem;
    const META: CommandInfo = CommandInfo {
        ty: CommandType::SREQ,
        sub_system: Self::SUBSYSTEM,
        id: Self::ID,
    };
    type Reply: SyncReply;

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
        let serialized = self.data_to_vec().map_err(|error| CommandError {
            command: std::any::type_name::<Self>(),
            cause: error,
        })?;
        let frame = [serialized.len() as u8]
            .into_iter()
            .chain(Self::META.serialize())
            .chain(serialized);

        let checksum = frame
            .clone()
            .reduce(|checksum, byte| checksum ^ byte)
            .expect("never empty");

        Ok(iter::once(START_OF_FRAME)
            .chain(frame)
            .chain([checksum])
            .collect())
    }
}

pub trait SyncReply: DeserializeOwned {
    type Request: SyncRequest;
    const META: CommandInfo = CommandInfo {
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
}

pub trait AsyncRequest: Serialize {
    const ID: u8;
    const SUBSYSTEM: SubSystem;

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
        todo!();
    }
}

pub trait AsyncReply: DeserializeOwned {
    const ID: u8;
    const SUBSYSTEM: SubSystem;

    fn from_reader(_: &mut impl std::io::Read) -> Result<Self, ReplyError> {
        todo!()
    }
}
