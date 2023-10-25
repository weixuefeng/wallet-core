// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::abi::{AbiError, AbiErrorKind, AbiResult};
use std::fmt;
use tw_number::U256;

#[derive(Clone, Copy, PartialEq)]
pub struct UintBits(usize);

impl Default for UintBits {
    fn default() -> Self {
        UintBits::new(U256::BITS).expect("U256::BITS must be a valid number of bits")
    }
}

impl UintBits {
    pub fn new(bits: usize) -> AbiResult<UintBits> {
        check_uint_bits(bits)?;
        Ok(UintBits(bits))
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl fmt::Display for UintBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for UintBits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<UintBits> for usize {
    fn from(value: UintBits) -> Self {
        value.0
    }
}

// https://docs.soliditylang.org/en/latest/abi-spec.html#types
pub fn check_uint_bits(bits: usize) -> AbiResult<()> {
    if bits % 8 != 0 || bits == 0 || bits > 256 {
        return Err(AbiError(AbiErrorKind::Error_invalid_uint_value));
    }
    Ok(())
}
