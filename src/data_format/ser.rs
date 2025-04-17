use serde::{Serialize, ser};

use super::error::{Error, Result};

pub struct Serializer {
    output: Vec<u8>,
}

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_bytes`.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl ser::Serializer for &mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_u8(v.cast_unsigned())
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        Err(super::Error::I16NotSupported)
    }

    fn serialize_i32(self, _: i32) -> Result<()> {
        Err(super::Error::I32NotSupported)
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        Err(super::Error::I64NotSupported)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        Err(super::Error::F32NotSupported)
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        Err(super::Error::F64NotSupported)
    }

    fn serialize_char(self, _: char) -> Result<()> {
        Err(super::Error::CharNotSupported)
    }

    fn serialize_str(self, _: &str) -> Result<()> {
        Err(super::Error::StrNotSupported)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        Err(super::Error::NoneNotSupported)
    }

    fn serialize_some<T>(self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(super::Error::SomeNotSupported)
    }

    // Empty commands are unit so their data len is zero
    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    // Unit struct means a named value containing no data.
    // There is no need to serialize the name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        dbg!(name, variant, variant_index);
        todo!()
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may, or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.push(
            len.ok_or(Error::UnknownSequenceLen)?
                .try_into()
                .map_err(Error::LenDoesNotFit)?,
        );
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::TupleUnsupported)
        // self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::TupleStructUnsupported)
        // self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::TupleVariantUnsupported)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::MapsUnsupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::StructVariantUnsupported)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl ser::SerializeSeq for &mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::TupleUnsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::TupleUnsupported)
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::TupleStructUnsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::TupleStructUnsupported)
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::TupleStructUnsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::TupleStructUnsupported)
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::MapsUnsupported)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::MapsUnsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::MapsUnsupported)
    }
}

// Structs fields are serialized in the order they where provided
impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::TupleStructUnsupported)
    }

    fn end(self) -> Result<()> {
        Err(Error::TupleStructUnsupported)
    }
}

#[cfg(test)]
mod tests {
    use serde_repr::Serialize_repr;

    use super::*;

    #[allow(dead_code)]
    #[derive(Serialize_repr)]
    #[repr(u8)]
    pub enum LatencyReq {
        NoLatency = 0x00,
        FastBeacons = 0x01,
        SlowBeacons = 0x02,
    }

    #[derive(Serialize)]
    struct AfRegister {
        end_point: u8,
        app_prof_id: u16,
        app_device_id: u16,
        app_dev_ver: u8,
        latency_req: LatencyReq,
        app_in_cluster_list: Vec<u16>,
        app_out_cluster_list: Vec<u16>,
    }

    /// Might have been tested so might be usefully comparison
    fn org_code(
        AfRegister {
            end_point,
            app_prof_id,
            app_device_id,
            app_dev_ver,
            latency_req,
            app_in_cluster_list,
            app_out_cluster_list,
        }: AfRegister,
    ) -> Vec<u8> {
        pub fn encode_short(short: u16, buf: &mut [u8], offset: usize) {
            buf[offset] = (short & 0xFF) as u8;
            buf[offset + 1] = ((short >> 8) & 0xFF) as u8;
        }

        pub fn encode_short_slice(
            shorts: &[u16],
            buf: &mut [u8],
            offset: usize,
        ) {
            let buf_len = buf.len();

            if offset >= buf_len || offset + (shorts.len() * 2) >= buf_len {
                panic!("Offset and length exceed the buffer size");
            }

            let mut i = offset;
            for short in shorts {
                encode_short(*short, buf, i);
                i += 2;
            }
        }

        let app_num_in_clusters = app_in_cluster_list.len() as u8;
        let app_num_out_clusters = app_out_cluster_list.len() as u8;

        let data_len =
            0x09 + (app_num_in_clusters * 2) + (app_num_out_clusters * 2);
        let mut data: [u8; 256] = [0; 256];
        data[0] = end_point;
        encode_short(app_prof_id, &mut data, 1);
        encode_short(app_device_id, &mut data, 3);
        data[5] = app_dev_ver;
        data[6] = latency_req as u8;
        data[7] = app_num_in_clusters;
        encode_short_slice(
            &app_in_cluster_list[..app_num_in_clusters as usize],
            &mut data,
            8,
        );
        let out_clusters_offset = 8 + (app_num_in_clusters * 2) as usize;
        data[out_clusters_offset] = app_num_out_clusters;
        encode_short_slice(
            &app_out_cluster_list[..app_num_out_clusters as usize],
            &mut data,
            out_clusters_offset + 1,
        );
        (&data[..data_len as usize]).to_vec()
    }

    #[test]
    fn test_name() {
        let reg = AfRegister {
            end_point: 42,
            app_prof_id: 13,
            app_device_id: 4,
            app_dev_ver: 10,
            latency_req: LatencyReq::FastBeacons,
            app_in_cluster_list: vec![16000, 12000, 800],
            app_out_cluster_list: vec![16, 12, 9000],
        };

        let serialized = to_vec(&reg).unwrap();
        let control = org_code(reg);
        assert_eq!(serialized, control)
    }
}
