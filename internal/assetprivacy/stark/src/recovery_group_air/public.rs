// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Operation, RecoveryGroupAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

#[derive(Clone, Copy)]
pub struct InputPublic {
    pub nullifier: [u8; 64],
    pub root: [u8; 64],
}

/// All expected fields come from finalized native bodies and accepted-root state.
pub struct PublicStatement {
    pub input: InputPublic,
    pub outputs: Vec<[u8; 64]>,
    pub group: [u8; 64],
}

impl PublicStatement {
    pub(super) fn validate(&self, operation: Operation) -> Option<()> {
        if self.outputs.len() != operation.output_count()
            || self.input.root == [0; 64]
            || self.group == [0; 64]
        {
            return None;
        }
        if self.outputs.contains(&[0; 64])
            || (self.outputs.len() == 2 && self.outputs[0] == self.outputs[1])
        {
            return None;
        }
        Some(())
    }
}

impl RecoveryGroupAir {
    /// Complete public values, including group/authority, enter the pinned transcript.
    pub fn public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        statement.validate(self.operation)?;
        let mut values = Vec::with_capacity(self.operation.public_count());
        values.extend(digest_values(statement.input.nullifier));
        values.extend(digest_values(statement.input.root));
        for &commitment in &statement.outputs {
            values.extend(digest_values(commitment));
        }
        values.extend(
            self.operation
                .credit_values()
                .into_iter()
                .map(Value::from_u16),
        );
        values.extend(digest_values(statement.group));
        values.extend(self.authority.into_iter().map(Value::from_u8));
        Some(values)
    }
}
