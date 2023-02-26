export NEAR_ENV=testnet
export ACCOUNT_ID=duonghb3.testnet
export CONTRACT_ID=dev-1677397761500-82279137383421

# 0. Build
./build.sh

# 1. Deploy:
# near deploy --wasmFile ./res/non_fungible_token.wasm --accountId $CONTRACT_NFT_ID
near dev-deploy --wasmFile ./res/non_fungible_token.wasm

# 2. Init contract:
echo "################### Init Contract ###################"
near call $CONTRACT_ID --accountId=$ACCOUNT_ID new_default_meta '{"owner_id" : "'$ACCOUNT_ID'"}'

# 3. Check owner:
near view $CONTRACT_ID  get_owner '{}'

# 4. Check nft_total_supply:
near view $CONTRACT_ID  nft_total_supply '{}'
