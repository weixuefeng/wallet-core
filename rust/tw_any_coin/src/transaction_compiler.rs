// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use tw_coin_entry::coin_entry::{PublicKeyBytes, SignatureBytes};
use tw_coin_entry::error::{SigningError, SigningResult};
use tw_coin_registry::coin_type::CoinType;
use tw_coin_registry::dispatcher::coin_dispatcher;
use tw_memory::Data;

/// Non-core transaction utility methods, like building a transaction using an external signature.
pub struct TransactionCompiler;

impl TransactionCompiler {
    /// Obtains pre-signing hashes of a transaction.
    #[inline]
    pub fn preimage_hashes(coin: CoinType, input: &[u8]) -> SigningResult<Data> {
        let (ctx, entry) = coin_dispatcher(coin)?;
        entry
            .preimage_hashes(&ctx, input)
            .map_err(SigningError::from)
    }

    /// Compiles a complete transaction with one or more external signatures.
    #[inline]
    pub fn compile(
        coin: CoinType,
        input: &[u8],
        signatures: Vec<SignatureBytes>,
        public_keys: Vec<PublicKeyBytes>,
    ) -> SigningResult<Data> {
        let (ctx, entry) = coin_dispatcher(coin)?;
        entry
            .compile(&ctx, input, signatures, public_keys)
            .map_err(SigningError::from)
    }
}
