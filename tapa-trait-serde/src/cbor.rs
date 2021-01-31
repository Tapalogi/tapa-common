use anyhow::Result as AnyResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_cbor::{from_slice, to_vec};

pub trait ICborSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Send + Sized,
{
    fn from_cbor_bytes(cbor_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice(cbor_slice)?)
    }

    fn to_cbor_bytes(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }
}
