use anyhow::Result as AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_yaml::{from_slice, from_str, to_string, to_vec};

pub trait IYamlSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Send + Sized,
{
    fn from_yaml_string(yaml_string: &str) -> AnyResult<Self> {
        Ok(from_str::<Self>(yaml_string)?)
    }

    fn from_yaml_bytes(yaml_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice::<Self>(yaml_slice)?)
    }

    fn to_yaml_string(&self) -> String {
        to_string(self).unwrap()
    }

    fn to_yaml_bytes(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }
}
