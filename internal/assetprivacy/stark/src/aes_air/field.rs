// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

// Host witness arithmetic only. Algebraic constraints independently enforce it.
pub(super) fn xtime(value: u8) -> u8 {
    (value << 1) ^ (0x1b & 0u8.wrapping_sub(value >> 7))
}

pub(super) fn multiply(mut left: u8, mut right: u8) -> u8 {
    let mut result = 0;
    for _ in 0..8 {
        result ^= left & 0u8.wrapping_sub(right & 1);
        left = xtime(left);
        right >>= 1;
    }
    result
}

pub(super) fn inverse(value: u8) -> u8 {
    (0..8).rev().fold(1, |accumulator, bit| {
        let square = multiply(accumulator, accumulator);
        if bit == 0 {
            square
        } else {
            multiply(square, value)
        }
    })
}
