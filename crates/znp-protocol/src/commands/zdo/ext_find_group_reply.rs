use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GroupName(pub String);

#[cfg_attr(feature = "mocking", derive(Deserialize))]
#[derive(Debug, Clone, Serialize)]
pub struct ExtFindGroupReply {
    pub group_id: u16,
    pub group_name: GroupName,
}

impl<'de> Deserialize<'de> for GroupName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(GroupNameVisitor)
    }
}

struct GroupNameVisitor;

impl<'de> Visitor<'de> for GroupNameVisitor {
    type Value = GroupName;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "struct ExtFindGroupReply is 18 bytes the first two the group id. 
            Then the group name",
        )
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let group_name = seq
            .next_element::<[u8; 16]>()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;

        Ok(GroupName(String::from_utf8(group_name.to_vec()).map_err(
            |_| de::Error::custom("Expected group name to be valid utf8"),
        )?))
    }
}

#[cfg(feature = "mocking")]
impl Serialize for GroupName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let name = self
            .0
            .as_bytes()
            .iter()
            .chain(std::iter::repeat(&0))
            .take(16);
        let mut tup = serializer.serialize_tuple(16)?;
        for element in name {
            tup.serialize_element(element)?;
        }
        tup.end()
    }
}
