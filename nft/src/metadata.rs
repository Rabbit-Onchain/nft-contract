use crate::*;
use near_sdk::{
    serde::{Deserialize, Serialize},
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum Rarity {
    Common,
    Rare,
    Mythic,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadataExtend {
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MetadataJson {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub rarity: Rarity,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
}
