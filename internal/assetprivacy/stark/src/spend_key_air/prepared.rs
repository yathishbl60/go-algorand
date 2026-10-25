// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE_BYTES, SpendKeyAir};
use crate::Value;
use p3_field::{Field, PrimeCharacteristicRing};
use sha3::{Digest, Sha3_512};

pub(super) type Input = ([u8; 64], [u8; 32], u64, u64);

pub(super) struct Prepared {
    pub message: [u8; MESSAGE_BYTES],
    pub commitment: [u8; 64],
    pub inverses: [Value; 3],
}

impl Prepared {
    pub fn new(air: &SpendKeyAir, input: Input) -> Option<Self> {
        let message = air.message(input.0, input.1, input.2, input.3)?;
        let commitment = Sha3_512::digest(message).into();
        let inverses = [
            inverse(&input.1),
            inverse(&input.2.to_be_bytes()),
            inverse(&input.0),
        ];
        Some(Self {
            message,
            commitment,
            inverses,
        })
    }
}

fn inverse(bytes: &[u8]) -> Value {
    let sum: u32 = bytes.iter().map(|value| value.count_ones()).sum();
    Value::from_u32(sum).inverse()
}
