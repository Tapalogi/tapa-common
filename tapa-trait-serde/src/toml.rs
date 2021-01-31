use anyhow::Result as AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use toml::{from_slice, from_str, to_string, to_string_pretty, to_vec};

pub trait ITomlSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_toml_string(toml_string: &str) -> AnyResult<Self> {
        Ok(from_str::<Self>(toml_string)?)
    }

    fn from_toml_bytes(toml_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice::<Self>(toml_slice)?)
    }

    fn to_toml_string(&self) -> String {
        to_string(self).unwrap()
    }

    fn to_toml_string_pretty(&self) -> String {
        to_string_pretty(self).unwrap()
    }

    fn to_toml_bytes(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }

    fn to_toml_bytes_pretty(&self) -> Vec<u8> {
        to_string_pretty(self).unwrap().as_bytes().to_vec()
    }
}
