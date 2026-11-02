// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Opening, Oracle, V};
use p3_commit::Mmcs;
use p3_field::PrimeCharacteristicRing;

pub fn reject(oracle: &Oracle, index: usize, valid: &Opening) {
    for case in 0..6 {
        let mut changed = valid.clone();
        match case {
            0 => changed.opened_values[0][0] += V::ONE,
            1 => changed.opening_proof.0[0][0] += V::ONE,
            2 => changed.opening_proof.1[0][0] ^= 1,
            3 => {
                changed.opening_proof.1.pop();
            }
            4 => {
                changed.opening_proof.0.pop();
            }
            _ => changed.opened_values[0].push(V::ONE),
        }
        assert!(!accepts(oracle, index, &changed));
    }
    assert!(!accepts(oracle, index ^ 1, valid));
}

fn accepts(oracle: &Oracle, index: usize, opening: &Opening) -> bool {
    oracle
        .mmcs
        .verify_batch(&oracle.cap, &oracle.dimensions, index, opening.into())
        .is_ok()
}
