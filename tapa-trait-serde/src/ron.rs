use anyhow::Result as AnyResult;
use ron::{from_str, to_string};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait IRonSerializable<T = Self>
where
    Self: DeserializeOwned + Serialize + Send + Sized,
{
    fn from_ron_string(ron_string: &str) -> AnyResult<Self> {
        Ok(from_str::<Self>(ron_string)?)
    }

    fn to_ron_string(&self) -> String {
        to_string(self).unwrap()
    }
}
