#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "cbor")]
mod cbor;
#[cfg(feature = "flexbuffers")]
mod flexbuffers;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "msgpack")]
mod msgpack;
#[cfg(feature = "ron")]
mod ron;
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
#[cfg(feature = "msgpack")]
pub use crate::msgpack::IMessagePackSerializable;
#[cfg(feature = "ron")]
pub use crate::ron::IRonSerializable;
#[cfg(feature = "toml")]
pub use crate::toml::ITomlSerializable;
#[cfg(feature = "yaml")]
pub use crate::yaml::IYamlSerializable;
#[cfg(feature = "bincode")]
pub use tapa_trait_serde_derive::IBincodeSerializable;
#[cfg(feature = "cbor")]
pub use tapa_trait_serde_derive::ICborSerializable;
#[cfg(feature = "flexbuffers")]
pub use tapa_trait_serde_derive::IFlexbuffersSerializable;
#[cfg(feature = "json")]
pub use tapa_trait_serde_derive::IJsonSerializable;
#[cfg(feature = "msgpack")]
pub use tapa_trait_serde_derive::IMessagePackSerializable;
#[cfg(feature = "ron")]
pub use tapa_trait_serde_derive::IRonSerializable;
#[cfg(feature = "toml")]
pub use tapa_trait_serde_derive::ITomlSerializable;
#[cfg(feature = "yaml")]
pub use tapa_trait_serde_derive::IYamlSerializable;
