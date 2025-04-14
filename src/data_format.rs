mod ser;
mod error;
mod de;

pub use de::{from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_vec, Serializer};
