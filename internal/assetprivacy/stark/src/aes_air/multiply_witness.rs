// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GfMul, byte::bits, field::xtime};
use crate::Value;

impl GfMul<Value> {
    /// Generates private columns; calling this does not evaluate the constraints.
    pub fn witness(left: u8, right: u8) -> Self {
        let mut power = left;
        let powers = core::array::from_fn(|_| {
            power = xtime(power);
            bits(power)
        });
        let selected = select(left, right);
        let mut accumulator = 0;
        let sums = selected.map(|byte| {
            accumulator ^= byte;
            bits(accumulator)
        });
        Self {
            left: bits(left),
            right: bits(right),
            powers,
            selected: selected.map(bits),
            sums,
        }
    }
}

fn select(mut power: u8, right: u8) -> [u8; 8] {
    core::array::from_fn(|step| {
        let selected = power & 0u8.wrapping_sub((right >> step) & 1);
        power = xtime(power);
        selected
    })
}
