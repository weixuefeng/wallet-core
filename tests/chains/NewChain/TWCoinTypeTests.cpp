// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

#include "TestUtilities.h"
#include <TrustWalletCore/TWCoinTypeConfiguration.h>
#include <gtest/gtest.h>

TEST(TWNewChainCoinType, TWCoinType) {
    const auto coin = TWCoinTypeNewChain;
    const auto symbol = WRAPS(TWCoinTypeConfigurationGetSymbol(coin));
    const auto id = WRAPS(TWCoinTypeConfigurationGetID(coin));
    const auto name = WRAPS(TWCoinTypeConfigurationGetName(coin));
    const auto txId = WRAPS(TWStringCreateWithUTF8Bytes("0xb9ae2e808fe8e57171f303ad8f6e3fd17d949b0bfc7b4db6e8e30a71cc517d7e"));
    const auto txUrl = WRAPS(TWCoinTypeConfigurationGetTransactionURL(coin, txId.get()));
    const auto accId = WRAPS(TWStringCreateWithUTF8Bytes("0x35552c16704d214347f29Fa77f77DA6d75d7C752"));
    const auto accUrl = WRAPS(TWCoinTypeConfigurationGetAccountURL(coin, accId.get()));

    assertStringsEqual(id, "newchain");
    assertStringsEqual(name, "NewChain");
    assertStringsEqual(symbol, "AB");
    ASSERT_EQ(TWCoinTypeConfigurationGetDecimals(coin), 18);
    ASSERT_EQ(TWCoinTypeBlockchain(coin), TWBlockchainNewChain);
    ASSERT_EQ(TWCoinTypeP2pkhPrefix(coin), 0);
    ASSERT_EQ(TWCoinTypeP2shPrefix(coin), 0);
    ASSERT_EQ(TWCoinTypeStaticPrefix(coin), 0);
    assertStringsEqual(txUrl, "https://explorer.newtonproject.org/tx/0xb9ae2e808fe8e57171f303ad8f6e3fd17d949b0bfc7b4db6e8e30a71cc517d7e");
    assertStringsEqual(accUrl, "https://explorer.newtonproject.org/address/0x35552c16704d214347f29Fa77f77DA6d75d7C752");
}
