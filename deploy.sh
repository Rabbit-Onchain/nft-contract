# 0. Build
./build.sh

# 1. Deploy:
# near deploy --wasmFile ./res/non_fungible_token.wasm --accountId $CONTRACT_NFT_ID
near dev-deploy --wasmFile ./res/non_fungible_token.wasm