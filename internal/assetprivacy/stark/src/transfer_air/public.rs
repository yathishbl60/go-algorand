// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CONTEXT_START, CONVERSION_START, PUBLIC_VALUES, TransferAir};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

/// The ledger supplies accepted roots and checks both nullifiers are distinct/fresh.
pub struct InputPublic {
    pub nullifier: [u8; 64],
    pub root: [u8; 64],
}

/// Finalized body context must be recomputed by the native verifier, not trusted.
pub struct PublicStatement {
    pub inputs: [InputPublic; 2],
    pub outputs: [[u8; 64]; 2],
    pub group: [u8; 64],
}

impl TransferAir {
    /// Canonical public vector; its complete contents enter the pinned P3 transcript.
    pub fn public_values(&self, statement: &PublicStatement) -> [Value; PUBLIC_VALUES] {
        let mut values = [Value::ZERO; PUBLIC_VALUES];
        for (index, digest) in statement.hashes().into_iter().enumerate() {
            values[index * 32..index * 32 + 32].copy_from_slice(&digest_values(digest));
        }
        values[CONVERSION_START..CONTEXT_START].copy_from_slice(&self.conversion_values());
        values[CONTEXT_START..].copy_from_slice(&digest_values(statement.group));
        values
    }

    fn conversion_values(&self) -> [Value; 8] {
        let amounts = [self.conversion.debit, self.conversion.credit];
        core::array::from_fn(|i| Value::from_u16((amounts[i / 4] >> (i % 4 * 16)) as u16))
    }
}

impl PublicStatement {
    fn hashes(&self) -> [[u8; 64]; 6] {
        [
            self.inputs[0].nullifier,
            self.inputs[0].root,
            self.inputs[1].nullifier,
            self.inputs[1].root,
            self.outputs[0],
            self.outputs[1],
        ]
    }
}
