// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#![allow(clippy::missing_safety_doc)]

use crate::any_address::AnyAddress;
use tw_coin_entry::derivation::Derivation;
use tw_keypair::ffi::pubkey::TWPublicKey;
use tw_memory::ffi::tw_data::TWData;
use tw_memory::ffi::tw_string::TWString;
use tw_memory::ffi::RawPtrTrait;
use tw_misc::{try_or_else, try_or_false};

/// Represents an address in Rust for almost any blockchain.
pub struct TWAnyAddress(AnyAddress);

impl RawPtrTrait for TWAnyAddress {}

/// Determines if the string is a valid Any address.
///
/// \param string address to validate.
/// \param coin coin type of the address.
/// \return bool indicating if the address is valid.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_is_valid(string: *const TWString, coin: u32) -> bool {
    let string = try_or_false!(TWString::from_ptr_as_ref(string));
    let string = try_or_false!(string.as_str());

    AnyAddress::is_valid(coin, string, None)
}

/// Creates an address from a string representation and a coin type. Must be deleted with `TWAnyAddressDelete` after use.
///
/// \param string address to create.
/// \param coin coin type of the address.
/// \return `TWAnyAddress` pointer or nullptr if address and coin are invalid.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_create_with_string(
    string: *const TWString,
    coin: u32,
) -> *mut TWAnyAddress {
    let string = try_or_else!(TWString::from_ptr_as_ref(string), std::ptr::null_mut);
    let string = try_or_else!(string.as_str(), std::ptr::null_mut);

    AnyAddress::with_string(coin, string, None)
        .map(|any_address| TWAnyAddress(any_address).into_ptr())
        .unwrap_or_else(|_| std::ptr::null_mut())
}

/// Creates an address from a public key and derivation option.
///
/// \param public_key derivates the address from the public key.
/// \param coin coin type of the address.
/// \param derivation the custom derivation to use.
/// \return `TWAnyAddress` pointer or nullptr if public key is invalid.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_create_with_public_key_derivation(
    public_key: *mut TWPublicKey,
    coin: u32,
    derivation: u32,
) -> *mut TWAnyAddress {
    let public_key = try_or_else!(TWPublicKey::from_ptr_as_ref(public_key), std::ptr::null_mut);
    let derivation = try_or_else!(Derivation::from_raw(derivation), std::ptr::null_mut);

    AnyAddress::with_public_key(coin, public_key.as_ref().clone(), derivation, None)
        .map(|any_address| TWAnyAddress(any_address).into_ptr())
        .unwrap_or_else(|_| std::ptr::null_mut())
}

/// Deletes an address.
///
/// \param address address to delete.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_delete(address: *mut TWAnyAddress) {
    // Take the ownership back to rust and drop the owner.
    let _ = TWAnyAddress::from_ptr(address);
}

/// Returns the address string representation.
///
/// \param address address to get the string representation of.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_description(address: *const TWAnyAddress) -> *mut TWString {
    // Take the ownership back to rust and drop the owner.
    let address = try_or_else!(TWAnyAddress::from_ptr_as_ref(address), std::ptr::null_mut);

    let description = address.0.description().to_string();
    TWString::from(description).into_ptr()
}

/// Returns underlying data (public key or key hash)
///
/// \param address address to get the data of.
#[no_mangle]
pub unsafe extern "C" fn tw_any_address_data(address: *const TWAnyAddress) -> *mut TWData {
    // Take the ownership back to rust and drop the owner.
    let address = try_or_else!(TWAnyAddress::from_ptr_as_ref(address), std::ptr::null_mut);

    let data = try_or_else!(address.0.get_data(), std::ptr::null_mut);
    TWData::from(data).into_ptr()
}
