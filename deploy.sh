# 0. Build
./build.sh

# 1. Deploy:
# near deploy --wasmFile ./out/nft-contract.wasm --accountId $CONTRACT_NFT_ID
near dev-deploy --wasmFile ./out/nft-contract.wasm