// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use sha3::{Digest, Sha3_256, Sha3_512};

pub(super) struct Inputs {
    pub key: [u8; 1184],
    pub polynomials: [u16; 768],
    pub message: [u8; 64],
    pub output: [u8; 64],
}

impl Inputs {
    pub fn new(key: &[u8], message: [u8; 32]) -> Option<Self> {
        let key: [u8; 1184] = key.try_into().ok()?;
        let polynomials = core::array::from_fn(|index| coefficient(&key, index));
        if polynomials.iter().any(|&coefficient| coefficient >= 3329) {
            return None;
        }
        let digest: [u8; 32] = Sha3_256::digest(key).into();
        let mut expanded = [0; 64];
        expanded[..32].copy_from_slice(&message);
        expanded[32..].copy_from_slice(&digest);
        let output = Sha3_512::digest(expanded).into();
        Some(Self {
            key,
            polynomials,
            message: expanded,
            output,
        })
    }
}

fn coefficient(key: &[u8; 1184], index: usize) -> u16 {
    let start = index / 2 * 3;
    let packed = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((packed >> (12 * (index % 2))) & 4095) as u16
}
