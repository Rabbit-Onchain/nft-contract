use crate::*;

#[near_bindgen]
impl Contract {
    /// Mint a new token
    #[payable]
    pub fn nft_mint(&mut self, title: Option<String>, rarity: Rarity) -> Token {
        let receiver_id = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        let check_deposit_near;
        let mut media = URL_COMMON_NFT;
        let mut _rarity = Rarity::Common;
        match rarity {
            Rarity::Common => {
                check_deposit_near = deposit == ONE_NEAR;
            }
            Rarity::Rare => {
                // _rarity = Rarity::Rare;
                check_deposit_near = deposit == FIVE_NEAR;
                media = URL_RARE_NFT;
            }
            Rarity::Mythic => {
                // _rarity = Rarity::Mythic;
                check_deposit_near = deposit == TEN_NEAR;
                media = URL_MYTHIC_NFT;
            } // _ => unreachable!(),
        };

        assert!(check_deposit_near, "Deposit Near wrong!");

        let mut token_metadata = default_metadata();

        token_metadata.title = title;
        token_metadata.media = Some(media.to_owned());
        token_metadata.starts_at = Some(env::block_timestamp().to_string());
        token_metadata.expires_at = Some((env::block_timestamp() + DEFAULT_EXPIRES).to_string());
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
        self.assert_one_yocto();
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
