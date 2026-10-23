// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CONTEXT_START, DEBIT_START, PUBLIC_VALUES, ShieldAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub struct PublicStatement {
    pub outputs: [[u8; 64]; 2],
    pub group: [u8; 64],
}

impl ShieldAir {
    pub fn public_values(&self, statement: &PublicStatement) -> [Value; PUBLIC_VALUES] {
        let mut values = [Value::ZERO; PUBLIC_VALUES];
        for index in 0..2 {
            values[index * 32..index * 32 + 32]
                .copy_from_slice(&digest_values(statement.outputs[index]));
        }
        for limb in 0..4 {
            values[DEBIT_START + limb] = Value::from_u16((self.debit >> (limb * 16)) as u16);
        }
        values[CONTEXT_START..].copy_from_slice(&digest_values(statement.group));
        values
    }
}
