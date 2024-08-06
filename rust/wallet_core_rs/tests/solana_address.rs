// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use tw_memory::test_utils::tw_string_helper::TWStringHelper;
use wallet_core_rs::ffi::solana::address::tw_solana_address_default_token_address;

#[test]
fn test_solana_address_default_token_address() {
    let main_address = "B1iGmDJdvmxyUiYM8UEo2Uw2D58EmUrw4KyLYMmrhf8V";
    let main_address = TWStringHelper::create(main_address);

    let token_mint_address = "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt";
    let token_mint_address = TWStringHelper::create(token_mint_address);

    let actual = unsafe {
        TWStringHelper::wrap(tw_solana_address_default_token_address(
            main_address.ptr(),
            token_mint_address.ptr(),
            false
        ))
    }
    .to_string()
    .expect("!tw_solana_address_default_token_address returned a nullptr");

    assert_eq!(actual, "EDNd1ycsydWYwVmrYZvqYazFqwk1QjBgAUKFjBoz1jKP");
}

#[test]
fn test_solana_address_default_token_2022_address() {
    let main_address = "5RMMQUsbkeHE6PsNjBZ3Cnoo1yehgwSnJjp1abGroDkL";
    let main_address = TWStringHelper::create(main_address);

    let token_mint_address = "BSQCmMAFB9itonyVSLsUxX92Ne1rgBZFqothBk3q91k6";
    let token_mint_address = TWStringHelper::create(token_mint_address);

    let actual = unsafe {
        TWStringHelper::wrap(tw_solana_address_default_token_address(
            main_address.ptr(),
            token_mint_address.ptr(),
            true
        ))
    }
    .to_string()
    .expect("!tw_solana_address_default_token_address returned a nullptr");

    assert_eq!(actual, "8ZZWsB6HjfeRyoTrobToJh5NA6baKYy9wxwoKQEjYLaM");
}
