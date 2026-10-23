// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Aes256, ExpandedKey, byte::bits, witness_round::Rounds};
use crate::Value;

impl Aes256<Value> {
    /// Generates private columns only; proof evaluation is a separate operation.
    pub fn witness(key: [u8; 32], plaintext: [u8; 16]) -> Self {
        let key = ExpandedKey::witness(key);
        let plaintext = plaintext.map(bits);
        let mut rounds = Rounds::new(&key, plaintext);
        let substitutions = core::array::from_fn(|r| rounds.step(&key, r));
        let (state, mix) = rounds.finish();
        Self {
            key,
            plaintext,
            substitutions,
            state,
            mix,
        }
    }
}
