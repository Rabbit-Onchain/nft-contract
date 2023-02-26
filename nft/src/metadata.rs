use crate::*;
use near_sdk::{
    serde::{Deserialize, Serialize},
    BlockHeight,
};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Rarity {
    Conmon,
    Rare,
    Mythic,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct  TokenMetadataExtend {
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MetadataJson {
    pub title: Option<String>,
    pub description: Option<String>,
    pub rarity: Rarity,
    pub starts_at: BlockHeight,
    pub expires_at: BlockHeight,
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
}
