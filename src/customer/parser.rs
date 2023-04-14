use crate::customer::customer::Customer;


use serde::{Serialize, Deserialize};
use serde::ser::SerializeStruct;
use uuid::Uuid;

impl Serialize for Customer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Customer", 2)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("balance", &self.balance)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Customer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        struct CustomerVisitor;

        impl<'de> serde::de::Visitor<'de> for CustomerVisitor {
            type Value = Customer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Customer")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Customer, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut balance = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "id" => {
                            let id_str = map.next_value::<String>()?;
                            id = Some(Uuid::parse_str(&id_str).map_err(serde::de::Error::custom)?);
                        }
                        "balance" => {
                            balance = Some(map.next_value()?);
                        }
                        _ => {}
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let balance = balance.ok_or_else(|| serde::de::Error::missing_field("balance"))?;

                Ok(Customer { id, balance })
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "balance"];
        deserializer.deserialize_struct("Customer", FIELDS, CustomerVisitor)
    }
}