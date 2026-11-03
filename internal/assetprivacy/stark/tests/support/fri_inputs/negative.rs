// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Argument, E, Fixture, V, verification};
use p3_field::PrimeCharacteristicRing;

pub fn reject(fixture: &Fixture, argument: Argument) {
    assert!(!verification::accepts(fixture, &argument, 18));
    let mut leaf = Argument {
        proof: argument.proof.clone(),
        values: argument.values.clone(),
    };
    leaf.proof.input_openings[0].opened_values[0][0][0] += V::ONE;
    assert_eq!(verification::outcome(fixture, &leaf, 17), (false, false));
    let mut fold = Argument {
        proof: argument.proof.clone(),
        values: argument.values.clone(),
    };
    fold.proof.commit_phase_openings[0].sibling_values[0][0] += E::ONE;
    assert_eq!(verification::outcome(fixture, &fold, 17), (true, false));
    let mut claim = argument;
    claim.values[0] += E::ONE;
    assert!(!verification::accepts(fixture, &claim, 17));
}
