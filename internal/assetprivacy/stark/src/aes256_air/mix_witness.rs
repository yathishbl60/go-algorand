// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    MixColumn,
    byte::{bits, integer},
};
use crate::{Value, aes_air::Byte};

impl MixColumn<Value> {
    pub fn witness(input: [Byte<Value>; 4]) -> Self {
        let input = input.map(integer);
        let t0 = input[0] ^ input[1];
        let t1 = t0 ^ input[2];
        let t2 = t1 ^ input[3];
        let difference = core::array::from_fn(|r| input[r] ^ input[(r + 1) % 4]);
        let doubled = difference.map(|b: u8| (b << 1) ^ (0x1b & 0u8.wrapping_sub(b >> 7)));
        let partial = input.map(|b| b ^ t2);
        Self {
            total: [t0, t1, t2].map(bits),
            difference: difference.map(bits),
            doubled: doubled.map(bits),
            partial: partial.map(bits),
            output: core::array::from_fn(|r| bits(partial[r] ^ doubled[r])),
        }
    }
}
