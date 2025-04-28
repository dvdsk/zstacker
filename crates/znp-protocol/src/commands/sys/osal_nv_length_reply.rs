use std::num::NonZeroU16;

use serde::Deserialize;
use serde::de::Visitor;

use crate::commands::SyncReply;

use super::OsalNvLength;

#[derive(Debug, Clone)]
pub enum OsalNvLengthReply {
    ItemExists {
        /// Length in bytes of the NV item
        length: NonZeroU16,
    },
    ItemDoesNotExist,
}

impl<'de> Deserialize<'de> for OsalNvLengthReply {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u16(OsalNvLengthReplyVisitor)
    }
}

struct OsalNvLengthReplyVisitor;

impl<'de> Visitor<'de> for OsalNvLengthReplyVisitor {
    type Value = OsalNvLengthReply;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "Expected 16 bit value representing if the 
            item exists (all zeros) and if it is its length in bytes",
        )
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match NonZeroU16::new(v) {
            Some(nonzero) => OsalNvLengthReply::ItemExists { length: nonzero },
            None => OsalNvLengthReply::ItemDoesNotExist,
        })
    }
}

impl SyncReply for OsalNvLengthReply {
    type Request = OsalNvLength;
}
