// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CONTEXT_START, CREDIT_START, InputPublic, PUBLIC_VALUES, UnshieldAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub struct PublicStatement {
    pub inputs: [InputPublic; 2],
    pub group: [u8; 64],
}

impl UnshieldAir {
    pub fn public_values(&self, statement: &PublicStatement) -> [Value; PUBLIC_VALUES] {
        let mut values = [Value::ZERO; PUBLIC_VALUES];
        for (index, input) in statement.inputs.iter().enumerate() {
            let start = index * 64;
            values[start..start + 32].copy_from_slice(&digest_values(input.nullifier));
            values[start + 32..start + 64].copy_from_slice(&digest_values(input.root));
        }
        for limb in 0..4 {
            values[CREDIT_START + limb] = Value::from_u16((self.credit >> (limb * 16)) as u16);
        }
        values[CONTEXT_START..].copy_from_slice(&digest_values(statement.group));
        values
    }
}
