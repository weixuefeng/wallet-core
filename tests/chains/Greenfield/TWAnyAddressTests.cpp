// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#include <TrustWalletCore/TWAnyAddress.h>
#include "HexCoding.h"

#include "TestUtilities.h"
#include <gtest/gtest.h>

using namespace TW;

TEST(TWGreenfield, Address) {
    auto string = STRING("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed");
    auto addr = WRAP(TWAnyAddress, TWAnyAddressCreateWithString(string.get(), TWCoinTypeGreenfield));
    auto actual = WRAPS(TWAnyAddressDescription(addr.get()));
    auto expected = STRING("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    EXPECT_TRUE(TWStringEqual(actual.get(), expected.get()));
}
