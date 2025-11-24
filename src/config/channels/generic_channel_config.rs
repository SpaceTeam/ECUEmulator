use crate::config::config_representation::Variable;
use crate::config::serde_binary_deserialize::deserialize_prefixed_biguint;
use crate::config::serde_binary_deserialize::deserialize_prefixed_u32;
use crate::protocol::channels::FlashStatus;
use num_bigint::BigUint;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenericChannelConfig {
    pub generic_request_data: Option<GenericReqdata>,
    pub generic_request_node_info: Option<GenericRequestNodeInfo>,
    pub generic_request_flash_clear: Option<GenericReqFlashClear>,
    pub variables: Option<Vec<Variable>>,
}

#[derive(Deserialize)]
pub struct GenericReqdata {
    #[serde(deserialize_with = "deserialize_prefixed_u32")]
    channel_mask: u32,
    #[serde(deserialize_with = "deserialize_prefixed_biguint")]
    data: BigUint,
}

#[derive(Deserialize)]
pub struct GenericRequestNodeInfo {
    #[serde(deserialize_with = "deserialize_prefixed_u32")]
    firmware_version: u32,
    #[serde(deserialize_with = "deserialize_prefixed_u32")]
    channel_mask: u32,
    #[serde(deserialize_with = "deserialize_prefixed_biguint")]
    //TODO change to Option<BigUint> if needed(probably should..) -> implement deserialize for Option<BigUint>
    data: BigUint,
}

#[derive(Deserialize)]
pub struct GenericReqFlashClear {
    status: FlashStatus,
}
