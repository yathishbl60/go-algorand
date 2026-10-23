// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Gf128, block::bits, field::shift};
use crate::Value;

impl Gf128<Value> {
    pub fn witness(left: [u8; 16], right: [u8; 16]) -> Self {
        Self::from_integers(u128::from_be_bytes(left), u128::from_be_bytes(right))
    }

    pub(super) fn from_integers(left: u128, right: u128) -> Self {
        let mut power = right;
        let powers = core::array::from_fn(|_| {
            power = shift(power);
            bits(power)
        });
        let selected = selections(left, right);
        let mut sum = 0;
        let sums = selected.map(|block| {
            sum ^= block;
            bits(sum)
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

fn selections(left: u128, mut power: u128) -> [u128; 128] {
    core::array::from_fn(|step| {
        let selected = power & 0u128.wrapping_sub((left >> (127 - step)) & 1);
        power = shift(power);
        selected
    })
}
