// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EligibleRecordAir, MESSAGE_BYTES};
use crate::Value;
use p3_field::{Field, PrimeCharacteristicRing};
use sha3::{Digest, Sha3_512};

pub(super) struct Prepared {
    pub message: [u8; MESSAGE_BYTES],
    pub commitment: [u8; 64],
    pub inverses: [Value; 2],
}

impl Prepared {
    pub fn new(
        air: &EligibleRecordAir,
        account: [u8; 32],
        revision: u64,
        updated: u64,
    ) -> Option<Self> {
        let message = air.message(account, revision, updated)?;
        Some(Self {
            commitment: Sha3_512::digest(message).into(),
            message,
            inverses: [inverse(&account), inverse(&revision.to_be_bytes())],
        })
    }
}

fn inverse(bytes: &[u8]) -> Value {
    let sum: u32 = bytes.iter().map(|value| value.count_ones()).sum();
    Value::from_u32(sum).inverse()
}
