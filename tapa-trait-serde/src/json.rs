use anyhow::Result as AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_slice, from_str, to_string, to_string_pretty, to_vec, to_vec_pretty};

pub trait IJsonSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_json_string(json_string: &str) -> AnyResult<Self> {
        Ok(from_str::<Self>(json_string)?)
    }

    fn from_json_slice(json_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice::<Self>(json_slice)?)
    }

    fn to_json_string(&self) -> String {
        to_string(self).unwrap()
    }

    fn to_json_string_pretty(&self) -> String {
        to_string_pretty(self).unwrap()
    }

    fn to_json_vec(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }

    fn to_json_vec_pretty(&self) -> Vec<u8> {
        to_vec_pretty(self).unwrap()
    }
}
