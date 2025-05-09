use std;
use std::fmt::{self, Display};
use std::num::TryFromIntError;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

// This is a bare-bones implementation. A real library would provide additional
// information in its error type, for example the line and column at which the
// error occurred, the byte offset into the input, or the current key being
// processed.
#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case JSON.
    Eof,
    Syntax,
    ExpectedBoolean(u8),
    ExpectedInteger,
    ExpectedString,
    ExpectedNull,
    ExpectedArray,
    ExpectedArrayComma,
    ExpectedArrayEnd,
    ExpectedMap,
    ExpectedMapColon,
    ExpectedMapComma,
    ExpectedMapEnd,
    ExpectedEnum,
    TrailingCharacters,
    I8NotSupported,
    I16NotSupported,
    I32NotSupported,
    I64NotSupported,
    F32NotSupported,
    F64NotSupported,
    CharNotSupported,
    StrNotSupported,
    NoneNotSupported,
    SomeNotSupported,
    UnknownSequenceLen,
    LenDoesNotFit(TryFromIntError),
    TupleUnsupported,
    TupleStructUnsupported,
    TupleVariantUnsupported,
    MapsUnsupported,
    StructVariantUnsupported,
    FormatIsNotSelfDescribing,
    OptionNotSupported,
    UnitStructNotSupported,
    UnitDeserNotSupported,
    EnumUnsupported,
    IdentifierUnsupported,
    Reading(std::io::Error),
    TupleUnsupported1,
    TupleUnsupported2,
    TupleUnsupported3,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::ExpectedBoolean(byte) => formatter
                .write_fmt(format_args!("expected boolean got: 0b{byte:0b}")),
            _ => formatter.write_fmt(format_args!("{self:?}")),
            /* and so forth */
        }
    }
}

impl std::error::Error for Error {}
