use anyhow::Result as AnyResult;
use rmp_serde::{from_slice, to_vec};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IMessagePackSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Send + Sized,
{
    fn from_msgpack_bytes(msgpack_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice(msgpack_slice)?)
    }

    fn to_msgpack_bytes(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }
}
