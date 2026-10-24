// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, KEY_BYTES};
use sha3::{Digest, Sha3_256, Sha3_512};

pub(super) struct Inputs {
    pub key: [u8; KEY_BYTES],
    pub message: [u8; 64],
    pub digest: [u8; 32],
    pub output: [u8; 64],
}

impl Inputs {
    pub fn new(key: [u8; KEY_BYTES], randomness: [u8; 32]) -> Option<Self> {
        if (0..COEFFICIENTS).any(|i| coefficient(&key, i) >= 3329) {
            return None;
        }
        let digest: [u8; 32] = Sha3_256::digest(key).into();
        let mut message = [0; 64];
        message[..32].copy_from_slice(&randomness);
        message[32..].copy_from_slice(&digest);
        let output: [u8; 64] = Sha3_512::digest(message).into();
        Some(Self {
            key,
            message,
            digest,
            output,
        })
    }
}

pub(super) fn coefficient(key: &[u8; KEY_BYTES], index: usize) -> u16 {
    if index >= COEFFICIENTS {
        return 0;
    }
    let start = index / 2 * 3;
    let packed = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((packed >> (12 * (index % 2))) & 4095) as u16
}
