#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "cbor")]
mod cbor;
#[cfg(feature = "flexbuffers")]
mod flexbuffers;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "toml")]
mod toml;
#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "bincode")]
pub use crate::bincode::IBincodeSerializable;
#[cfg(feature = "cbor")]
pub use crate::cbor::ICborSerializable;
#[cfg(feature = "flexbuffers")]
pub use crate::flexbuffers::IFlexbuffersSerializable;
#[cfg(feature = "json")]
pub use crate::json::IJsonSerializable;
#[cfg(feature = "toml")]
pub use crate::toml::ITomlSerializable;
#[cfg(feature = "yaml")]
pub use crate::yaml::IYamlSerializable;
