// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#pragma once

#include "../Data.h"
#include "../proto/Bitcoin.pb.h"

#include <optional>

namespace TW::Zen {

using SigningInput = Bitcoin::Proto::SigningInput;
using SigningOutput = Bitcoin::Proto::SigningOutput;
using TransactionPlan = Bitcoin::Proto::TransactionPlan;
using PreSigningOutput = Bitcoin::Proto::PreSigningOutput;

/// Helper class that performs Zen transaction signing.
class Signer {
public:
    /// Hide default constructor
    Signer() = delete;

    /// Returns a transaction plan (utxo selection, fee estimation)
    static TransactionPlan plan(const SigningInput& input) noexcept;

    /// Signs a SigningInput transaction
    static SigningOutput sign(const SigningInput& input, std::optional<SignaturePubkeyList> optionalExternalSigs = {}) noexcept;

    /// Collect pre-image hashes to be signed
    static PreSigningOutput preImageHashes(const SigningInput& input) noexcept;
};

} // namespace TW::Zen
