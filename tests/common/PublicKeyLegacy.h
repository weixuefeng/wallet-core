// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#pragma once

#include <TrezorCrypto/ecdsa.h>
#include <TrezorCrypto/nist256p1.h>
#include "Data.h"

namespace TW::TrezorCrypto {

/// Verifies a signature for the provided message by using `trezor-crypto` library (legacy implementation).
inline bool verifyNist256p1Signature(const Data& publicKey, const Data& signature, const Data& message) {
    return ecdsa_verify_digest(&nist256p1, publicKey.data(), signature.data(), message.data()) == 0;
}

} // namespace TW::TrezorCrypto
