use anyhow::Result as AnyResult;
use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IBincodeSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Send + Sized,
{
    fn from_bincode_bytes(bincode_slice: &[u8]) -> AnyResult<Self> {
        Ok(deserialize(bincode_slice)?)
    }

    fn to_bincode_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }
}
