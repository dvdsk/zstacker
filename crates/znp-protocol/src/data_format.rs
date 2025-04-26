//! This is a `serde` implementation for the `zpn` data format
//! 
//! The data format is send in a `zstack` frame:
//! ______________________________________
//! n bytes: | 1       | 2       | 0-250
//!          | Length  | Command | Data
//! ______________________________________
//! Length: length of data field (0 to 250)
//!

mod ser;
mod error;
mod de;

pub use de::{from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_vec, Serializer};
