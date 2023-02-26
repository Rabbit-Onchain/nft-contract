use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_nft_info(&self, token_id: TokenId) -> Option<MetadataJson> {
        let token = self.tokens.nft_token(token_id.clone()).unwrap();
        let rarity = self.token_metadata_extend.get(&token_id).unwrap().rarity;

        Some(MetadataJson {
            title: token.metadata.clone().unwrap().title,
            description: token.metadata.clone().unwrap().description,
            media: token.metadata.clone().unwrap().description,
            rarity,
            starts_at: token.metadata.clone().unwrap().starts_at,
            expires_at: token.metadata.clone().unwrap().expires_at,
        })
    }

    pub fn get_nfts_by_owner(&self, owner_id: AccountId) -> Option<Vec<TokenId>> {
        match &self.tokens.tokens_per_owner {
            Some(tokens) => Some(tokens.get(&owner_id).unwrap().to_vec()),
            None => None,
        }
    }
}
