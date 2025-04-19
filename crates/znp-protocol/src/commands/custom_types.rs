use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

#[derive(Debug, Clone)]
struct Network {
    // PAN ID of the neighbor device
    pan_id: u16,
    // The current logical channel occupied by the network.
    logical_channel: u8,
    zigbee_version: u8,
    beacon_order: u8,
    permit_joining: u8,
}


impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { PanId, LogicalChannel, ZigbeeVersion, BeaconOrder, PermitJoining }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        unimplemented!()
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Duration, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Duration::new(secs, nanos))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Duration, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut secs = None;
                let mut nanos = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            secs = Some(map.next_value()?);
                        }
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(map.next_value()?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
                let nanos = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;
                Ok(Duration::new(secs, nanos))
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}
