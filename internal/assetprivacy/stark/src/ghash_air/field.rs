// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

// Host witness arithmetic only; the AIR independently enforces every operation.
pub(super) fn shift(value: u128) -> u128 {
    (value >> 1) ^ ((0xe1u128 << 120) & 0u128.wrapping_sub(value & 1))
}

pub(super) fn multiply(left: u128, mut right: u128) -> u128 {
    let mut output = 0;
    for step in 0..128 {
        output ^= right & 0u128.wrapping_sub((left >> (127 - step)) & 1);
        right = shift(right);
    }
    output
}
