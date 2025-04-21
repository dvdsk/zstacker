use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, de};

use crate::commands::{DeviceType, IeeeAddr, ShortAddr};

#[derive(Debug, Clone)]
pub struct NeighborLqi {
    pub extended_pan_id: u8,
    pub extended_address: IeeeAddr,
    pub network_address: ShortAddr,
    pub device_type: DeviceType,
    pub rx_on_when_idle: u8,
    pub relationship: u8,
    pub permit_joining: bool,
    pub depth: u8,
    pub lqi: u8,
}

struct NeighborLqiVisitor;

impl<'de> Visitor<'de> for NeighborLqiVisitor {
    type Value = NeighborLqi;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "struct NeighborLqi with single byte representing \
            `device_type`, `rx_on_when_idle` and `relationship`",
        )
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let extended_pan_id = seq
            .next_element::<u8>()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let extended_address = seq
            .next_element::<IeeeAddr>()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let network_address = seq
            .next_element::<ShortAddr>()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let packed = seq
            .next_element::<u8>()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        let permit_joining = seq
            .next_element::<bool>()?
            .ok_or_else(|| de::Error::invalid_length(4, &self))?;
        let depth = seq
            .next_element::<u8>()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;
        let lqi = seq
            .next_element::<u8>()?
            .ok_or_else(|| de::Error::invalid_length(6, &self))?;

        Ok(NeighborLqi {
            extended_pan_id,
            extended_address,
            network_address,
            device_type: DeviceType::from_repr(packed & 0b0000_0011)
                .expect("can only represent valid values for DeviceType"),
            rx_on_when_idle: (packed & 0b0000_1100) >> 2,
            relationship: (packed & 0b1111_0000) >> 4,
            permit_joining,
            depth,
            lqi,
        })
    }
}

impl<'de> Deserialize<'de> for NeighborLqi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "NeighborLqi",
            &[
                "extended_pan_id",
                "extended_address",
                "network_address",
                "bit_packed_lqi_entry",
                "permit_joining",
                "depth",
                "lqi",
            ],
            NeighborLqiVisitor,
        )
    }
}
