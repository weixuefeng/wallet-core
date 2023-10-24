// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use serde::Deserialize;
use tw_hash::{H256, H520};
use tw_keypair::ecdsa::secp256k1::{KeyPair, VerifySignature};
use tw_keypair::traits::{SigningKeyTrait, VerifyingKeyTrait};

/// The tests were generated in C++ using the `trezor-crypto` library.
const SECP256K1_SIGN: &str = include_str!("secp256k1_sign.json");

#[derive(Deserialize)]
struct Secp256k1SignTest {
    secret: H256,
    hash: H256,
    signature: H520,
}

#[test]
fn test_secp256k1_sign_verify() {
    let tests: Vec<Secp256k1SignTest> = serde_json::from_str(SECP256K1_SIGN).unwrap();
    for test in tests {
        let keypair = KeyPair::try_from(test.secret.as_slice()).unwrap();
        let actual = keypair.sign(test.hash).unwrap();
        assert_eq!(actual.to_bytes(), test.signature);

        let verify_sign = VerifySignature::from(actual);
        assert!(keypair.verify(verify_sign, test.hash));
    }
}
