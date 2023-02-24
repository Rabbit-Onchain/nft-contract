use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_nft_info(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }

    pub fn get_nfts(&self, owner_id: AccountId) -> Option<Vec<TokenId>> {
        match &self.tokens.tokens_per_owner {
            Some(tokens) => Some(tokens.get(&owner_id).unwrap().to_vec()),
            None => None,
        }
    }
}
