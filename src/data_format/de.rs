use std::io::{BufReader, Read};

use serde::de::DeserializeOwned;
use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};

use super::error::{Error, Result};

pub struct Deserializer<R> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    reader: BufReader<R>,
}

impl<R: Read> Deserializer<R> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_reader(reader: R) -> Self {
        Deserializer {
            reader: BufReader::new(reader),
        }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_reader<T>(reader: &mut impl Read) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_reader(reader);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<R: Read> Deserializer<R> {
    fn parse_bool(&mut self) -> Result<bool> {
        let byte = self.parse_u8()?;
        if byte == true as u8 {
            Ok(true)
        } else if byte == false as u8 {
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn parse_u8(&mut self) -> Result<u8> {
        let mut byte = [0u8];
        self.reader.read_exact(&mut byte).map_err(Error::Reading)?;
        Ok(byte[0])
    }

    fn parse_i8(&mut self) -> Result<i8> {
        let mut byte = [0u8];
        self.reader.read_exact(&mut byte).map_err(Error::Reading)?;
        Ok(byte[0].cast_signed())
    }

    fn parse_u16(&mut self) -> Result<u16> {
        let mut bytes = [0u8, 0u8];
        self.reader.read_exact(&mut bytes).map_err(Error::Reading)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn parse_u32(&mut self) -> Result<u32> {
        let mut bytes = [0u8; 4];
        self.reader.read_exact(&mut bytes).map_err(Error::Reading)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn parse_u64(&mut self) -> Result<u64> {
        let mut bytes = [0u8; 8];
        self.reader.read_exact(&mut bytes).map_err(Error::Reading)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

impl<'de, R: Read> de::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::FormatIsNotSelfDescribing)
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::I16NotSupported)
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::I32NotSupported)
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::I64NotSupported)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::F32NotSupported)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::F64NotSupported)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::CharNotSupported)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::StrNotSupported)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::StrNotSupported)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
        // visitor.visit_seq(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
        // visitor.visit_byte_buf(self.parse_bytes()?)
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::OptionNotSupported)
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnitNotSupported)
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnitStructNotSupported)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.parse_u8()?;
        let value = visitor.visit_seq(KnownLenSenquence {
            de: self,
            left: len as usize,
        })?;
        Ok(value)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(KnownLenSenquence {
            left: len,
            de: self,
        })
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::TupleStructUnsupported)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::MapsUnsupported)
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::EnumUnsupported)
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::IdentifierUnsupported)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::FormatIsNotSelfDescribing)
    }
}

struct KnownLenSenquence<'a, R> {
    de: &'a mut Deserializer<R>,
    left: usize,
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a, R: Read> SeqAccess<'de> for KnownLenSenquence<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.left == 0 {
            return Ok(None);
        }
        self.left -= 1;
        dbg!(self.left);
        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_repr::Deserialize_repr;
    use std::io::Cursor;

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            a_number: u32,
            a_list: Vec<u8>,
        }

        let mut j = Cursor::new([42, 0, 0, 0, 5, 1, 2, 3, 4, 5]);
        let expected = Test {
            a_number: 42,
            a_list: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(expected, from_reader(&mut j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize_repr, PartialEq, Debug)]
        #[repr(u8)]
        enum E {
            One = 1,
            Ten = 10,
            FourTeen = 14,
        }

        let mut j = Cursor::new(vec![10]);
        let expected = E::Ten;
        assert_eq!(expected, from_reader(&mut j).unwrap());

        let mut j = Cursor::new(vec![14]);
        let expected = E::FourTeen;
        assert_eq!(expected, from_reader(&mut j).unwrap());

        let mut j = Cursor::new(vec![1]);
        let expected = E::One;
        assert_eq!(expected, from_reader(&mut j).unwrap());
    }
}
