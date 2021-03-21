use crate::SerializeError;

pub struct Argument {
    pub(crate) name: String,
    pub(crate) serialize_result: Result<serde_json::Value, SerializeError>,
    pub(crate) type_: String,
}

impl Argument {
    pub fn new(name: &str, gql_type: &str, value: impl serde::Serialize) -> Argument {
        Argument {
            name: name.to_string(),
            serialize_result: serde_json::to_value(value).map_err(Into::into),
            type_: gql_type.to_string(),
        }
    }
}

impl serde::Serialize for Argument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;

        match &self.serialize_result {
            Ok(json_val) => serde::Serialize::serialize(json_val, serializer),
            Err(e) => Err(S::Error::custom(e.to_string())),
        }
    }
}
