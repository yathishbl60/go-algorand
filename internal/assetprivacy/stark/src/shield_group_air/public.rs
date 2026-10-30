// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PUBLIC_VALUES, ShieldGroupAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub struct PublicStatement {
    pub outputs: [[u8; 64]; 2],
    pub group: [u8; 64],
}

impl PublicStatement {
    pub(super) fn validate(&self) -> Option<()> {
        (self.group != [0; 64]
            && !self.outputs.contains(&[0; 64])
            && self.outputs[0] != self.outputs[1])
            .then_some(())
    }
}

impl ShieldGroupAir {
    pub fn public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        statement.validate()?;
        let mut values = Vec::with_capacity(PUBLIC_VALUES);
        for commitment in statement.outputs {
            values.extend(digest_values(commitment));
        }
        for limb in 0..4 {
            values.push(Value::from_u16((self.debit >> (limb * 16)) as u16));
        }
        values.extend(digest_values(statement.group));
        Some(values)
    }
}
