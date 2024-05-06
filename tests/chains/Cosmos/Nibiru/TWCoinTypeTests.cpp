// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.
//
// This is a GENERATED FILE, changes made here MAY BE LOST.
// Generated one-time (codegen/bin/cointests)
//

#include "TestUtilities.h"
#include <TrustWalletCore/TWCoinTypeConfiguration.h>
#include <gtest/gtest.h>

namespace TW::Cosmos::tests {

TEST(TWNibiCoinType, TWCoinType) {
    const auto coin = TWCoinTypeNibi;
    const auto symbol = WRAPS(TWCoinTypeConfigurationGetSymbol(coin));
    const auto id = WRAPS(TWCoinTypeConfigurationGetID(coin));
    const auto name = WRAPS(TWCoinTypeConfigurationGetName(coin));
    const auto chainId = WRAPS(TWCoinTypeChainId(coin));
    const auto txId = WRAPS(TWStringCreateWithUTF8Bytes("FF370C65D8D67B8236F9D3A8D2B1256337C60C1965092CADD1FA970288FCE99B"));
    const auto txUrl = WRAPS(TWCoinTypeConfigurationGetTransactionURL(coin, txId.get()));
    const auto accId = WRAPS(TWStringCreateWithUTF8Bytes("nibi1mry47pkga5tdswtluy0m8teslpalkdq0fy5h4x"));
    const auto accUrl = WRAPS(TWCoinTypeConfigurationGetAccountURL(coin, accId.get()));
    assertStringsEqual(id, "nibi");
    assertStringsEqual(name, "Nibiru");
    assertStringsEqual(symbol, "NIBI");
    ASSERT_EQ(TWCoinTypeConfigurationGetDecimals(coin), 6);
    ASSERT_EQ(TWCoinTypeBlockchain(coin), TWBlockchainCosmos);
    ASSERT_EQ(TWCoinTypeP2shPrefix(coin), 0x0);
    ASSERT_EQ(TWCoinTypeStaticPrefix(coin), 0x0);
    assertStringsEqual(chainId, "cataclysm-1");
    assertStringsEqual(txUrl, "https://explorer.nibiru.fi/cataclysm-1/tx/FF370C65D8D67B8236F9D3A8D2B1256337C60C1965092CADD1FA970288FCE99B");
    assertStringsEqual(accUrl, "https://explorer.nibiru.fi/cataclysm-1/account/nibi1mry47pkga5tdswtluy0m8teslpalkdq0fy5h4x");
}

} // namespace TW::Cosmos::tests
