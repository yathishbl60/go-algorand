// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Aes256, CipherBlock, ExpandedKey, byte::bits, witness_round::Rounds};
use crate::Value;

impl Aes256<Value> {
    /// Generates private columns only; proof evaluation is a separate operation.
    pub fn witness(key: [u8; 32], plaintext: [u8; 16]) -> Self {
        let key = ExpandedKey::witness(key);
        let block = CipherBlock::witness(&key, plaintext);
        Self { key, block }
    }
}

impl CipherBlock<Value> {
    /// Uses exactly the shared expanded key; adds no second key schedule.
    pub fn witness(key: &ExpandedKey<Value>, plaintext: [u8; 16]) -> Self {
        let plaintext = plaintext.map(bits);
        let mut rounds = Rounds::new(key, plaintext);
        let substitutions = core::array::from_fn(|r| rounds.step(key, r));
        let (state, mix) = rounds.finish();
        Self {
            plaintext,
            substitutions,
            state,
            mix,
        }
    }
}
