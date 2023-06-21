#!/bin/bash
SOURCE_ROOT=/Users/weixuefeng/source/gitlab/forceWallet/tech_research/wallet-core
FLUTTER_ROOT=/Users/weixuefeng/source/bitbucket/web3_wallet_core
WASM_ROOT=/Users/weixuefeng/source/bitbucket/web3_wallet_core_lib

cp -r ${SOURCE_ROOT}/swift/build/WalletCore.xcframework ${FLUTTER_ROOT}/ios/Frameworks
cp -r ${SOURCE_ROOT}/swift/build/SwiftProtobuf.xcframework ${FLUTTER_ROOT}/ios/Frameworks
cp -r ${SOURCE_ROOT}/wasm/dist/* ${WASM_ROOT}/dist/
cp -r ${SOURCE_ROOT}/build/wallet-core.aar ${FLUTTER_ROOT}/android/libs/trustwalletcore.aar
echo "copy success"
