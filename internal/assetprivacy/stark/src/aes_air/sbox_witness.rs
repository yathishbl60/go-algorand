// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GfMul, SBox, byte::bits, field::inverse};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

impl SBox<Value> {
    /// Generates private intermediate bits, independently checked by eval_sbox.
    pub fn witness(input: u8) -> Self {
        let inverse = inverse(input);
        let mut accumulator = inverse;
        let affine = core::array::from_fn(|step| {
            accumulator ^= inverse.rotate_left((4 - step) as u32);
            bits(accumulator)
        });
        Self {
            inversion: GfMul::witness(input, inverse),
            is_zero: Value::from_bool(input == 0),
            affine,
            output: bits(accumulator ^ 0x63),
        }
    }
}
