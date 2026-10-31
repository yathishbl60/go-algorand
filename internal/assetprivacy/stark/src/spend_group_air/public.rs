// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Operation, SpendGroupAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

#[derive(Clone, Copy)]
pub struct InputPublic {
    pub nullifier: [u8; 64],
    pub root: [u8; 64],
}

/// Expected fields must come from the finalized statement and current native state.
pub struct PublicStatement {
    pub inputs: [InputPublic; 2],
    pub outputs: Vec<[u8; 64]>,
    pub group: [u8; 64],
    pub current_eligibility_root: [u8; 64],
}

impl PublicStatement {
    pub(super) fn validate(&self, operation: Operation) -> Option<()> {
        if self.outputs.len() != operation.output_count()
            || self.inputs[0].nullifier == self.inputs[1].nullifier
        {
            return None;
        }
        if self.group == [0; 64]
            || self.current_eligibility_root == [0; 64]
            || self.inputs.iter().any(|input| input.root == [0; 64])
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

impl SpendGroupAir {
    /// Complete public vector enters the pinned transcript, including group digest.
    pub fn public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        let mut values = self.base_public_values(statement)?;
        if let Some(participants) = &self.participants {
            crate::public_participants::append(&mut values, participants);
        }
        Some(values)
    }

    fn base_public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        statement.validate(self.operation)?;
        let mut values = Vec::with_capacity(self.public_count());
        for input in &statement.inputs {
            values.extend(digest_values(input.nullifier));
            values.extend(digest_values(input.root));
        }
        for &commitment in &statement.outputs {
            values.extend(digest_values(commitment));
        }
        values.extend(
            self.operation
                .conversion_values()
                .into_iter()
                .map(Value::from_u16),
        );
        values.extend(digest_values(statement.group));
        values.extend(digest_values(statement.current_eligibility_root));
        Some(values)
    }
}
