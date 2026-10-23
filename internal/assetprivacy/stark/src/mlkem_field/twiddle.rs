// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Q;

/// Canonical 17^BitRev7(index) for trusted forward/inverse NTT schedules.
pub const fn ntt_twiddle(index: u8) -> Option<u16> {
    if index >= 128 {
        return None;
    }
    Some(root(index.reverse_bits() >> 1))
}

/// Canonical 17^(2*BitRev7(index)+1) for an NTT-domain quadratic factor.
pub const fn ntt_gamma(index: u8) -> Option<u16> {
    if index >= 128 {
        return None;
    }
    Some(root((index.reverse_bits() >> 1) * 2 + 1))
}

// FIPS 203 twiddles are 17^BitRev7(k) mod 3329, without Montgomery factors.
const fn root(mut exponent: u8) -> u16 {
    let mut value = 1u32;
    let mut power = 17u32;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = value * power % Q as u32;
        }
        power = power * power % Q as u32;
        exponent >>= 1;
    }
    value as u16
}
