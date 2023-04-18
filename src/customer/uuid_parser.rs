use serde::{Deserialize, Serialize, Serializer, Deserializer};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UuidWrapper(pub Uuid);

impl UuidWrapper {
    pub(crate) fn new_v4() -> UuidWrapper {
        UuidWrapper(Uuid::new_v4())
    }
}

impl std::fmt::Display for UuidWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Serialize for UuidWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for UuidWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s).map_err(serde::de::Error::custom).map(UuidWrapper)
    }
}
