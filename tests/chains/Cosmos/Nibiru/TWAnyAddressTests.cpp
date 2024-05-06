// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

#include "../CosmosTestHelpers.h"

namespace TW::Cosmos::tests {

static const std::string gNibiAddr = "nibi1mry47pkga5tdswtluy0m8teslpalkdq0fy5h4x";
static const std::string gNibiHrp = "nibi";

TEST(TWNibiAnyAddress, AllNibiAddressTests) {
    CosmosAddressParameters parameters{.hrp = gNibiHrp, .coinType = TWCoinTypeNibiru, .address = gNibiAddr};
    TestCosmosAddressParameters(parameters);
}

}
 