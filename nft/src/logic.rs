use crate::*;

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
    pub fn nft_mint(&mut self, rarity: Rarity) -> Token {
        let receiver_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        let mut check_deposit_near = false;
        let mut media = URL_COMMON_NFT;
        match rarity {
            Rarity::Conmon => {
                check_deposit_near = deposit == ONE_NEAR;
            }
            Rarity::Rare => {
                check_deposit_near = deposit == FIVE_NEAR;
                media = URL_RARE_NFT;
            }
            Rarity::Mythic => {
                check_deposit_near = deposit == TEN_NEAR;
                media = URL_MYTHIC_NFT;
            }
        };

        assert!(check_deposit_near, "Deposit Near wrong!");

        let mut token_metadata = default_metadata();
        token_metadata.media = Some(media.to_owned());

        if token_metadata.starts_at.is_none() {
            token_metadata.starts_at = Some(env::block_height().to_string());
        }

        if token_metadata.expires_at.is_none() {
            token_metadata.expires_at = Some((env::block_height() + DEFAULT_EXPIRES).to_string());
        }
        self.token_metadata_extend
            .insert(&self.token_id.to_string(), &TokenMetadataExtend { rarity });
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
}
