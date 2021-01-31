use anyhow::Result as AnyResult;
use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IBincodeSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_bincode(bincode_slice: &[u8]) -> AnyResult<Self> {
        Ok(deserialize(bincode_slice)?)
    }

    fn to_bincode(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }
}
