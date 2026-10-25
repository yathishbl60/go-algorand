// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE_BYTES, ViewKeyAir};
use crate::Value;
use p3_field::{Field, PrimeCharacteristicRing};
use sha3::{Digest, Sha3_512};

pub(super) type Input<'a> = (&'a [u8], [u8; 32], u64, u64);

pub(super) struct Prepared {
    pub message: [u8; MESSAGE_BYTES],
    pub coefficients: [u16; 768],
    pub commitment: [u8; 64],
    pub inverses: [Value; 2],
}

impl Prepared {
    pub fn new(air: &ViewKeyAir, input: Input<'_>) -> Option<Self> {
        let message = air.message(input.0, input.1, input.2, input.3)?;
        let coefficients = coefficients(input.0)?;
        let commitment = Sha3_512::digest(message).into();
        let inverses = [inverse(&input.1), inverse(&input.2.to_be_bytes())];
        Some(Self {
            message,
            coefficients,
            commitment,
            inverses,
        })
    }
}

fn inverse(bytes: &[u8]) -> Value {
    let sum: u32 = bytes.iter().map(|value| value.count_ones()).sum();
    Value::from_u32(sum).inverse()
}

fn coefficients(key: &[u8]) -> Option<[u16; 768]> {
    let coefficients = core::array::from_fn(|index| coefficient(key, index));
    coefficients
        .iter()
        .all(|&value| value < 3329)
        .then_some(coefficients)
}

fn coefficient(key: &[u8], index: usize) -> u16 {
    let start = index / 2 * 3;
    let value = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((value >> (12 * (index % 2))) & 4095) as u16
}
