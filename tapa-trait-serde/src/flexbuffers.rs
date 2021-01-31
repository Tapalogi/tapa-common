use anyhow::Result as AnyResult;
use flexbuffers::{from_slice, to_vec};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IFlexbuffersSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Clone + Send + Sized,
{
    fn from_flexbuffers_bytes(flexbuffers_slice: &[u8]) -> AnyResult<Self> {
        Ok(from_slice(flexbuffers_slice)?)
    }

    fn to_flexbuffers_bytes(&self) -> Vec<u8> {
        to_vec(self).unwrap()
    }
}
