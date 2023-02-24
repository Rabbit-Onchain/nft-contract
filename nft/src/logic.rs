use near_sdk::collections::LookupMap;

use crate::{constants::DEFAULT_EXPIRES, *};

#[near_bindgen]
impl Contract {
    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    #[payable]
    pub fn nft_mint(&mut self, rarity: Rarity, token_metadata: TokenMetadata) -> Token {
        let receiver_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        let mut token_metadata = token_metadata;

        if token_metadata.starts_at.is_none() {
            token_metadata.starts_at = Some(env::block_height().to_string());
        }

        if token_metadata.expires_at.is_none() {
            token_metadata.expires_at = Some((env::block_height() + DEFAULT_EXPIRES).to_string());
        }

        let check_deposit_near = match rarity {
            Rarity::Conmon => deposit == ONE_NEAR,
            Rarity::Rare => deposit == FIVE_NEAR,
            Rarity::Mythic => deposit == TEN_NEAR,
        };

        assert!(check_deposit_near, "Deposit Near wrong!");
        let token =
            self.tokens
                .internal_mint(self.token_id.to_string(), receiver_id, Some(token_metadata));
        self.token_id += 1;
        token
    }

    #[payable]
    pub fn extend_expired(&mut self, token_id: TokenId, time: u64) {
        let token_metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .unwrap()
            .get(&token_id);
        let mut token_metadata = token_metadata.unwrap();
        token_metadata.expires_at =
            Some((token_metadata.expires_at.unwrap().parse::<u64>().unwrap() + time).to_string());

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &token_metadata));
    }

    pub fn nft_check_exist(&self, account_id: AccountId, token_id: TokenId) -> bool {
        // let tokens = self.tokens_per_owner.get(&account_id);
        // match tokens {
        //     Some(tokens) => tokens.contains(&token_id),
        //     None => false,
        // }
        todo!()
    }
}
