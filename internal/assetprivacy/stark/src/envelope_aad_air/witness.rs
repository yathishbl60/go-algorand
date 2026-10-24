// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use sha3::{Digest, Sha3_512};

pub(super) struct Inputs {
    pub context: [u8; 256],
    pub key: [u8; 1184],
    pub polynomials: [u16; 768],
    pub ciphertext: [u8; 1088],
    pub associated: [u8; 64],
}

impl Inputs {
    pub fn new(
        context: &[u8; 248],
        key: &[u8],
        epoch: u64,
        ciphertext: [u8; 1088],
    ) -> Option<Self> {
        let key: [u8; 1184] = key.try_into().ok()?;
        let polynomials = decode(&key)?;
        let context = expanded_context(context, epoch);
        let associated = digest(&context, &key, &ciphertext);
        Some(Self {
            context,
            key,
            polynomials,
            ciphertext,
            associated,
        })
    }
}

fn expanded_context(context: &[u8; 248], epoch: u64) -> [u8; 256] {
    let mut expanded = [0; 256];
    expanded[..147].copy_from_slice(&context[..147]);
    expanded[147..155].copy_from_slice(&epoch.to_be_bytes());
    expanded[155..].copy_from_slice(&context[147..]);
    expanded
}

fn digest(context: &[u8; 256], key: &[u8; 1184], ciphertext: &[u8; 1088]) -> [u8; 64] {
    let mut hash = Sha3_512::new();
    hash.update(context);
    hash.update(key);
    hash.update(ciphertext);
    hash.finalize().into()
}

fn coefficient(key: &[u8; 1184], index: usize) -> u16 {
    let start = index / 2 * 3;
    let packed = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((packed >> (12 * (index % 2))) & 4095) as u16
}

fn decode(key: &[u8; 1184]) -> Option<[u16; 768]> {
    let decoded = core::array::from_fn(|index| coefficient(key, index));
    decoded.iter().all(|&value| value < 3329).then_some(decoded)
}
