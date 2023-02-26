use crate::*;

pub fn default_metadata() -> TokenMetadata {
    TokenMetadata {
        title: Some("Rabbit Onchain".to_owned()),
        description: Some("Rabbit Onchain".to_owned()),
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    }
}
