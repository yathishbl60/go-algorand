// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Value, aes256_air::CipherBlock};
use p3_field::{PrimeCharacteristicRing, PrimeField64};

pub(super) fn bits<const N: usize>(bytes: [u8; N]) -> [[Value; 8]; N] {
    bytes.map(|byte| core::array::from_fn(|bit| Value::from_bool(byte & (1 << bit) != 0)))
}

pub(super) fn block_output(block: &CipherBlock<Value>) -> [u8; 16] {
    block.state[14].map(byte)
}

pub(super) fn byte(bits: [Value; 8]) -> u8 {
    bits.into_iter().enumerate().fold(0, |value, (bit, flag)| {
        value | ((flag.as_canonical_u64() as u8) << bit)
    })
}

pub(super) fn hash_output(bits: [Value; 128]) -> [u8; 16] {
    core::array::from_fn(|byte| {
        (0..8).fold(0, |value, bit| {
            value | ((bits[byte * 8 + bit].as_canonical_u64() as u8) << (7 - bit))
        })
    })
}
