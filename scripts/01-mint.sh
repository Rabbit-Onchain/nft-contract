#!/bin/bash
export NEAR_ENV=testnet
export ACCOUNT_ID=duonghb3.testnet
export CONTRACT_ID=dev-1677397761500-82279137383421

# 1. Mint NFT
near call $CONTRACT_ID --accountId=$ACCOUNT_ID nft_mint '{"rarity" : "Common"}' --deposit=1

# 2. Check nft_total_supply:
near view $CONTRACT_ID  nft_total_supply '{}'

# 3. Check get_nft_info:
near view $CONTRACT_ID  get_nft_info '{"token_id": "3"}'
# Return
# {
#   title: 'Rabbit Onchain',
#   description: 'Rabbit Onchain',
#   media: 'Rabbit Onchain',
#   rarity: 'Common',
#   starts_at: '1677432746508233531',
#   expires_at: '1680024746508233531'
# }

# 4. Get get_nfts_by_owner
near view $CONTRACT_ID  get_nfts_by_owner '{"owner_id": "duonghb3.testnet"}'

# 5. Extend lend nft:
near call $CONTRACT_ID --accountId=$ACCOUNT_ID extend_expired '{"token_id": "3", "time": 2592000000000000}' --depositYocto=1